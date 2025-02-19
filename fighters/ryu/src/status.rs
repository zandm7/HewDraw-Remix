use super::*;
use globals::*;

utils::import_noreturn!(common::shoto_status::{
    fgc_pre_dashback,
    fgc_end_dashback,
    ryu_idkwhatthisis2
});

extern "Rust" {
    // from common::shoto_status
    fn ryu_kara_cancel(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn ryu_attack_main_uniq_chk(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn fgc_dashback_main(fighter: &mut L2CFighterCommon) -> L2CValue;
    fn ryu_attack_main_uniq_chk4(fighter: &mut L2CFighterCommon, param_1: L2CValue) -> L2CValue;
    fn ryu_final_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue;
    fn ryu_hit_cancel(fighter: &mut L2CFighterCommon, situation: L2CValue) -> L2CValue;
}

extern "C" {
    fn status_pre_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue;
}
extern "C" {
    fn status_end_EscapeAir(fighter: &mut L2CFighterCommon) -> L2CValue;
}

// status script import
 
pub fn install() {
    CustomStatusManager::add_new_agent_status_script(
        Hash40::new("fighter_kind_ryu"),
        statuses::ryu::AIR_DASH,
        StatusInfo::new()
            .with_init(air_dash_init)
            .with_pre(status_pre_EscapeAir)
            .with_main(air_dash_main)
            .with_end(status_end_EscapeAir)
    );
    install_status_scripts!(
        pre_turndash,
        pre_dashback,
        main_dashback,
        end_dashback,
        main_attack,
        escape_air_pre
    );
}

// FIGHTER_STATUS_KIND_TURN_DASH //

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_TURN_DASH, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_turndash(fighter: &mut L2CFighterCommon) -> L2CValue {
    let lr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_SPECIAL_COMMAND_USER_INSTANCE_WORK_ID_FLOAT_OPPONENT_LR_1ON1);
    if lr != 0.0 {
        if PostureModule::lr(fighter.module_accessor) == lr {
            if fighter.global_table[PREV_STATUS_KIND] != FIGHTER_STATUS_KIND_TURN {
                StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_RYU_STATUS_KIND_DASH_BACK);
                return L2CValue::I32(1);
            }
        }
    }
    StatusModule::set_status_kind_interrupt(fighter.module_accessor, *FIGHTER_STATUS_KIND_TURN);
    return 1.into()
}

// FIGHTER_RYU_STATUS_KIND_DASH_BACK //

#[status_script(agent = "ryu", status = FIGHTER_RYU_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
pub unsafe fn pre_dashback(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::shoto_status::fgc_pre_dashback(fighter);
    original!(fighter)
}

#[status_script(agent = "ryu", status = FIGHTER_RYU_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
pub unsafe fn main_dashback(fighter: &mut L2CFighterCommon) -> L2CValue {
    fgc_dashback_main(fighter)
}

#[status_script(agent = "ryu", status = FIGHTER_RYU_STATUS_KIND_DASH_BACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_END)]
pub unsafe fn end_dashback(fighter: &mut L2CFighterCommon) -> L2CValue {
    common::shoto_status::fgc_end_dashback(fighter);
    original!(fighter)
}

// FIGHTER_STATUS_KIND_ATTACK //

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_ATTACK, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn main_attack(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_AttackCommon();
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_NEAR_OPPONENT) {
        WorkModule::set_int64(fighter.module_accessor, 0x10556e6036, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK_NEAR, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
    }
    else {
        WorkModule::set_int64(fighter.module_accessor, 0xb4f4e6f8f, *FIGHTER_STATUS_ATTACK_WORK_INT_ATTACK11_MOTION);
        WorkModule::set_int(fighter.module_accessor, *FIGHTER_LOG_ATTACK_KIND_ATTACK11, *FIGHTER_RYU_STATUS_ATTACK_INT_LOG_KIND);
    }
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_attack_main_uniq_chk(fighter);
    }
    fighter.global_table[SUB_STATUS3].assign(&L2CValue::Ptr(ryu_attack_main_uniq_chk as *const () as _));
    if !StopModule::is_stop(fighter.module_accessor) {
        ryu_attack_main_uniq_chk4(fighter, false.into());
    }
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(ryu_attack_main_uniq_chk4 as *const () as _));
    fighter.sub_shift_status_main(L2CValue::Ptr(ryu_attack_main_loop as *const () as _))
}

