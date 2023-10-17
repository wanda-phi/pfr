use arrayref::array_ref;
use rand::{thread_rng, Rng};
use unnamed_entity::EntityId;

use crate::{
    assets::table::{
        dm::DmFont,
        lights::LightBind,
        script::{DmCoord, ScriptBind, ScriptPosId, ScriptScore, Uop},
        sound::JingleBind,
    },
    bcd::Bcd,
    config::{HighScore, TableId},
};

use super::{
    dm::{
        ScriptTaskDmAnim, ScriptTaskDmLongMsg, ScriptTaskDmMsgScroll, ScriptTaskDmTowerHunt,
        ScriptTaskDmWipeDown, ScriptTaskDmWipeDownStriped, ScriptTaskDmWipeRight,
    },
    game::{ScriptTaskAccBonus, ScriptTaskMatch, ScriptTaskMatchStones},
    tasks::TaskKind,
    KbdState, Table,
};

pub struct ScriptState {
    pos: ScriptPosId,
    task: ScriptTask,
    timer_idle: u16,
    need_default_bg: bool,
    in_idle: bool,
    pub enter_attract: bool,
    repeat_cnt: u16,
}

#[derive(Debug)]
pub enum ScriptTask {
    Placeholder,
    Default,
    Delay(u16),
    Halt,
    ConfirmQuit,

    WaitJingle,

    WaitWhileGameStarting,
    AccBonus(ScriptTaskAccBonus),

    Mode(ScriptScore),

    DmClear,
    DmWipeDown(ScriptTaskDmWipeDown),
    DmWipeRight(ScriptTaskDmWipeRight),
    DmWipeDownStriped(ScriptTaskDmWipeDownStriped),
    DmMsgScroll(ScriptTaskDmMsgScroll),
    DmLongMsg(ScriptTaskDmLongMsg),
    DmAnim(ScriptTaskDmAnim),
    DmTowerHunt(ScriptTaskDmTowerHunt),

    Match(ScriptTaskMatch),
    MatchStones(ScriptTaskMatchStones),
    RecordHighScores,
    RecordHighScoresGetName(usize),
    RecordHighScoresFinish(u16),
}

