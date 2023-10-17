use crate::assets::table::sound::{JingleBind, SfxBind};

use super::Table;

impl Table {
    pub fn play_sfx_bind(&self, bind: SfxBind) {
        self.play_sfx_bind_volume(bind, 0x40)
    }

    pub fn play_sfx_bind_volume(&self, bind: SfxBind, volume: u8) {
        if let Some(sfx) = self.assets.sfx_binds[bind] {
            self.player.play_sfx(sfx, volume);
        }
    }

    pub fn play_jingle_bind(&self, bind: JingleBind) -> bool {
        let jingle = self.assets.jingle_binds[bind].unwrap();
        self.sequencer.play_jingle(jingle, false, None)
    }

    pub fn play_jingle_bind_force(&self, bind: JingleBind) -> bool {
        let jingle = self.assets.jingle_binds[bind].unwrap();
        self.sequencer.play_jingle(jingle, true, None)
    }

    pub fn play_jingle_bind_silence(&self, bind: JingleBind) -> bool {
        let jingle = self.assets.jingle_binds[bind].unwrap();
        let silence = self.assets.jingle_binds[JingleBind::Silence].unwrap();
        self.sequencer
            .play_jingle(jingle, false, Some(silence.position))
    }

    pub fn set_music_silence(&self) {
        self.sequencer.set_music(
            self.assets.jingle_binds[JingleBind::Silence]
                .unwrap()
                .position,
        );
    }

    pub fn set_music_plunger(&self) {
        let jingle = if self.options.no_music {
            JingleBind::Silence
        } else {
            JingleBind::Plunger
        };
        self.sequencer
            .set_music(self.assets.jingle_binds[jingle].unwrap().position);
    }

    pub fn set_music_main(&self) {
        let jingle = if self.options.no_music {
            JingleBind::Silence
        } else {
            JingleBind::Main
        };
        self.sequencer
            .set_music(self.assets.jingle_binds[jingle].unwrap().position);
    }

    pub fn play_jingle_plunger(&self) {
        let jingle = self.assets.jingle_binds[if self.options.no_music {
            JingleBind::Silence
        } else {
            JingleBind::Plunger
        }]
        .unwrap();
        self.sequencer
            .play_jingle(jingle, false, Some(jingle.position));
    }
}
