use crate::{assets::table::lights::LightBind, bcd::Bcd, config::TableId};

use super::{show::PrizeState, Table};

#[derive(Clone, Copy, Debug)]
pub struct PlayerState {
    pub score_main: Bcd,
    pub score_bonus: Bcd,
    pub num_cyclone: u16,
    pub bcd_num_cyclone: Bcd,
    pub table: TablePlayerState,
}

#[derive(Clone, Copy, Debug)]
pub enum TablePlayerState {
    Party(PartyPlayerState),
    Speed(SpeedPlayerState),
    Show(ShowPlayerState),
    Stones(StonesPlayerState),
}

impl PlayerState {
    pub fn new(table: TableId) -> Self {
        Self {
            score_main: Bcd::ZERO,
            score_bonus: Bcd::ZERO,
            num_cyclone: 0,
            bcd_num_cyclone: Bcd::ZERO,
            table: match table {
                TableId::Table1 => TablePlayerState::Party(PartyPlayerState {
                    light_puke: [false; 4],
                    light_mad: [false; 3],
                    light_crazy: [false; 5],
                    light_party: [false; 5],
                    score_tunnel_skill_shot: Bcd::ZERO,
                    score_cyclone_skill_shot: Bcd::ZERO,
                }),
                TableId::Table2 => TablePlayerState::Speed(SpeedPlayerState {
                    cur_gear: 0,
                    cur_speed: 0,
                    cur_place: 0,
                    max_place: 0,
                    car_mods: 0,
                    light_goal: false,
                    light_car_lit: [false; 5],
                    light_car: [false; 5],
                }),
                TableId::Table3 => TablePlayerState::Show(ShowPlayerState { prize_sets: 0 }),
                TableId::Table4 => TablePlayerState::Stones(StonesPlayerState {
                    cur_ghost: 0,
                    ghost_active: false,
                    score_skill_shot: Bcd::ZERO,
                    kickback: false,
                    light_rip: [false; 3],
                    light_stone: [false; 5],
                    light_bone: [false; 4],
                }),
            },
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PartyPlayerState {
    pub light_puke: [bool; 4],
    pub light_mad: [bool; 3],
    pub light_crazy: [bool; 5],
    pub light_party: [bool; 5],
    pub score_tunnel_skill_shot: Bcd,
    pub score_cyclone_skill_shot: Bcd,
}

#[derive(Clone, Copy, Debug)]
pub struct SpeedPlayerState {
    pub cur_gear: u8,
    pub cur_speed: u8,
    pub cur_place: u8,
    pub max_place: u8,
    pub car_mods: u8,
    pub light_goal: bool,
    pub light_car_lit: [bool; 5],
    pub light_car: [bool; 5],
}

#[derive(Clone, Copy, Debug)]
pub struct ShowPlayerState {
    pub prize_sets: u8,
}

#[derive(Clone, Copy, Debug)]
pub struct StonesPlayerState {
    pub cur_ghost: u8,
    pub ghost_active: bool,
    pub score_skill_shot: Bcd,
    pub kickback: bool,
    pub light_rip: [bool; 3],
    pub light_stone: [bool; 5],
    pub light_bone: [bool; 4],
}

impl Table {
    pub fn load_cur_player(&mut self) {
        let player = self.players[self.cur_player as usize - 1];
        self.score_main = player.score_main;
        self.score_bonus = player.score_bonus;
        self.num_cyclone = player.num_cyclone;
        self.bcd_num_cyclone = player.bcd_num_cyclone;
        match player.table {
            TablePlayerState::Party(party) => {
                self.light_load(LightBind::PartyPuke, party.light_puke);
                self.light_load(LightBind::PartyMad, party.light_mad);
                self.light_load(LightBind::PartyCrazy, party.light_crazy);
                self.light_load(LightBind::PartyParty, party.light_party);
                self.party.score_cyclone_skill_shot = party.score_cyclone_skill_shot;
                self.party.score_tunnel_skill_shot = party.score_tunnel_skill_shot;
            }
            TablePlayerState::Speed(speed) => {
                self.speed.cur_gear = speed.cur_gear;
                self.speed.cur_speed = speed.cur_speed;
                self.speed.car_mods = speed.car_mods;
                self.speed.cur_place = speed.cur_place;
                self.speed.max_place = speed.max_place;
                self.light_set(LightBind::SpeedPitStopGoal, 0, speed.light_goal);
                self.light_load(LightBind::SpeedCarPart, speed.light_car);
                self.light_load(LightBind::SpeedCarPartLit, speed.light_car_lit);
                self.speed_load_fixup();
            }
            TablePlayerState::Show(show) => {
                self.show.prize_sets = show.prize_sets;
                for i in 0..(show.prize_sets * 3) {
                    self.show.prizes[i as usize] = PrizeState::Taken;
                    self.light_set(LightBind::ShowPrize, i, true);
                }
            }
            TablePlayerState::Stones(stones) => {
                self.stones.cur_ghost = stones.cur_ghost;
                self.stones.ghost_active = stones.ghost_active;
                self.stones.score_skill_shot = stones.score_skill_shot;
                self.stones.kickback = stones.kickback;
                self.light_load(LightBind::StonesRip, stones.light_rip);
                self.light_load(LightBind::StonesStone, stones.light_stone);
                self.light_load(LightBind::StonesBone, stones.light_bone);
                self.stones_load_fixup();
            }
        };
    }

    pub fn save_cur_player(&mut self) {
        let player = PlayerState {
            score_main: self.score_main,
            score_bonus: self.score_bonus,
            num_cyclone: self.num_cyclone,
            bcd_num_cyclone: self.bcd_num_cyclone,
            table: match self.assets.table {
                TableId::Table1 => TablePlayerState::Party(PartyPlayerState {
                    light_puke: self.light_save(LightBind::PartyPuke),
                    light_mad: self.light_save(LightBind::PartyMad),
                    light_crazy: self.light_save(LightBind::PartyCrazy),
                    light_party: self.light_save(LightBind::PartyParty),
                    score_tunnel_skill_shot: self.party.score_tunnel_skill_shot,
                    score_cyclone_skill_shot: self.party.score_cyclone_skill_shot,
                }),
                TableId::Table2 => TablePlayerState::Speed(SpeedPlayerState {
                    cur_gear: self.speed.cur_gear,
                    cur_speed: self.speed.cur_speed,
                    cur_place: self.speed.cur_place,
                    max_place: self.speed.max_place,
                    car_mods: self.speed.car_mods,
                    light_goal: self.light_state(LightBind::SpeedPitStopGoal, 0),
                    light_car_lit: self.light_save(LightBind::SpeedCarPartLit),
                    light_car: self.light_save(LightBind::SpeedCarPart),
                }),
                TableId::Table3 => TablePlayerState::Show(ShowPlayerState {
                    prize_sets: self.show.prize_sets,
                }),
                TableId::Table4 => TablePlayerState::Stones(StonesPlayerState {
                    cur_ghost: self.stones.cur_ghost,
                    ghost_active: self.stones.ghost_active,
                    score_skill_shot: self.stones.score_skill_shot,
                    kickback: self.stones.kickback,
                    light_rip: self.light_save(LightBind::StonesRip),
                    light_stone: self.light_save(LightBind::StonesStone),
                    light_bone: self.light_save(LightBind::StonesBone),
                }),
            },
        };
        self.players[self.cur_player as usize - 1] = player;
    }
}