unsafe extern "C" fn ryu_attack_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !CancelModule::is_enable_cancel(fighter.module_accessor) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_INSTANCE_WORK_ID_FLAG_FINAL_HIT_CANCEL) {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
                if ryu_final_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                    return 1.into();
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_HIT_CANCEL) {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_SHIELD | *COLLISION_KIND_MASK_HIT) {
                if ryu_hit_cancel(fighter, SITUATION_KIND_GROUND.into()).get_bool() {
                    return 1.into();
                }
            }
        }
    }
    if ComboModule::count(fighter.module_accessor) == 1 {
        if !CancelModule::is_enable_cancel(fighter.module_accessor) {
            let current_frame = fighter.global_table[CURRENT_FRAME].get_f32();
            let attack_start_cancel_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_start_cancel_frame"));
            if current_frame < attack_start_cancel_frame {
                if ryu_kara_cancel(fighter).get_bool() {
                    return 1.into();
                }
            }
        }
    }
    if CancelModule::is_enable_cancel(fighter.module_accessor) {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool() {
            return 1.into();
        }
    }
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if [
        hash40("attack_11_w"),
        hash40("attack_11_s"),
        hash40("attack_11_near_s")
    ].contains(&mot) {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_WEAK_CANCEL) {
            if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_BUTTON_TRIGGER) {
                if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    let stick_y = fighter.global_table[STICK_Y].get_f32();
                    let attack_hi3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_hi3_stick_y"));
                    let cont;
                    if !(stick_y < attack_hi3_stick_y) {
                        cont = false;
                    }
                    else {
                        let attack_lw3_stick_y = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_lw3_stick_y"));
                        if !(attack_lw3_stick_y < stick_y) {
                            cont = false;
                        }
                        else {
                            let stick_x = fighter.global_table[STICK_X].get_f32();
                            let attack_s3_stick_x = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("attack_s3_stick_x"));
                            cont = stick_x < attack_s3_stick_x;
                        }
                    }
                    if cont {
                        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
                        return 1.into();
                    }
                }
            }
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_FLAG_SAME_ATTACK_CANCEL) {
            let button_on_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_RYU_STATUS_ATTACK_INT_BUTTON_ON_FRAME);
            let attack_11_s_button_on_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("param_private"), hash40("attack_11_s_button_on_frame"));
            if attack_11_s_button_on_frame <= button_on_frame as f32 {
                fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
                return 1.into();
            }
        }
    }
    if fighter.global_table[SITUATION_KIND].get_i32() == *SITUATION_KIND_AIR {
        fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
        return 0.into();
    }
    // if 0 < WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_INT_RESERVE_ATTACK_MINI_JUMP_ATTACK_FRAME) {
    //     if !StopModule::is_stop(fighter.module_accessor) {
    //         if fighter.sub_check_button_jump().get_bool() {
                
    //         }
    //     }
    // }
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_RESTART) {
        if !MotionModule::is_end(fighter.module_accessor) {
            common::shoto_status::ryu_idkwhatthisis2(fighter);
        }
        else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), false.into());
        }
    }
    else {
        fighter.change_status(FIGHTER_STATUS_KIND_ATTACK.into(), false.into());
    }
    0.into()
}

// FIGHTER_STATUS_KIND_ESCAPE_AIR

