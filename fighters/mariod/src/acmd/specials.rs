
use super::*;





#[acmd_script( agent = "mariod", script = "game_specialn" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 16.0/(14.0-1.0));
        VarModule::off_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){ 
            VarModule::on_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL);
        }

        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL){
            FT_MOTION_RATE(fighter, 3.0/(14.0-10.0));
        } 
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL){
            ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 10.0, 69, 90, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 69, 90, 0, 50, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("arml"), 10.0, 69, 90, 0, 50, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
            HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            FT_MOTION_RATE(fighter, 1.0);
        }
        else {
            ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, 0);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL){
            AttackModule::clear_all(boma);
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
            FT_MOTION_RATE(fighter, 1.5);
        }
    }
}

#[acmd_script( agent = "mariod", script = "effect_specialn" , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_special_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_action_smoke_h"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.75, 0.6, 1.0);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("mariod_capsule_shoot"), Hash40::new("mariod_capsule_shoot"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.46, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        LANDING_EFFECT(fighter, Hash40::new("sys_atk_smoke"), Hash40::new("top"), -1, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_ice"), Hash40::new("arml"), 7.5, 0, 0, 0, 0, 0, 0.35, true);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice_landing"), Hash40::new("arml"), 7.5, 0, 0, 0, 0, 0, 0.75, true);
            LAST_EFFECT_SET_RATE(fighter, 0.75);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.5, 0.25, 1, 0.35);
        }
        else{
            FLASH(fighter, 1, 1, 0, 0.353);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {

        }
        else{
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.5, 0.25, 1, 0.75);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.5, 0.25, 1, 0.35);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
        }
        else{
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.75);
        }
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            COL_NORMAL(fighter);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_ice"), false, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_ice_landing"), false, true);
        }
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            COL_NORMAL(fighter);
        }
    }
}

#[acmd_script( agent = "mariod", script = "game_specialairn" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_air_n_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 16.0/(14.0-1.0));
        VarModule::off_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if (ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL_RAW)){ 
            VarModule::on_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL);
        }

        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL){
            FT_MOTION_RATE(fighter, 3.0/(14.0-10.0));
        } 
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL){
            ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 10.0, 69, 90, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("arml"), 10.0, 69, 90, 0, 50, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 2, 0, Hash40::new("arml"), 10.0, 69, 90, 0, 50, 4.5, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_MARIOD_CAPSULE, *ATTACK_REGION_PUNCH);
            HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
            HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
            FT_MOTION_RATE(fighter, 1.0);
        }
        else {
            ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRCAPSULE, false, 0);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL){
            AttackModule::clear_all(boma);
            HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
            FT_MOTION_RATE(fighter, 1.5);
        }
    }
    
}

#[acmd_script( agent = "mariod", script = "effect_specialairn" , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_special_air_n_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 11.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.4, true);
            LAST_EFFECT_SET_COLOR(fighter, 0.75, 0.6, 1.0);
        }
    }
    frame(lua_state, 13.0);
    if is_excute(fighter) {
        EFFECT_FLIP(fighter, Hash40::new("mariod_capsule_shoot"), Hash40::new("mariod_capsule_shoot"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.46, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        EFFECT_FLIP(fighter, Hash40::new("sys_smash_flash"), Hash40::new("sys_smash_flash"), Hash40::new("top"), -1, 8, 11, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_ice"), Hash40::new("arml"), 7.5, 0, 0, 0, 0, 0, 0.35, true);
            LAST_EFFECT_SET_RATE(fighter, 1.5);
            EFFECT_FOLLOW(fighter, Hash40::new("sys_ice_landing"), Hash40::new("arml"), 7.5, 0, 0, 0, 0, 0, 0.75, true);
            LAST_EFFECT_SET_RATE(fighter, 0.75);
            EffectModule::enable_sync_init_pos_last(boma);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.5, 0.25, 1, 0.35);
        }
        else{
            FLASH(fighter, 1, 1, 0, 0.353);
        }
    }
    frame(lua_state, 17.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {

        }
        else{
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.5, 0.25, 1, 0.75);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            
        }
    }
    frame(lua_state, 21.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.5, 0.25, 1, 0.35);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
        }
        else{
            COL_NORMAL(fighter);
        }
    }
    frame(lua_state, 30.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.75);
        }
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
        }
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            COL_NORMAL(fighter);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_hit_ice"), false, true);
            EFFECT_OFF_KIND(fighter, Hash40::new("sys_ice_landing"), false, true);
        }
    }
    frame(lua_state, 39.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            FLASH(fighter, 0.75, 0.75, 1.0, 0.35);
        }
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mario_fb_shoot"), false, false);
    }
    frame(lua_state, 42.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_N_CHILL_PILL) {
            COL_NORMAL(fighter);
        }
    }
}

