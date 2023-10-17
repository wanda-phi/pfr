use crate::{
    assets::table::{
        physics::{HitTrigger, Layer, PhysmapBind, RollTrigger},
        script::EffectBind,
        sound::SfxBind,
    },
    bcd::Bcd,
    config::TableId,
};

use super::Table;

impl Table {
    pub fn do_hit_triggers(&mut self) {
        if self.tilted {
            return;
        }
        let Some(mut hit_pos) = self.hit_pos.take() else {
            return;
        };
        hit_pos.1 += self.push.offset();
        if self.ball.layer != Layer::Ground {
            return;
        }
        for area in &self.assets.hit_triggers {
            if area.rect.contains(hit_pos) {
                match area.kind {
                    HitTrigger::PartyArcadeButton => self.party_arcade_button(),
                    HitTrigger::PartyDuck(which) => self.party_hit_duck(which),
                    HitTrigger::SpeedBur(which) => self.speed_hit_bur(which),
                    HitTrigger::SpeedNin(which) => self.speed_hit_nin(which),
                    HitTrigger::ShowDollar(which) => self.show_hit_dollar(which),
                    HitTrigger::ShowCenter(which) => self.show_hit_center(which),
                    HitTrigger::ShowLeft(which) => self.show_hit_left(which),
                    HitTrigger::StonesBone(which) => self.stones_hit_bone(which),
                    HitTrigger::StonesStone(which) => self.stones_hit_stone(which),
                }
                return;
            }
        }
    }

    pub fn do_roll_triggers(&mut self) {
        let pos = self.ball_center();
        let list = if self.tilted {
            &self.assets.roll_triggers_tilt[self.ball.layer]
        } else {
            &self.assets.roll_triggers[self.ball.layer]
        };
        for area in list {
            if area.rect.contains(pos) {
                if self.roll_trigger != Some(area.kind) {
                    self.roll_trigger = Some(area.kind);
                    self.do_roll_trigger(area.kind);
                    self.prev_roll_trigger = self.roll_trigger;
                }
                return;
            }
        }
        self.roll_trigger = None;
    }

