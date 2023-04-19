use super::*;


/// Re-enables the ability to use aerial specials when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if (fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF))
    || WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_DAMAGE_REACTION_FRAME) > 0.0 {
        if let Some(mythra_id) = Some(fighter.battle_object_id + 0x10000){
            let mythra = crate::util::get_battle_object_from_id(mythra_id);
            if !mythra.is_null() {
                VarModule::off_flag(mythra, vars::elight::instance::DISABLE_SPECIAL_HI_JUMP);
            }
        }
    }
    true.into()
}

#[smashline::fighter_init]
fn eflame_init(fighter: &mut L2CFighterCommon) {
    unsafe {
        // set the callbacks on fighter init
        if fighter.kind() == *FIGHTER_KIND_EFLAME {
            fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
        }
    }
}


pub fn install() {
    smashline::install_agent_init_callbacks!(eflame_init);
}