#[acmd_script( agent = "mariod", script = "game_specials" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_S_ELECTRIC_BLANKET);
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, false, 0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIOD_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::on_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_S_ELECTRIC_BLANKET);
        }
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_S_ELECTRIC_BLANKET){
            FT_MOTION_RATE(fighter, 5.0/3.0);
            ArticleModule::set_rate(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, 3.0/5.0);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 361, 87, 0, 45, 4.5, 0.0, 13.0, 8.0, Some(0.0), Some(1.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 9.5, 361, 87, 0, 45, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
            shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 110, 100, 80, 0, 4.5, 0.0, 13.0, 8.0, Some(0.0), Some(1.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13c64f9fca), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_MANT, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 110, 100, 80, 0, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13c64f9fca), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_MANT, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 0.0, 35, 100, 45, 0, 10, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(13.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_S_ELECTRIC_BLANKET){
            WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_S_ELECTRIC_BLANKET){
            let motion_rate = 25.0/(46.0-23.0);
            FT_MOTION_RATE(fighter, motion_rate);
            ArticleModule::set_rate(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, 1.0/motion_rate);
        }
        else{
            let motion_rate = 13.0/(46.0-23.0);
            FT_MOTION_RATE(fighter, motion_rate);
            ArticleModule::set_rate(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, 1.0/motion_rate);
        }
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    
}

#[acmd_script( agent = "mariod", script = "effect_specials" , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_special_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_down_smoke"), Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(fighter, 1.2);
        if PostureModule::lr(boma) > 0.0{
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), 1, 7.5, 5, 0, 35, 0, 1.2, true, *EF_FLIP_NONE);
        }
        else{
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), -1, 7.5, 5, 0, -35, 0, 1.2, true, *EF_FLIP_NONE);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("mariod_supermant_flash"), Hash40::new("top"), 0, 5.5, 7.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_S_ELECTRIC_BLANKET){
            EFFECT_FOLLOW(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("top"), 1.0, 7.5, 5.0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("top"), 1.0, 7.5, 5.0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
        }
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
    }
}

