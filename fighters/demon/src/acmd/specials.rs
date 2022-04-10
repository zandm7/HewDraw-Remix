
use super::*;

#[acmd_script( agent = "demon", script = "game_attackstep2" , category = ACMD_GAME , low_priority)]
unsafe fn kazuya_wind_god_fist_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(0.15, 0.0, 0.0));
        FT_MOTION_RATE(fighter, 0.7);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.5);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }  
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(-0.15, 0.0, 0.0));
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        ATTACK(fighter, 0, 0, Hash40::new("handr"), 13.5, 88, 10, 0, 98, 2.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.5, 88, 10, 0, 98, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 13.5, 88, 10, 0, 98, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("bust"), 13.5, 88, 10, 0, 98, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 13.5, 88, 10, 0, 98, 5.0, 0.0, 13.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 13.5, 88, 10, 0, 98, 3.0, -1.0, 9.0, 3.5, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 5, 1.2);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 4, 15.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 5, 15.0, false);
        //AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
        //AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_M, false);
        //AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_M, false);
        //AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_M, false);
        //AttackModule::set_attack_camera_quake_forced(boma, 4, *CAMERA_QUAKE_KIND_M, false);
        //AttackModule::set_attack_camera_quake_forced(boma, 5, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.9);
        AttackModule::clear(boma, 4, false);
        AttackModule::clear(boma, 5, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.9);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(boma);
    }
    
}

#[acmd_script( agent = "demon", script = "game_attackstep2f" , category = ACMD_GAME , low_priority)]
unsafe fn kazuya_electric_wind_god_fist_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
    }
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(0.15, 0.0, 0.0));
        FT_MOTION_RATE(fighter, 0.7);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.5);
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 10.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(-0.15, 0.0, 0.0));
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_NORMAL);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);

        ATTACK(fighter, 0, 0, Hash40::new("handr"), 14.5, 86, 20, 0, 85, 4.5, 0.0, 0.0, 0.0, None, None, None, 0.31, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("armr"), 14.5, 86, 20, 0, 85, 5.0, 0.0, 0.0, 0.0, None, None, None, 0.31, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("shoulderr"), 14.5, 86, 20, 0, 85, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.31, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("bust"), 14.5, 86, 20, 0, 85, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.31, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("top"), 14.5, 86, 20, 0, 85, 5.0, 0.0, 13.0, 6.0, None, None, None, 0.31, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 5, 0, Hash40::new("top"), 14.5, 86, 20, 0, 85, 3.0, -1.0, 9.0, 3.5, None, None, None, 0.31, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 5, 1.2);

        ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 5, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.2);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.2);
        AttackModule::set_add_reaction_frame_revised(boma, 0, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 1, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 2, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 3, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 4, 12.0, false);
        AttackModule::set_add_reaction_frame_revised(boma, 5, 12.0, false);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_S, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_S, false);
        AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_S, false);
        AttackModule::set_attack_camera_quake_forced(boma, 3, *CAMERA_QUAKE_KIND_S, false);
        AttackModule::set_attack_camera_quake_forced(boma, 4, *CAMERA_QUAKE_KIND_S, false);
        AttackModule::set_attack_camera_quake_forced(boma, 5, *CAMERA_QUAKE_KIND_S, false);
    }
    frame(lua_state, 12.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.9);
        AttackModule::clear(boma, 4, false);
        AttackModule::clear(boma, 5, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
    }
    frame(lua_state, 15.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 6, false);
        FT_MOTION_RATE(fighter, 0.9);
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
        
        
    
}