#[status_script(agent = "ryu", status = FIGHTER_STATUS_KIND_ESCAPE_AIR, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_PRE)]
unsafe fn escape_air_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.battle_object, vars::common::instance::PERFECT_WAVEDASH) || fighter.handle_waveland(false) {
        VarModule::on_flag(fighter.battle_object, vars::common::status::SHOULD_WAVELAND);
        GroundModule::attach_ground(fighter.module_accessor, true);
        fighter.change_status(FIGHTER_STATUS_KIND_LANDING.into(), false.into());
        return 0.into();
    }
    if !fighter.is_prev_status(*FIGHTER_STATUS_KIND_JUMP_SQUAT) {
        let mut stick_value_x = ControlModule::get_stick_x(fighter.module_accessor);
        let stick_value_y = ControlModule::get_stick_y(fighter.module_accessor);
        let lr =  PostureModule::lr(fighter.module_accessor);
        if (stick_value_x.abs() > 0.66 && stick_value_y == 0.0) {
            // airdash gfx
            if (stick_value_x == 0.0) {
                stick_value_x = 0.001; // to avoid a divide by zero on the next instruction below
            }
            let angle = (stick_value_y/stick_value_x).atan();
            let norm_stick_pos = Vector3f { x: stick_value_x * -1.0 * lr, y: stick_value_y * -1.0, z: 0.0}.normalize();
            let pos1 = Vector3f { x: norm_stick_pos.x * 4.0, y: 8.0 + norm_stick_pos.y * 8.0, z: 0.};
            let pos2 = Vector3f { x: norm_stick_pos.x * 9.0, y: 8.0 + norm_stick_pos.y * 12.0, z: 0.};
            let rot = Vector3f { x:5.0, y:0.0, z: ((stick_value_x.signum() * 90.0) + 180. * angle/3.14159)};
            let effect_hash = Hash40::new("sys_down_smoke");

            EffectModule::req_on_joint(fighter.module_accessor, effect_hash, Hash40::new("top"),
            &pos1, &rot, 0.5, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, false, 0, 0, 0);

            // transition to custom airdash status
            // we use change_status rather than set_status_kind_interrupt here because we want ESCAPE_AIR's main loop to run exactly once before transitioning to the airdash status
            // this allows:
            // 1. Initial airdodge energy to be applied
            // 2. change_motion to run so we change into airdodge animation
            // 3. Certain flags/transition terms to be set, such as FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_ESCAPE_AIR
            let air_dash_status_kind = CustomStatusModule::get_agent_status_kind(fighter.battle_object, statuses::ryu::AIR_DASH);
            fighter.change_status(
                air_dash_status_kind.into(),
                false.into()
            );
            return 0.into();
        }
    }
    StatusModule::init_settings(
        fighter.module_accessor,
        app::SituationKind(*SITUATION_KIND_AIR),
        *FIGHTER_KINETIC_TYPE_MOTION_FALL,
        *GROUND_CORRECT_KIND_AIR as u32,
        app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE),
        false,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT,
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT,
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor,
        false,
        *FIGHTER_TREADED_KIND_DISABLE,
        false,
        false,
        false,
        0,
        0,
        0,
        0
    );
    0.into()
}

pub unsafe extern "C" fn air_dash_init(fighter: &mut L2CFighterCommon) -> L2CValue {
    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE);

    // Clear knockback energy for airdash out of hitstun
    fighter.clear_lua_stack();
    lua_args!(fighter, FIGHTER_KINETIC_ENERGY_ID_DAMAGE);
    app::sv_kinetic_energy::clear_speed(fighter.lua_state_agent);

    0.into()
}

// Air Dash main status
pub unsafe extern "C" fn air_dash_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.global_table[SUB_STATUS].assign(&L2CValue::Ptr(L2CFighterCommon_sub_escape_air_uniq as *const () as _));
    fighter.main_shift(air_dash_main_loop)
}

// Air Dash main loop
unsafe extern "C" fn air_dash_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let frame = fighter.global_table[CURRENT_FRAME].get_i32();

    if GroundModule::is_touch(fighter.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
        fighter.global_table[SITUATION_KIND].assign(&L2CValue::I32(*SITUATION_KIND_GROUND));
    }

    // TODO: Move to ACMD once airdash anim is made
    if frame == 6 {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("fall_aerial"), 0.0, 1.0, false, 0.0, false, false);
    }
    // Remove intan on f9
    if frame == 8 {
        HitModule::set_whole(fighter.module_accessor, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    // Airdash actionable on f9
    if frame >= 8 {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    // Enable drift on f30
    if frame == 29 {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ESCAPE_AIR_FLAG_SLIDE_ENABLE_CONTROL);
    }
    // Allow ledgegrab on f35
    if frame == 34 {
        smash_script::notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    
    if !StatusModule::is_changing(fighter.module_accessor) {
        if !fighter.sub_escape_air_common_main().get_bool() {
            fighter.sub_escape_check_rumble();
        }
    }
    0.into()
}