#[acmd_script( agent = "mariod", script = "game_specialairs" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_air_s_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_S_ELECTRIC_BLANKET);
        ArticleModule::generate_article(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, false, 0);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        SEARCH(fighter, 0, 0, Hash40::new("top"), 8.0, 0.0, 6.5, 2.5, Some(0.0), Some(6.5), Some(8.0), *COLLISION_KIND_MASK_ATTACK, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        WorkModule::set_float(boma, 9.0, *FIGHTER_MARIOD_STATUS_SPECIAL_S_WORK_ID_FLOAT_REFLECT_MOTION_FRAME);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        if ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_SPECIAL) {
            VarModule::on_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_S_ELECTRIC_BLANKET);
        }
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_S_ELECTRIC_BLANKET){
            FT_MOTION_RATE(fighter, 5.0/3.0);
            ArticleModule::set_rate(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, 3.0/5.0);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 9.5, 361, 87, 0, 45, 4.5, 0.0, 13.0, 8.0, Some(0.0), Some(1.0), Some(8.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 9.5, 361, 87, 0, 45, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_OBJECT);
            shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        }
        else {
            ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 110, 100, 80, 0, 4.5, 0.0, 13.0, 8.0, Some(0.0), Some(1.0), Some(8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13c64f9fca), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_MANT, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 110, 100, 80, 0, 5.0, 0.0, 6.7, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new_raw(0x13c64f9fca), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_MARIOD_MANT, *ATTACK_REGION_OBJECT);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 0.0, 35, 100, 45, 0, 10, 0.0, 8.0, 8.0, Some(0.0), Some(8.0), Some(13.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_OBJECT);
        }
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
    }
    frame(lua_state, 18.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        AttackModule::clear_all(boma);
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_S_ELECTRIC_BLANKET){
            WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
        }
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_S_FLAG_SPECIAL_FALL);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_S_ELECTRIC_BLANKET){
            let motion_rate = 25.0/(46.0-23.0);
            FT_MOTION_RATE(fighter, motion_rate);
            ArticleModule::set_rate(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, 1.0/motion_rate);
        }
        else{
            let motion_rate = 13.0/(46.0-23.0);
            FT_MOTION_RATE(fighter, motion_rate);
            ArticleModule::set_rate(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, 1.0/motion_rate);
        }
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MARIOD_REFLECTOR_KIND_DRMANTLE, *FIGHTER_REFLECTOR_GROUP_EXTEND);
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        ArticleModule::remove_exist(boma, *FIGHTER_MARIOD_GENERATE_ARTICLE_DRMANTLE, app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
    frame(lua_state, 46.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    
}

#[acmd_script( agent = "mariod", script = "effect_specialairs" , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_special_air_s_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), 1, 7.5, 5, 0, 35, 0, 1.2, true, *EF_FLIP_NONE);
        }
        else{
            EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_supermant_wind_r"), Hash40::new("mariod_supermant_wind_l"), Hash40::new("top"), -1, 7.5, 5, 0, -35, 0, 1.2, true, *EF_FLIP_NONE);
        }
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("mariod_supermant_flash"), Hash40::new("top"), 0, 5.5, 7.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_S_ELECTRIC_BLANKET){
            EFFECT_FOLLOW(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("top"), 1.0, 7.5, 5.0, 0, 0, 0, 1.0, true);
            EFFECT_FOLLOW(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("top"), 1.0, 7.5, 5.0, 0, 0, 0, 1.0, true);
            LAST_EFFECT_SET_RATE(fighter, 0.5);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("mariod_supermant_wind_r"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("mariod_supermant_wind_l"), -1);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), true, true);
    }
}

#[acmd_script( agent = "mariod", script = "game_specialhi" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_HI_UNABLE_CANCEL);
        VarModule::off_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_HI_SWEETSPOT_HIT);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        // Linking hitbox into the sweetspot that you can use to combo off of if you cancel the early hit of up B
        ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 70, 100, 125, 0, 5.0, 0.0, 6.0, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 5.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_HI_SWEETSPOT_HIT) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 1, Hash40::new("top"), 12.0, 50, 102, 0, 30, 6.0, 0.0, 6.0, 9.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 366, 100, 35, 0, 6.25, 0.0, 6.5, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 366, 100, 35, 0, 5.0, 0.0, 6.5, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_HI_SWEETSPOT_HIT) {
            ATTACK(fighter, 0, 1, Hash40::new("top"), 6.0, 74, 66, 0, 64, 4.0, 0.0, 6.5, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_HI_SWEETSPOT_HIT) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 1, Hash40::new("top"), 2.0, 366, 100, 25, 0, 6.25, 0.0, 6.5, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 1, Hash40::new("top"), 2.0, 366, 100, 25, 0, 5.0, 0.0, 6.5, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_HI_SWEETSPOT_HIT) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_HI_SWEETSPOT_HIT) {
            AttackModule::clear_all(boma);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 150, 0, 40, 8.75, 0.0, 5.5, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 80, 150, 0, 40, 4.0, 0.0, 4.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "mariod", script = "effect_specialhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_special_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 1.2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("mariod_smash_aura"), Hash40::new("havel"), -1.5, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 0, 1, 0, 0, 0, 1.05, true);
        }
        else{
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 1, 0, 0, 0, 0, 1.05, true);
        }
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("havel"), 1.0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("mariod_superjump_power"), Hash40::new("handl"), 1.2, 0, 0, 0, 0, 0, 1.45, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("mariod_superjump_fnish"), -1);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("mariod_smash_impact"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("mariod_smash_aura"), -1);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_superjump_power"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), false, true);
    }
}