#[acmd_script( agent = "demon", script = "game_attackstep2s" , category = ACMD_GAME , low_priority)]
unsafe fn kazuya_spinning_demon_to_left_hook_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    frame(lua_state, 1.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(0.1, 0.0, 0.0));
    }
    frame(lua_state, 9.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("legr"), 4.0, 60, 100, 45, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.0, 72, 100, 45, 0, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        ATTACK(fighter, 2, 0, Hash40::new("kneer"), 4.0, 85, 100, 45, 0, 4.0, 5.0, 0.0, 0.0, None, None, None, 0.4, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, true, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_DEMON_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(boma, 0, 12.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 12.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 12.0, false);
    }
    frame(lua_state, 14.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 0.8);
    }
    frame(lua_state, 33.0);
    if is_excute(fighter) {
        KineticModule::add_speed(boma, &Vector3f::new(-0.1, 0.0, 0.0));
        FT_MOTION_RATE(fighter, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("top"), 11.5, 35, 30, 0, 75, 4.0, 0.0, 15.0, 7.0, None, None, None, 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 11.5, 35, 30, 0, 75, 5.0, 0.0, 14.0, 8.0, Some(0.0), Some(8.5), Some(8.0), 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 11.5, 35, 30, 0, 75, 5.0, 0.0, 13.0, 3.5, Some(0.0), Some(5.5), Some(3.5), 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 3, 0, Hash40::new("shoulderl"), 11.5, 35, 30, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 4, 0, Hash40::new("shoulderr"), 11.5, 35, 30, 0, 75, 3.5, 0.0, 0.0, 0.0, None, None, None, 0.4, 0.25, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(boma, 0, 8.0, false);
        AttackModule::set_add_reaction_frame(boma, 1, 8.0, false);
        AttackModule::set_add_reaction_frame(boma, 2, 8.0, false);
        AttackModule::set_add_reaction_frame(boma, 3, 8.0, false);
        AttackModule::set_add_reaction_frame(boma, 4, 8.0, false);
        AttackModule::set_attack_camera_quake(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake(boma, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake(boma, 2, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 36.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 0, false);
        AttackModule::clear(boma, 1, false);
        AttackModule::clear(boma, 2, false);
        AttackModule::clear(boma, 3, false);
        AttackModule::clear(boma, 4, false);
        if AttackModule::is_infliction_status(boma, *COLLISION_KIND_MASK_HIT) {
            ATTACK(fighter, 5, 1, Hash40::new("top"), 0.0, 235, 100, 3, 0, 10.0, 0.0, 10.0, 10.0, Some(0.0), Some(10.0), Some(15.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PUNCH);
        }
        
    }
    frame(lua_state, 45.0);
    if is_excute(fighter) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(fighter, 0.6);
    }
    
}

#[acmd_script( agent = "demon", script = "game_attackstep2l" , category = ACMD_GAME , low_priority)]
unsafe fn kazuya_lightning_uppercut_game(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    let boma = fighter.boma();
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 0.9);
    }
    frame(lua_state, 7.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("head"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("bust"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("shoulderr"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("armr"), *HIT_STATUS_XLU);
        if VarModule::is_flag(boma.object(), vars::demon::LIGHTNING_SCREW_UPPERCUT){
            FT_MOTION_RATE(fighter, 10.0/(19.0-7.0));
        }
    }
    frame(lua_state, 19.0);
    if is_excute(fighter) {
        FT_MOTION_RATE(fighter, 1.0);
    }
    frame(lua_state, 22.0);
    if is_excute(fighter) {
        HIT_NODE(fighter, Hash40::new("shoulderl"), *HIT_STATUS_XLU);
        HIT_NODE(fighter, Hash40::new("arml"), *HIT_STATUS_XLU);
        AttackModule::set_damage_shake_scale(boma, 1.5);
        ATTACK(fighter, 0, 0, Hash40::new("handl"), 22.0, 60, 65, 0, 80, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 22.0, 60, 65, 0, 80, 5.25, 0.0, 10.5, 4.75, Some(0.0), Some(16.0), Some(7.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 2, 0, Hash40::new("top"), 22.0, 60, 65, 0, 80, 5.25, 0.0, 5.0, 2.5, Some(0.0), Some(10.5), Some(4.75), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 0.5);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 2, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 23.0);
    if is_excute(fighter) {
        AttackModule::clear(boma, 2, false);
        AttackModule::set_damage_shake_scale(boma, 1.0);
        ATTACK(fighter, 0, 0, Hash40::new("handl"), 18.0, 70, 60, 0, 80, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.1), 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 18.0, 70, 60, 0, 80, 5.0, 0.0, 13.0, 3.0, Some(0.0), Some(23.0), Some(3.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 0.5);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 24.0);
    if is_excute(fighter) {
        ATTACK(fighter, 0, 0, Hash40::new("handl"), 16.0, 70, 50, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 16.0, 70, 50, 0, 60, 5.0, 0.0, 14.0, 3.0, Some(0.0), Some(24.0), Some(3.0), 0.17, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.5);
        ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 0.5);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_L, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_L, false);
    }
    frame(lua_state, 25.0);
    if is_excute(fighter) {
        HitModule::set_status_all(boma, app::HitStatus(*HIT_STATUS_NORMAL), 0);
        ATTACK(fighter, 0, 0, Hash40::new("handl"), 14.0, 70, 50, 0, 60, 3.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 70, 50, 0, 60, 5.0, 0.0, 21.0, 3.0, Some(0.0), Some(24.0), Some(3.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 26.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 70, 50, 0, 60, 5.0, 0.0, 21.5, 3.0, Some(0.0), Some(24.5), Some(3.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 27.0);
    if is_excute(fighter) {
        ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 70, 50, 0, 60, 5.0, 0.0, 22.0, 3.0, Some(0.0), Some(25.0), Some(3.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_DEMON_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_camera_quake_forced(boma, 0, *CAMERA_QUAKE_KIND_M, false);
        AttackModule::set_attack_camera_quake_forced(boma, 1, *CAMERA_QUAKE_KIND_M, false);
    }
    frame(lua_state, 28.0);
    FT_MOTION_RATE(fighter, 0.9);
    if is_excute(fighter) {
    AttackModule::clear_all(boma);
    }
    
}

pub fn install() {
    install_acmd_scripts!(
        kazuya_wind_god_fist_game,
        kazuya_electric_wind_god_fist_game,
        kazuya_spinning_demon_to_left_hook_game,
        kazuya_lightning_uppercut_game,
    );
}