impl ScriptTask {
    pub fn run(&mut self, table: &mut Table) -> bool {
        match *self {
            ScriptTask::Placeholder => unreachable!(),
            ScriptTask::Default => {
                if table.kbd_state != KbdState::Main {
                    return false;
                }
                if table.script.timer_idle == 720 {
                    if !table.at_spring {
                        table.script.in_idle = true;
                        table.start_script(ScriptBind::GameIdle);
                    }
                } else {
                    if table.script.enter_attract {
                        table.script.enter_attract = false;
                        table.start_script(ScriptBind::Attract);
                        return false;
                    } else if table.in_attract {
                        table.start_script(ScriptBind::GameOver);
                    } else if !table.in_plunger && table.script.need_default_bg {
                        table.script.need_default_bg = false;
                        table.run_uop(table.assets.script_binds[ScriptBind::Main].unwrap());
                    }
                    table.script.timer_idle += 1;
                }
                table.check_top_score();
                table.dm_put_bcd(
                    DmFont::H13,
                    DmCoord { x: 64, y: 1 },
                    table.score_main,
                    false,
                );
                false
            }
            ScriptTask::Delay(ref mut time) => {
                assert!(*time != 0);
                *time -= 1;
                *time != 0
            }
            ScriptTask::Halt => true,
            ScriptTask::ConfirmQuit => table.kbd_state == KbdState::ConfirmQuit || table.quitting,

            ScriptTask::WaitJingle => table.sequencer.jingle_playing(),

            ScriptTask::WaitWhileGameStarting => table.in_game_start,
            ScriptTask::AccBonus(ref mut task) => task.run(table),

            ScriptTask::Mode(score) => table.mode_frame(score),

            ScriptTask::DmClear => {
                table.dm.clear();
                false
            }
            ScriptTask::DmWipeDown(ref mut task) => task.run(table),
            ScriptTask::DmWipeRight(ref mut task) => task.run(table),
            ScriptTask::DmWipeDownStriped(ref mut task) => task.run(table),
            ScriptTask::DmMsgScroll(ref mut task) => task.run(table),
            ScriptTask::DmLongMsg(ref mut task) => task.run(table),
            ScriptTask::DmAnim(ref mut task) => task.run(table),
            ScriptTask::DmTowerHunt(ref mut task) => task.run(table),
            ScriptTask::Match(ref mut task) => task.run(table),
            ScriptTask::MatchStones(ref mut task) => task.run(table),
            ScriptTask::RecordHighScores => {
                if table.cur_player > table.total_players {
                    if !table.got_high_score {
                        table.play_jingle_bind_force(JingleBind::GameOverSad);
                    }
                    false
                } else {
                    let score = table.players[table.cur_player as usize - 1].score_main;
                    for place in 0..4 {
                        if score > table.high_scores[place].score {
                            if !table.got_high_score {
                                table.play_jingle_bind_force(JingleBind::GameOverHighScore);
                                table.got_high_score = true;
                            }
                            table.dm_puts(
                                DmFont::H13,
                                DmCoord { x: 0, y: 1 },
                                b"HIGHSCORE PL \x94 (   )",
                            );
                            *self = ScriptTask::RecordHighScoresGetName(place);
                            table.kbd_state = KbdState::GetName;
                            table.name_buf.clear();
                            return true;
                        }
                    }
                    table.cur_player += 1;
                    true
                }
            }
            ScriptTask::RecordHighScoresGetName(place) => {
                let name = table.name_buf.clone();
                table.dm_puts(
                    DmFont::H13,
                    DmCoord {
                        x: 160 - 4 * 8,
                        y: 1,
                    },
                    &name,
                );
                if name.len() == 3 {
                    let score = HighScore {
                        score: table.players[table.cur_player as usize - 1].score_main,
                        name: *array_ref![name, 0, 3],
                    };
                    table.high_scores.copy_within(place..3, place + 1);
                    table.high_scores[place] = score;
                    table.cur_player += 1;
                    table.flush_high_scores = true;
                    *self = ScriptTask::RecordHighScoresFinish(60);
                }
                true
            }
            ScriptTask::RecordHighScoresFinish(ref mut delay) => {
                *delay -= 1;
                if *delay == 30 {
                    table.dm.clear();
                }
                if *delay == 2 {
                    *self = ScriptTask::RecordHighScores;
                }
                true
            }
        }
    }
}

impl ScriptState {
    pub fn new() -> Self {
        ScriptState {
            pos: ScriptPosId::from_idx(0),
            task: ScriptTask::Placeholder,
            need_default_bg: false,
            timer_idle: 718,
            in_idle: false,
            enter_attract: true,
            repeat_cnt: 0,
        }
    }
}

impl Table {
    pub fn script_frame(&mut self) {
        let mut task = core::mem::replace(&mut self.script.task, ScriptTask::Placeholder);
        if task.run(self) {
            if matches!(self.script.task, ScriptTask::Placeholder) {
                self.script.task = task;
            }
        } else {
            self.run_uop(self.script.pos);
        }
    }

    pub fn start_script(&mut self, bind: ScriptBind) {
        self.start_script_raw(self.assets.script_binds[bind].unwrap());
    }

    pub fn start_script_raw(&mut self, pos: ScriptPosId) {
        self.dm.stop_blink();
        self.script.need_default_bg = true;
        self.script.timer_idle = 0;
        self.run_uop(pos);
    }

    pub fn script_score(&self, which: ScriptScore) -> Bcd {
        match which {
            ScriptScore::Bonus => self.score_bonus,
            ScriptScore::ModeHit => self.score_mode_hit,
            ScriptScore::ModeRamp => self.score_mode_ramp,
            ScriptScore::Jackpot => self.score_jackpot,
            ScriptScore::HighScore(idx) => self.high_scores[idx].score,
            ScriptScore::Const(x) => x,
            ScriptScore::CycloneIncr => Bcd::from_ascii(b"100000"),
            ScriptScore::NumCyclone => self.bcd_num_cyclone,
            ScriptScore::CycloneBonus => self.score_cyclone_bonus,
            ScriptScore::PartyTunnelSkillShot => self.party.score_tunnel_skill_shot,
            ScriptScore::PartyCycloneSkillShot => self.party.score_cyclone_skill_shot,
            ScriptScore::ShowRaisingMillions => self.score_raising_millions,
            ScriptScore::ShowSpinWheel => self.show_wheel_score(),
            ScriptScore::ShowCashpot => self.show.score_cashpot,
            ScriptScore::ShowCashpotX5 => {
                let mut score = Bcd::ZERO;
                for _ in 0..5 {
                    score += self.show.score_cashpot;
                }
                score
            }
            ScriptScore::StonesSkillShot => self.stones.score_skill_shot,
            ScriptScore::StonesMillionPlus => self.stones.score_million_plus,
            ScriptScore::StonesVault => self.stones.score_vault,
            ScriptScore::StonesWell => self.stones.score_well,
            ScriptScore::StonesTowerBonus => self.stones.score_tower_bonus,
        }
    }