#[acmd_script( agent = "mariod", script = "game_specialairhi" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_air_hi_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        VarModule::off_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_HI_UNABLE_CANCEL);
        VarModule::off_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_HI_SWEETSPOT_HIT);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        ATTACK(fighter, 0, 1, Hash40::new("top"), 12.0, 50, 102, 0, 30, 6.0, 0.0, 6.0, 9.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    frame(lua_state, 4.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_HI_SWEETSPOT_HIT) {
            ATTACK(fighter, 0, 1, Hash40::new("top"), 6.0, 74, 66, 0, 64, 4.0, 0.0, 6.5, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 366, 100, 55, 0, 6.25, 0.0, 6.5, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 366, 100, 55, 0, 5.0, 0.0, 6.5, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_HI_SWEETSPOT_HIT) {
            AttackModule::clear_all(boma);
            ATTACK(fighter, 0, 1, Hash40::new("top"), 2.0, 366, 100, 40, 0, 6.25, 0.0, 6.5, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 1, Hash40::new("top"), 2.0, 366, 100, 40, 0, 5.0, 0.0, 6.5, 4.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        if !VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_HI_SWEETSPOT_HIT) {
            AttackModule::clear_all(boma);
        }
    }
    frame(lua_state, 20.0);
    if is_excute(fighter) {
        if VarModule::is_flag(fighter.battle_object, vars::mariod::status::IS_SPECIAL_HI_SWEETSPOT_HIT) {
            AttackModule::clear_all(boma);
        }
        else{
            ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 150, 0, 40, 5.5, 0.0, 8.75, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 80, 150, 0, 40, 3.5, 0.0, 4.0, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        }
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "mariod", script = "effect_specialairhi" , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_special_air_hi_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 1.2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("mariod_smash_aura"), Hash40::new("havel"), -1.5, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
    }
    frame(lua_state, 3.0);
    if is_excute(fighter) {
        if PostureModule::lr(boma) > 0.0{
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 0, 1, 0, 0, 0, 1.05, true);
        }
        else{
            EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("mariod_superjump_fnish"), Hash40::new("handl"), 2.5, 1, 0, 0, 0, 0, 1.05, true);
        }
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("mariod_smash_impact"), Hash40::new("havel"), 1.0, 0, 0, 0, 0, 0, 0.5, true, *EF_FLIP_YZ);
        LAST_EFFECT_SET_RATE(fighter, 0.75);
        EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("mariod_superjump_power"), Hash40::new("handl"), 1.2, 0, 0, 0, 0, 0, 1.45, true);
        LAST_EFFECT_SET_RATE(fighter, 2.0);
    }
    frame(lua_state, 6.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("mariod_superjump_fnish"), -1);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        EFFECT_DETACH_KIND(fighter, Hash40::new("mariod_smash_impact"), -1);
        EFFECT_DETACH_KIND(fighter, Hash40::new("mariod_smash_aura"), -1);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_superjump_power"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), false, true);
    }
}