    pub fn do_roll_trigger(&mut self, kind: RollTrigger) {
        match kind {
            RollTrigger::Dummy => (),
            RollTrigger::PlungerBottom => self.at_spring = true,
            RollTrigger::PlungerGo => {
                self.at_spring = false;
                match self.assets.table {
                    TableId::Table1 => {
                        self.party.timeout_skill_shot = 300;
                        self.party.timeout_spring_loop = 120;
                    }
                    TableId::Table2 => (),
                    TableId::Table3 => {
                        self.drop_physmap(PhysmapBind::ShowGatePlunger);
                    }
                    TableId::Table4 => (),
                }
            }
            RollTrigger::PartyLaneInner => {
                self.effect(EffectBind::PartyRollInner);
                self.play_sfx_bind(SfxBind::RollInner);
            }
            RollTrigger::PartyLaneOuter => self.party_lane_outer(),
            RollTrigger::PartyOrbitTopLeft => {
                if self.prev_roll_trigger == Some(RollTrigger::PartyOrbitTopRight) {
                    if self.party.timeout_spring_loop != 0 {
                        self.party.timeout_spring_loop = 0;
                    } else {
                        self.party_orbit_right();
                    }
                }
            }
            RollTrigger::PartyOrbitTopRight => {
                if self.prev_roll_trigger == Some(RollTrigger::PartyOrbitTopLeft) {
                    self.party_orbit_left();
                }
            }
            RollTrigger::PartySecret => self.party_secret(),
            RollTrigger::PartyTunnel => self.party_tunnel(),
            RollTrigger::PartyArcade => self.party_arcade(),
            RollTrigger::PartyOrbitEntryRight => self.party.timeout_spring_loop = 0,
            RollTrigger::PartyEnter => {
                if self.prev_roll_trigger == Some(RollTrigger::PlungerGo) {
                    self.enter();
                }
            }
            RollTrigger::PartyDemon => self.party_demon(),
            RollTrigger::PartySkyrideTop => {
                if self.prev_roll_trigger == Some(RollTrigger::PartySkyrideRamp) {
                    self.party_skyride_top();
                }
            }
            RollTrigger::PartySkyrideRamp => self.drop_physmap(PhysmapBind::PartyGateSkyride),
            RollTrigger::PartySkyridePuke(which) => self.party_puke(which),
            RollTrigger::PartyRampCyclone => self.party_ramp_cyclone(),
            RollTrigger::PartyRampSnack => self.party_ramp_snack(),
            RollTrigger::PartySecretTilt => self.party_secret_tilt(),
            RollTrigger::PartyTunnelTilt => self.party_tunnel_tilt(),
            RollTrigger::SpeedLaneInner => {
                self.play_sfx_bind(SfxBind::RollInner);
                self.effect(EffectBind::SpeedLaneInner);
            }
            RollTrigger::SpeedLaneOuter => {
                self.effect(EffectBind::SpeedLaneOuter);
            }
            RollTrigger::SpeedPitStop => self.speed_pit_stop(),
            RollTrigger::SpeedEnter => {
                if self.prev_roll_trigger == Some(RollTrigger::SpeedPlungerExit) {
                    self.enter();
                }
            }
            RollTrigger::SpeedPitLoopJump => match self.prev_roll_trigger {
                Some(RollTrigger::SpeedJumpPre) => self.speed_ramp_jump(),
                Some(RollTrigger::SpeedPitLoopPre) => self.speed_pit_loop(),
                _ => (),
            },
            RollTrigger::SpeedRampOffroad => self.speed_ramp_offroad(),
            RollTrigger::SpeedPitLoopPre => (),
            RollTrigger::SpeedPit(which) => self.speed_roll_pit(which),
            RollTrigger::SpeedOffroadExit => {
                self.effect(EffectBind::SpeedOffroadExit);
            }
            RollTrigger::SpeedRampMilesRight => {
                self.speed.timeout_miles_right = 390;
                if self.speed.timeout_miles_left != 0 {
                    self.speed.timeout_miles_left = 0;
                    self.speed_overtake();
                }
                self.speed_bump_miles();
            }
            RollTrigger::SpeedRampMilesLeft => {
                self.speed.timeout_miles_left = 390;
                if self.speed.timeout_miles_right != 0 {
                    self.speed.timeout_miles_right = 0;
                    self.speed_overtake();
                }
                self.speed_bump_miles();
            }
            RollTrigger::SpeedJumpPre => (),
            RollTrigger::SpeedPlungerExit => {
                self.ball.speed.1 = 0;
                self.ball.layer = Layer::Ground;
            }
            RollTrigger::ShowLaneInner => {
                self.effect(EffectBind::ShowLaneInner);
                self.play_sfx_bind(SfxBind::RollInner);
            }
            RollTrigger::ShowLaneOuter => {
                self.effect(EffectBind::ShowLaneOuter);
                self.play_sfx_bind_volume(SfxBind::RollTrigger, 0x20);
            }
            RollTrigger::ShowEnter => {
                if self.prev_roll_trigger == Some(RollTrigger::PlungerGo) {
                    self.raise_physmap(PhysmapBind::ShowGatePlunger);
                    self.enter();
                }
            }
            RollTrigger::ShowOrbitLeft => self.show_orbit_left(),
            RollTrigger::ShowOrbitRight => self.show_orbit_right(),
            RollTrigger::ShowCashpot => self.show_cashpot(),
            RollTrigger::ShowVault => self.show_vault(),
            RollTrigger::ShowVaultExit => self.raise_physmap(PhysmapBind::ShowGateVaultExit),
            RollTrigger::ShowRampSkillEntry => {
                self.effect(EffectBind::ShowSkillsEntry);
            }
            RollTrigger::ShowRampTopEntry => {
                self.effect(EffectBind::ShowTopEntry);
            }
            RollTrigger::ShowRampLoopEntry => {
                self.effect(EffectBind::ShowLoopEntry);
            }
            RollTrigger::ShowRampTop => self.show_ramp_top(),
            RollTrigger::ShowRampSkillMark => (),
            RollTrigger::ShowRampSkill => {
                if self.prev_roll_trigger == Some(RollTrigger::ShowRampSkillMark) {
                    self.show_ramp_skills()
                }
            }
            RollTrigger::ShowRampRight => self.show_ramp_right(),
            RollTrigger::ShowRampLoop => self.show_ramp_loop(),
            RollTrigger::ShowRampTopSecondary => self.incr_jackpot(),
            RollTrigger::StonesLaneInnerLeft | RollTrigger::StonesLaneInnerRight => {
                self.play_sfx_bind(SfxBind::RollInner);
                self.score_premult(Bcd::from_ascii(b"10070"), Bcd::from_ascii(b"1080"));
            }
            RollTrigger::StonesLaneOuterLeft => {
                self.play_sfx_bind(SfxBind::RollTrigger);
                self.score(Bcd::from_ascii(b"500010"), Bcd::ZERO);
            }
            RollTrigger::StonesLaneOuterRight => {
                self.play_sfx_bind(SfxBind::RollTrigger);
                self.score(Bcd::from_ascii(b"500030"), Bcd::ZERO);
            }
            RollTrigger::StonesKeyEntry => self.stones_roll_key_entry(),
            RollTrigger::StonesRampTower => {
                self.drop_physmap(PhysmapBind::StonesGateRampTower);
                self.mode_count_ramp();
            }
            RollTrigger::StonesKey(which) => self.stones_roll_key(which),
            RollTrigger::StonesWell => self.stones_well(),
            RollTrigger::StonesVault => self.stones_vault(),
            RollTrigger::StonesKeyClose => self.drop_physmap(PhysmapBind::StonesGateRampTower),
            RollTrigger::StonesTower => self.stones_tower(),
            RollTrigger::StonesRampTop => self.stones_ramp_top(),
            RollTrigger::StonesRip(which) => self.stones_roll_rip(which),
            RollTrigger::StonesRampTopExit => self.stones.timeout_top_loop = 300,
            RollTrigger::StonesRampScreams => self.stones_ramp_screams(),
            RollTrigger::StonesRampLeftToLane => self.stones_ramp_left_to_lane(),
            RollTrigger::StonesRampLeftToVault => self.stones_ramp_left_to_vault(),
            RollTrigger::StonesRampLeftFixup0 => {
                self.drop_physmap(PhysmapBind::StonesGateRampLeft1)
            }
            RollTrigger::StonesRampLeftFixup1 => {
                self.raise_physmap(PhysmapBind::StonesGateRampLeft1)
            }
            RollTrigger::StonesRampLeftFixup2 => {
                self.drop_physmap(PhysmapBind::StonesGateRampLeft2)
            }
            RollTrigger::StonesRampLeftFixup3 => {
                self.raise_physmap(PhysmapBind::StonesGateRampLeft2)
            }
            RollTrigger::StonesVaultExit => {
                self.stones.vault_from_ramp = true;
                self.stones_incr_vault();
            }
            RollTrigger::StonesEnter => {
                self.enter();
                self.ball.layer = Layer::Ground;
            }
            RollTrigger::StonesWellTilt => self.stones_well_tilt(),
            RollTrigger::StonesTowerTilt => self.stones_tower_tilt(),
        }
    }
}
