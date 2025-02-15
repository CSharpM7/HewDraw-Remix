// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn bouncing_fish_return_cancel(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_LW_RETURN && boma.status_frame() > 14 {
        if situation_kind == *SITUATION_KIND_AIR {
            boma.check_jump_cancel(false);
            boma.check_airdodge_cancel();
        }
    }
}

unsafe fn nspecial_cancels(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32) {
    //PM-like neutral-b canceling
    if status_kind == *FIGHTER_SHEIK_STATUS_KIND_SPECIAL_N_CANCEL {
        if situation_kind == *SITUATION_KIND_AIR {
            if WorkModule::get_int(boma, *FT_SHEIK_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS) == *FIGHTER_STATUS_KIND_ESCAPE_AIR {
                WorkModule::set_int(boma, *STATUS_KIND_NONE, *FT_SHEIK_STATUS_SPECIAL_N_WORK_INT_CANCEL_STATUS);
            }
        }
    }
}

// Removes "variable landing lag" from Vanish reappearance
// always lands with flat special fall landing lag
unsafe fn vanish_landing_lag(fighter: &mut L2CFighterCommon) {
    if fighter.is_status(*FIGHTER_SHEIK_STATUS_KIND_SPECIAL_HI_END)
    && !StatusModule::is_changing(fighter.module_accessor)
    && fighter.is_prev_situation(*SITUATION_KIND_AIR)
    && fighter.is_situation(*SITUATION_KIND_GROUND) {
        fighter.change_status_req(*FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
    }
}

extern "Rust" {
    fn gimmick_flash(boma: &mut BattleObjectModuleAccessor);
}

// pub unsafe fn hitfall_aerials(fighter: &mut L2CFighterCommon, frame: f32) {
//     if fighter.is_status(*FIGHTER_STATUS_KIND_ATTACK_AIR) {
//         // only allow the last hit of uair to be hitfalled
//         if fighter.is_motion(Hash40::new("attack_air_hi")) {
//             if frame >= 23.0 && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
//                 fighter.check_hitfall();
//             }
//         }
//         else {
//             fighter.check_hitfall();
//         }
//     }
// }

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    bouncing_fish_return_cancel(fighter, boma, status_kind, situation_kind, cat[0], frame);
    nspecial_cancels(fighter, boma, status_kind, situation_kind);
    //hitfall_aerials(fighter, frame);
    vanish_landing_lag(fighter);
}

#[utils::macros::opff(FIGHTER_KIND_SHEIK )]
pub fn sheik_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
        sheik_frame(fighter)
    }
}

pub unsafe fn sheik_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}