#[acmd_script( agent = "mariod", script = "game_speciallw" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 2.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_DAMAGE_POWER, 5.0);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
    for _ in 0..6{
        if is_excute(fighter) {
            ATTACK(fighter, 4, 0, Hash40::new("top"), 1.6, 90, 100, 80, 0, 4.0, 0.0, 3.2, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 1.6, 105, 100, 45, 0, 4.5, 0.0, 9.0, -6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 1.6, 367, 100, 15, 0, 4.5, 0.0, 9.0, -6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 105, 100, 45, 0, 4.5, 0.0, 9.0, 6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6, 367, 100, 15, 0, 4.5, 0.0, 9.0, 6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 3.0);
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 46, 154, 0, 80, 6.0, 0.0, 12.0, 6.0, Some(0.0), Some(12.0), Some(-6.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 46, 154, 0, 80, 5.5, 0.0, 4.0, 2.5, Some(0.0), Some(4.0), Some(-2.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "mariod", script = "effect_speciallw" , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_special_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_cyclone_r"), Hash40::new("mariod_cyclone_l"), Hash40::new("top"), 0, 1, 0, 0, 180, 0, 0.8, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 29.0);
    if is_excute(fighter) {
        LANDING_EFFECT_FLIP(fighter, Hash40::new("sys_whirlwind_l"), Hash40::new("sys_whirlwind_r"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false, *EF_FLIP_NONE);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_cyclone_r"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_cyclone_l"), false, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 1.25, true);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_cyclone_impact"), Hash40::new("mariod_cyclone_impact"), Hash40::new("top"), 3, 9.5, 0, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_DETACH_KIND(fighter, Hash40::new("mariod_cyclone_impact"), -1);
        EFFECT_FOLLOW(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 1.25, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(lua_state, 41.0);
    if is_excute(fighter) {
        LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.85, 0, 0, 0, 0, 0, 0, false);
        LAST_EFFECT_SET_RATE(fighter, 1.5);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
    }
}

#[acmd_script( agent = "mariod", script = "game_specialairlw" , category = ACMD_GAME , low_priority)]
unsafe fn mariod_special_air_lw_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0.0);
    }
    for _ in 0..6{
        if is_excute(fighter) {
            ATTACK(fighter, 4, 0, Hash40::new("top"), 1.6, 90, 100, 80, 0, 4.0, 0.0, 3.2, 0.0, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(fighter, 2, 0, Hash40::new("top"), 1.6, 105, 100, 45, 0, 4.5, 0.0, 9.0, -6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(fighter, 3, 0, Hash40::new("top"), 1.6, 367, 100, 15, 0, 4.5, 0.0, 9.0, -6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 105, 100, 45, 0, 4.5, 0.0, 9.0, 6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6, 367, 100, 15, 0, 4.5, 0.0, 9.0, 6.5, None, None, None, 0.5, 1.2, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        wait(lua_state, 1.0);
        if is_excute(fighter) {
            AttackModule::clear_all(boma);
        }
        wait(lua_state, 3.0);
    }
    if is_excute(fighter) {
        WorkModule::on_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(boma, *FIGHTER_MARIOD_STATUS_SPECIAL_LW_FLAG_RISE);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 46, 154, 0, 80, 6.0, 0.0, 12.0, 6.0, Some(0.0), Some(12.0), Some(-6.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 46, 154, 0, 80, 5.5, 0.0, 4.0, 2.5, Some(0.0), Some(4.0), Some(-2.5), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    wait(lua_state, 2.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
    }
}

#[acmd_script( agent = "mariod", script = "effect_specialairlw" , category = ACMD_EFFECT , low_priority)]
unsafe fn mariod_special_air_lw_effect(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    wait(lua_state, 4.0);
    if is_excute(fighter) {
        EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("haver"), 0, 0, 2, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, false);
    }
    frame(lua_state, 8.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_cyclone_r"), Hash40::new("mariod_cyclone_l"), Hash40::new("top"), 0, -1.5, 0, 0, 180, 0, 0.8, true, *EF_FLIP_NONE);
        LAST_EFFECT_SET_RATE(fighter, 1.4);
    }
    frame(lua_state, 35.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_cyclone_r"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_cyclone_l"), false, true);
        EFFECT_FOLLOW(fighter, Hash40::new("mariod_smash_aura"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 1.25, true);
    }
    frame(lua_state, 40.0);
    if is_excute(fighter) {
        EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mariod_cyclone_impact"), Hash40::new("mariod_cyclone_impact"), Hash40::new("top"), 3, 9.5, 0, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        EFFECT_DETACH_KIND(fighter, Hash40::new("mariod_cyclone_impact"), -1);
        EFFECT_FOLLOW(fighter, Hash40::new("mariod_smash_impact"), Hash40::new("top"), 0.0, 8.0, 0.0, 0, 0, 0, 1.25, true);
        LAST_EFFECT_SET_RATE(fighter, 0.7);
    }
    frame(lua_state, 50.0);
    if is_excute(fighter) {
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_impact"), false, true);
        EFFECT_OFF_KIND(fighter, Hash40::new("mariod_smash_aura"), false, true);
    }

}

pub fn install() {
    install_acmd_scripts!(
        mariod_special_n_game,
        mariod_special_n_effect,
        mariod_special_air_n_game,
        mariod_special_air_n_effect,
        mariod_special_s_game,
        mariod_special_s_effect,
        mariod_special_air_s_game,
        mariod_special_air_s_effect,
        mariod_special_hi_game,
        mariod_special_hi_effect,
        mariod_special_air_hi_game,
        mariod_special_air_hi_effect,
        mariod_special_lw_game,
        mariod_special_lw_effect,
        mariod_special_air_lw_game,
        mariod_special_air_lw_effect,
    );
}

