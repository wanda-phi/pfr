use std::path::PathBuf;

use crate::bcd::Bcd;
use arrayref::array_ref;
use enum_map::{enum_map, Enum, EnumMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Config {
    pub options: Options,
    pub high_scores: EnumMap<TableId, [HighScore; 4]>,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Options {
    pub balls: u8,
    pub angle_high: bool,
    pub scroll_speed: ScrollSpeed,
    pub resolution: Resolution,
    pub no_music: bool,
    pub mono: bool,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct HighScore {
    pub score: Bcd,
    pub name: [u8; 3],
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Angle {
    Low,
    High,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum ScrollSpeed {
    Hard,
    Medium,
    Soft,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum Resolution {
    Normal,
    High,
    Full,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Enum, Debug)]
pub enum TableId {
    Table1,
    Table2,
    Table3,
    Table4,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            balls: 3,
            angle_high: true,
            scroll_speed: ScrollSpeed::Medium,
            resolution: Resolution::Normal,
            no_music: false,
            mono: false,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            options: Default::default(),
            high_scores: enum_map! {
                TableId::Table1 => [
                    HighScore { name: *b"TSP", score: Bcd::from_ascii(b"50000000") },
                    HighScore { name: *b"ICE", score: Bcd::from_ascii(b"25000000") },
                    HighScore { name: *b"ANY", score: Bcd::from_ascii(b"10000000") },
                    HighScore { name: *b"J L", score: Bcd::from_ascii(b"5000000") },
                ],
                TableId::Table2 => [
                    HighScore { name: *b"TSP", score: Bcd::from_ascii(b"100000000") },
                    HighScore { name: *b"J L", score: Bcd::from_ascii(b"50000000") },
                    HighScore { name: *b"ICE", score: Bcd::from_ascii(b"25000000") },
                    HighScore { name: *b"ANY", score: Bcd::from_ascii(b"10000000") },
                ],
                TableId::Table3 => [
                    HighScore { name: *b"TSP", score: Bcd::from_ascii(b"50000000") },
                    HighScore { name: *b"ANY", score: Bcd::from_ascii(b"25000000") },
                    HighScore { name: *b"J L", score: Bcd::from_ascii(b"10000000") },
                    HighScore { name: *b"ICE", score: Bcd::from_ascii(b"5000000") },

                ],
                TableId::Table4 => [
                    HighScore { name: *b"TSP", score: Bcd::from_ascii(b"100000000") },
                    HighScore { name: *b"ICE", score: Bcd::from_ascii(b"50000000") },
                    HighScore { name: *b"ANY", score: Bcd::from_ascii(b"25000000") },
                    HighScore { name: *b"J L", score: Bcd::from_ascii(b"10000000") },
                ],
            },
        }
    }
}

impl Config {
    pub fn load(store: &impl ConfigStore) -> Config {
        let mut res = Config::default();
        if let Some(cfg) = store.load("PINBALL.CFG") {
            if cfg.len() == 6 {
                res.options.balls = match cfg[0] {
                    1 => 5,
                    _ => 3,
                };
                res.options.angle_high = cfg[1] != 1;
                res.options.scroll_speed = match cfg[2] {
                    0 => ScrollSpeed::Hard,
                    2 => ScrollSpeed::Soft,
                    _ => ScrollSpeed::Medium,
                };
                res.options.no_music = cfg[3] == 1;
                res.options.resolution = match cfg[4] {
                    1 => Resolution::High,
                    2 => Resolution::Full,
                    _ => Resolution::Normal,
                };
                res.options.mono = cfg[5] == 1;
            }
        }
        for (table, file) in [
            (TableId::Table1, "TABLE1.HI"),
            (TableId::Table2, "TABLE2.HI"),
            (TableId::Table3, "TABLE3.HI"),
            (TableId::Table4, "TABLE4.HI"),
        ] {
            if let Some(hi) = store.load(file) {
                if hi.len() == 0x40 {
                    for i in 0..4 {
                        let pos = i * 0x10;
                        let entry = &hi[pos..pos + 0x10];
                        res.high_scores[table][i].score =
                            Bcd::from_bytes(*array_ref![entry, 0, 12]);
                        res.high_scores[table][i].name = *array_ref![entry, 12, 3];
                    }
                }
            }
        }
        res
    }
}

impl Options {
    pub fn save(&self, store: &impl ConfigStore) {
        let raw: [u8; 6] = [
            if self.balls == 5 { 1 } else { 0 },
            if self.angle_high { 0 } else { 1 },
            match self.scroll_speed {
                ScrollSpeed::Hard => 0,
                ScrollSpeed::Medium => 1,
                ScrollSpeed::Soft => 2,
            },
            u8::from(self.no_music),
            match self.resolution {
                Resolution::Normal => 0,
                Resolution::High => 1,
                Resolution::Full => 2,
            },
            u8::from(self.mono),
        ];
        store.save("PINBALL.CFG", &raw);
    }
}

pub fn save_high_scores(table: TableId, scores: [HighScore; 4], store: &impl ConfigStore) {
    let file = match table {
        TableId::Table1 => "TABLE1.HI",
        TableId::Table2 => "TABLE2.HI",
        TableId::Table3 => "TABLE3.HI",
        TableId::Table4 => "TABLE4.HI",
    };
    let mut raw = vec![];
    for score in scores {
        raw.extend(score.score.digits);
        raw.extend(score.name);
        raw.push(0);
    }
    store.save(file, &raw);
}

pub trait ConfigStore {
    fn load(&self, fname: &str) -> Option<Vec<u8>>;
    fn save(&self, fname: &str, data: &[u8]);
}

pub struct FileConfigStore {
    pub path: PathBuf,
}

impl FileConfigStore {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl ConfigStore for FileConfigStore {
    fn load(&self, fname: &str) -> Option<Vec<u8>> {
        std::fs::read(self.path.join(fname)).ok()
    }

    fn save(&self, fname: &str, data: &[u8]) {
        let _ = std::fs::write(self.path.join(fname), data);
    }
}