    pub fn run_uop(&mut self, pos: ScriptPosId) {
        self.script.pos = pos + 1;
        let uop = self.assets.scripts[pos];
        match uop {
            Uop::End => {
                self.dm.stop_blink();
                self.script.task = ScriptTask::Default;
                self.script.pos = pos;
            }
            Uop::Noop => {
                self.script.task = ScriptTask::Delay(1);
            }
            Uop::Delay(delay) => self.script.task = ScriptTask::Delay(delay),
            Uop::DelayIfMultiplayer(delay) => {
                self.script.task =
                    ScriptTask::Delay(if self.total_players != 1 { delay } else { 2 })
            }
            Uop::Halt => self.script.task = ScriptTask::Halt,
            Uop::Jump(tgt) => self.run_uop(tgt),
            Uop::JccScoreZero(score, tgt) => {
                if self.script_score(score) == Bcd::ZERO {
                    self.run_uop(tgt);
                } else {
                    self.run_uop(self.script.pos);
                }
            }
            Uop::JccNoBonusMult(tgt) => {
                if self.bonus_mult_late == 1 {
                    self.run_uop(tgt);
                } else {
                    self.script.task = ScriptTask::Delay(1);
                }
            }
            Uop::RepeatSetup(cnt) => {
                self.script.task = ScriptTask::Delay(1);
                self.script.repeat_cnt = cnt;
            }
            Uop::RepeatLoop(cnt, target) => {
                self.script.task = ScriptTask::Delay(1);
                self.script.repeat_cnt -= 1;
                if self.script.repeat_cnt == 0 {
                    self.script.repeat_cnt = cnt;
                } else {
                    self.run_uop(target);
                }
            }
            Uop::FinalScoreSetup => {
                self.script.task = ScriptTask::Delay(1);
                self.cur_player = 1;
            }
            Uop::FinalScoreLoop(target) => {
                self.script.task = ScriptTask::Delay(1);
                self.dm_puts(DmFont::H5, DmCoord { x: 0, y: 1 }, b"PLAYER \x96");
                self.dm_put_bcd(
                    DmFont::H13,
                    DmCoord { x: 64, y: 1 },
                    self.players[self.cur_player as usize - 1].score_main,
                    false,
                );
                if self.cur_player != self.total_players {
                    self.cur_player += 1;
                    self.script.pos = target;
                }
            }
            Uop::ConfirmQuit => self.script.task = ScriptTask::ConfirmQuit,

            Uop::WaitWhileGameStarting => self.script.task = ScriptTask::WaitWhileGameStarting,
            Uop::ExtraBall => {
                self.script.task = ScriptTask::Delay(1);
                self.extra_ball();
            }
            Uop::SetupPartyOn => {
                self.script.task = ScriptTask::Delay(1);
                self.special_plunger_event = true;
            }
            Uop::SetupShootAgain => {
                self.script.task = ScriptTask::Delay(1);
            }
            Uop::SetSpecialPlungerEvent => {
                self.script.task = ScriptTask::Halt;
                self.special_plunger_event = true;
            }
            Uop::IssueBall => {
                self.add_task(TaskKind::IssueBall);
                self.run_uop(self.script.pos);
            }

            Uop::MultiplyBonus => {
                self.script.task = ScriptTask::Delay(1);
                let bonus = self.score_bonus;
                for _ in 1..self.bonus_mult_late {
                    self.score_bonus += bonus;
                }
            }
            Uop::AccBonusModeHit => {
                self.script.task = ScriptTask::Delay(1);
                self.score_bonus += self.score_mode_hit;
            }
            Uop::AccBonusModeRamp => {
                self.script.task = ScriptTask::Delay(1);
                self.score_bonus += self.score_mode_ramp;
            }
            Uop::AccBonusCyclones => {
                self.script.task = ScriptTask::Delay(1);
                self.score_cyclone_bonus = Bcd::ZERO;
                for _ in 0..self.num_cyclone {
                    self.score_cyclone_bonus += Bcd::from_ascii(b"100000");
                }
                self.score_bonus += self.score_cyclone_bonus;
            }
            Uop::AccBonus => {
                self.script.task = ScriptTask::AccBonus(ScriptTaskAccBonus::new(self.score_bonus));
            }
            Uop::CheckTopScore => {
                if !self.got_top_score && self.score_main > self.high_scores[0].score {
                    self.got_top_score = true;
                    self.run_uop(self.assets.script_binds[ScriptBind::TopScoreInterball].unwrap());
                } else {
                    self.run_uop(self.script.pos);
                }
            }
            Uop::NextBallIfMatched => {
                if self.match_digit.is_some() {
                    self.save_cur_player();
                    if self.extra_balls != 0 {
                        self.extra_balls -= 1;
                        self.run_uop(self.assets.script_binds[ScriptBind::ShootAgain].unwrap());
                    } else if self.cur_player != self.total_players {
                        self.cur_player += 1;
                        self.run_uop(self.assets.script_binds[ScriptBind::CheckMatch].unwrap());
                    } else {
                        self.run_uop(self.assets.script_binds[ScriptBind::PostMatch].unwrap());
                    }
                } else {
                    self.run_uop(self.script.pos);
                }
            }
            Uop::NextBall => {
                if !self.hold_bonus {
                    self.score_bonus = Bcd::ZERO;
                }
                self.save_cur_player();
                if self.extra_balls != 0 {
                    self.extra_balls -= 1;
                    self.run_uop(self.assets.script_binds[ScriptBind::ShootAgain].unwrap());
                } else if self.cur_player != self.total_players {
                    self.cur_player += 1;
                    self.add_task(TaskKind::IssueBall);
                    self.run_uop(self.script.pos);
                } else if self.cur_ball != self.total_balls {
                    self.cur_ball += 1;
                    self.cur_player = 1;
                    self.add_task(TaskKind::IssueBall);
                    self.run_uop(self.script.pos);
                } else {
                    self.run_uop(self.assets.script_binds[ScriptBind::Match].unwrap());
                }
            }

            Uop::Match => {
                self.cur_player = 1;
                self.play_jingle_bind_silence(JingleBind::MatchStart);
                for i in 0..self.players.len() {
                    let digit = self.players[i].score_main.digits[10];
                    self.dm_puts(
                        DmFont::H5,
                        DmCoord {
                            x: i as i16 * 16,
                            y: 0,
                        },
                        &[b'0' + digit],
                    );
                }
                let digit = thread_rng().gen_range(0..10);
                self.script.task = match self.assets.table {
                    TableId::Table1 => ScriptTask::Match(ScriptTaskMatch {
                        count: 22,
                        frames: if self.hifps { 11 } else { 9 },
                        frames_reload: if self.hifps { 11 } else { 9 },
                        digit,
                    }),
                    TableId::Table2 => ScriptTask::Match(ScriptTaskMatch {
                        count: 18,
                        frames: if self.hifps { 13 } else { 11 },
                        frames_reload: if self.hifps { 13 } else { 11 },
                        digit,
                    }),
                    TableId::Table3 => ScriptTask::Match(ScriptTaskMatch {
                        count: 15,
                        frames: 14,
                        frames_reload: 14,
                        digit,
                    }),
                    TableId::Table4 => ScriptTask::MatchStones(ScriptTaskMatchStones {
                        frames: self.match_timing[0],
                        timing_idx: 0,
                        digit,
                    }),
                };
            }
            Uop::CheckMatch => {
                let mut found = false;
                for (i, player) in self.players.iter().enumerate() {
                    if self.match_digit == Some(player.score_main.digits[10]) {
                        self.cur_player = i as u8 + 1;
                        self.run_uop(self.assets.script_binds[ScriptBind::ShootAgain].unwrap());
                        found = true;
                        break;
                    }
                }
                if !found {
                    self.run_uop(self.assets.script_binds[ScriptBind::PostMatch].unwrap());
                }
            }
            Uop::RecordHighScores => {
                self.cur_player = 1;
                self.script.task = ScriptTask::RecordHighScores;
            }
            Uop::GameOver => {
                self.add_task(TaskKind::GameOver);
                // emulation of bug in the DOS original
                self.script.task = ScriptTask::Default;
            }

            Uop::DmState(state) => {
                self.script.task = ScriptTask::Delay(1);
                self.dm.set_state(state);
            }
            Uop::DmBlink(period) => {
                self.script.task = ScriptTask::Delay(1);
                self.dm.start_blink(period);
            }
            Uop::DmStopBlink => {
                self.script.task = ScriptTask::Delay(1);
                self.dm.stop_blink();
            }
            Uop::DmClear => self.script.task = ScriptTask::DmClear,
            Uop::DmWipeDown => {
                self.script.task = ScriptTask::DmWipeDown(ScriptTaskDmWipeDown::new());
            }
            Uop::DmWipeRight => {
                self.script.task = ScriptTask::DmWipeRight(ScriptTaskDmWipeRight::new());
            }
            Uop::DmWipeDownStriped => {
                self.script.task =
                    ScriptTask::DmWipeDownStriped(ScriptTaskDmWipeDownStriped::new());
            }
            Uop::DmAnim(anim) => {
                self.script.task = ScriptTask::DmAnim(ScriptTaskDmAnim::new(self, anim));
            }
            Uop::DmPuts(font, pos, msg) => {
                self.script.task = ScriptTask::Delay(1);
                self.dm_puts(font, pos, &self.assets.msgs[msg].clone());
            }
            Uop::DmPrintScore(font, center, pos, which) => {
                self.script.task = ScriptTask::Delay(1);
                if which == ScriptScore::ShowSpinWheel {
                    self.dm.clear();
                }
                let bcd = self.script_score(which);
                self.dm_put_bcd(font, pos, bcd, center);
            }
            Uop::DmMsgScrollUp(msg, target) => {
                self.script.task =
                    ScriptTask::DmMsgScroll(ScriptTaskDmMsgScroll::new(msg, target, false));
            }
            Uop::DmMsgScrollDown(msg, target) => {
                self.script.task =
                    ScriptTask::DmMsgScroll(ScriptTaskDmMsgScroll::new(msg, target, true));
            }
            Uop::DmLongMsg(msg) => {
                self.script.task = ScriptTask::DmLongMsg(ScriptTaskDmLongMsg::new(msg));
            }
            Uop::DmTowerHunt(pos) => {
                self.script.task = ScriptTask::DmTowerHunt(ScriptTaskDmTowerHunt::new(pos));
            }

            Uop::SetJingleTimeout(_) => self.script.task = ScriptTask::Delay(1),
            Uop::WaitJingle | Uop::WaitJingleTimeout => self.script.task = ScriptTask::WaitJingle,
            Uop::PlayJingle(jingle) => {
                self.script.task = ScriptTask::Delay(1);
                self.sequencer.play_jingle(jingle, true, None);
            }
            Uop::PlaySfx(sfx, volume) => {
                self.script.task = ScriptTask::Delay(1);
                self.player.play_sfx(sfx, volume);
            }

            Uop::SetMusic(position) => {
                self.script.task = ScriptTask::Delay(1);
                self.sequencer.set_music(position)
            }
            Uop::ModeContinue(_, score) => {
                self.pending_mode = false;
                self.mode_timeout_frames = 1;
                self.script.task = ScriptTask::Mode(score);
            }
            Uop::ModeStart(timeout, score) => {
                self.pending_mode = false;
                self.mode_timeout_secs = timeout + 1;
                self.mode_timeout_frames = 1;
                self.script.task = ScriptTask::Mode(score);
            }
            Uop::ModeStartOrContinue(timeout, score) => {
                if self.pending_mode {
                    self.pending_mode = false;
                    self.mode_timeout_secs = timeout + 1;
                }
                self.mode_timeout_frames = 1;
                self.script.task = ScriptTask::Mode(score);
            }

            Uop::PartySecretDrop => {
                self.script.task = ScriptTask::Delay(1);
                self.party.secret_drop_release = true;
            }
            Uop::PartyArcadeReady => {
                self.script.task = ScriptTask::Delay(1);
                self.party.arcade_ready = true;
            }

            Uop::SpeedCheckTurboCont => {
                self.script.task = ScriptTask::Delay(1);
                if self.timer_stop {
                    self.timer_stop = false;
                    self.run_uop(
                        self.assets.script_binds[ScriptBind::SpeedModeRampContinue].unwrap(),
                    );
                }
            }
            Uop::SpeedClearFlagMode => {
                self.script.task = ScriptTask::Delay(1);
                self.in_mode = false;
                self.sequencer.reset_priority();
            }
            Uop::SpeedStartTurbo => {
                self.light_set(LightBind::SpeedPitStopGoal, 0, false);
                self.speed_do_turbo();
                self.run_uop(self.assets.script_binds[ScriptBind::SpeedModeRamp].unwrap());
            }

            Uop::ShowBlinkMoneyMania => {
                self.script.task = ScriptTask::Delay(1);
                self.light_blink(LightBind::ShowMoneyMania, 0, 3, 0);
            }
            Uop::ShowEndMoneyMania => {
                self.script.task = ScriptTask::Delay(1);
                self.light_set(LightBind::ShowMoneyMania, 0, false);
                self.in_mode = false;
                self.sequencer.reset_priority();
            }
            Uop::ShowSpinWheelEnd => {
                self.script.task = ScriptTask::Delay(1);
                self.add_task(TaskKind::ShowSpinWheelEnd);
            }
            Uop::StonesTowerEject => {
                self.script.task = ScriptTask::Delay(1);
                self.add_task(TaskKind::StonesTowerEject);
            }
            Uop::StonesVaultEject => {
                self.script.task = ScriptTask::Delay(1);
                self.add_task(TaskKind::StonesVaultEject);
                self.stones.vault_hold = false;
            }
            Uop::StonesWellEject => {
                self.script.task = ScriptTask::Delay(1);
                self.add_task(TaskKind::StonesWellEject);
            }
            Uop::StonesTiltEject => {
                self.script.task = ScriptTask::Delay(1);
                if self.stones.in_vault {
                    self.add_task(TaskKind::StonesVaultEject);
                } else if self.stones.in_tower {
                    self.add_task(TaskKind::StonesTowerEject);
                }
            }
            Uop::StonesSetFlagMode => {
                self.script.task = ScriptTask::Delay(1);
                self.in_mode = true;
            }
            Uop::StonesSetFlagModeRamp => {
                self.script.task = ScriptTask::Delay(1);
                self.in_mode_ramp = true;
            }
            Uop::StonesSetFlagModeHit => {
                self.script.task = ScriptTask::Delay(1);
                self.in_mode_hit = true;
            }
            Uop::StonesClearFlagMode => {
                self.script.task = ScriptTask::Delay(1);
                self.in_mode = false;
                self.sequencer.reset_priority();
            }
            Uop::StonesClearFlagModeRamp => {
                self.script.task = ScriptTask::Delay(1);
                self.in_mode_ramp = false;
            }
            Uop::StonesClearFlagModeHit => {
                self.script.task = ScriptTask::Delay(1);
                self.in_mode_hit = false;
            }
            Uop::StonesEndMode => {
                self.script.task = ScriptTask::Delay(1);
                self.stones_end_mode();
            }
            Uop::StonesEndGrimReaper => {
                self.script.task = ScriptTask::Delay(1);
                self.light_set(LightBind::StonesGhost, 7, false);
            }
        }
    }

    pub fn check_top_score(&mut self) {
        if self.in_plunger {
            return;
        }
        if self.in_mode {
            return;
        }
        if self.got_top_score {
            return;
        }
        if self.score_main > self.high_scores[0].score {
            self.got_top_score = true;
            self.start_script(ScriptBind::TopScoreIngame);
        }
    }

    pub fn reset_idle(&mut self) {
        self.script.timer_idle = 0;
        if self.script.in_idle {
            self.start_script(ScriptBind::Main);
            self.script.in_idle = false;
        }
    }
}
