use smash::app::sv_module_access::*;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::phx::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::app::sv_battle_object::module_accessor;
use smash::app::sv_module_access::shield;
use skyline::nn::ro::LookupSymbol;
use smash::hash40;
use acmd::{acmd, acmd_func};
use std::mem;
use smash::app::{self, sv_information};
use smash::app::sv_animcmd::LAST_EFFECT_SET_COLOR;
use smash::app::BattleObjectModuleAccessor;

pub static mut LANDING: [bool; 8] = [false; 8];
pub static mut MONADO_STATE: [i32; 8] = [0; 8];
pub static mut SHIELD_ART_TIMER: [i32; 8] = [0; 8];
pub static mut BUSTER_ART_TIMER: [i32; 8] = [0; 8];
pub static mut SMASH_ART_TIMER: [i32; 8] = [0; 8];
pub static mut ACCELERATE_ART_TIMER: [i32; 8] = [0; 8];
pub static mut SHIELD_ART_COOLDOWN: [i32; 8] = [0; 8];
pub static mut BUSTER_ART_COOLDOWN: [i32; 8] = [0; 8];
pub static mut SMASH_ART_COOLDOWN: [i32; 8] = [0; 8];
pub static mut ACCELERATE_ART_COOLDOWN: [i32; 8] = [0; 8];
static mut SHIELD_GFX_COUNTER: [i32; 8] = [0; 8];
static mut BUSTER_GFX_COUNTER: [i32; 8] = [0; 8];
static mut SMASH_GFX_COUNTER: [i32; 8] = [0; 8];
static mut ACCELERATE_GFX_COUNTER: [i32; 8] = [0; 8];

//Shield Right
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "appeal_hi_r",
    animcmd = "game_appealhir")]
    pub fn Monado_Ganon_Shield_Right(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                        if MONADO_STATE[entry_id] == 1
                        && MONADO_STATE[entry_id] == 2
                        && MONADO_STATE[entry_id] == 3
                        && MONADO_STATE[entry_id] == 4 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                        }
                        if SHIELD_ART_COOLDOWN[entry_id] > 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                        } 
                        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                    }
                }
                frame(Frame=10)
                if(is_excute){
                    rust{
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
                frame(Frame=14)
                if(is_excute){
                    rust{
                        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                    }
                }
                frame(Frame=58)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            });
        }
//Shield Right Sound
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "appeal_hi_r",
    animcmd = "sound_appealhir")]
    pub fn Monado_Ganon_Shield_Right_Sound(fighter: &mut L2CFighterCommon) 
    {
        acmd!({
            frame(Frame=2)
            if(is_excute){
                sv_animcmd::PLAY_SE(hash40("vc_ganon_appeal_h01"));
            }
        });
    }
//Shield Left
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "appeal_hi_l",
    animcmd = "game_appealhil")]
    pub fn Monado_Ganon_Shield_Left(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                        if MONADO_STATE[entry_id] == 1
                        && MONADO_STATE[entry_id] == 2
                        && MONADO_STATE[entry_id] == 3
                        && MONADO_STATE[entry_id] == 4 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                        }
                        if SHIELD_ART_COOLDOWN[entry_id] > 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                        } 
                        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                    }
                }
                frame(Frame=10)
                if(is_excute){
                    rust{
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
                frame(Frame=14)
                if(is_excute){
                    rust{
                        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                    }
                }
                frame(Frame=58)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            });
        }
//Shield Left Sound
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "appeal_hi_l",
    animcmd = "sound_appealhil")]
    pub fn Monado_Ganon_Shield_Left_Sound(fighter: &mut L2CFighterCommon) 
    {
        acmd!({
            frame(Frame=2)
            if(is_excute){
                sv_animcmd::PLAY_SE(hash40("vc_ganon_appeal_h01"));
            }
        });
    }
//Right Taunt
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "appeal_s_r",
    animcmd = "game_appealsr")]
    pub fn Monado_Ganon_Right_Taunt(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                        if MONADO_STATE[entry_id] == 1
                        && MONADO_STATE[entry_id] == 2
                        && MONADO_STATE[entry_id] == 3
                        && MONADO_STATE[entry_id] == 4 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                        }
                        if SMASH_ART_COOLDOWN[entry_id] > 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                        } 
                        if BUSTER_ART_COOLDOWN[entry_id] > 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                        } 
                        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                    }
                }
                frame(Frame=10)
                if(is_excute){
                    rust{
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
                frame(Frame=14)
                if(is_excute){
                    rust{
                        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                    }
                }
                frame(Frame=65)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
                frame(Frame=67)
                if(is_excute){
                    rust{
                        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                    }
                }
                frame(Frame=77)
                if(is_excute){
                    rust{
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
                frame(Frame=81)
                if(is_excute){
                    rust{
                        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                    }
                }
                frame(Frame=114)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            });
        }
//Left Taunt
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "appeal_s_l",
    animcmd = "game_appealsl")]
    pub fn Monado_Ganon_Left_Taunt(fighter: &mut L2CFighterCommon) 
    {
        acmd!({
            if(is_excute){
                rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 1
                    && MONADO_STATE[entry_id] == 2
                    && MONADO_STATE[entry_id] == 3
                    && MONADO_STATE[entry_id] == 4 {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                    if SMASH_ART_COOLDOWN[entry_id] > 0 {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    } 
                    if BUSTER_ART_COOLDOWN[entry_id] > 0 {
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    } 
                    HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                }
            }
            frame(Frame=10)
            if(is_excute){
                rust{
                    CancelModule::enable_cancel(module_accessor);
                }
            }
            frame(Frame=14)
            if(is_excute){
                rust{
                    HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                }
            }
            frame(Frame=65)
            if(is_excute){
                rust{
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                }
            }
            frame(Frame=67)
            if(is_excute){
                rust{
                    HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                }
            }
            frame(Frame=77)
            if(is_excute){
                rust{
                    CancelModule::enable_cancel(module_accessor);
                }
            }
            frame(Frame=81)
            if(is_excute){
                rust{
                    HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                }
            }
            frame(Frame=114)
            if(is_excute){
                rust{
                    StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                }
            }
        });
    }
//Left Taunt Sound
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "appeal_s_l",
    animcmd = "sound_appealsl")]
    pub fn Monado_Ganon_Left_Taunt_Sound(fighter: &mut L2CFighterCommon) 
    {
        acmd!({
            frame(Frame=2)
            if(is_excute){
                sv_animcmd::PLAY_SE(hash40("vc_ganon_appeal_s01"));
            }
            frame(Frame=68)
            if(is_excute){
                sv_animcmd::PLAY_SE(hash40("vc_ganon_appeal_s01"));
            }
        });
    }
//Right Taunt Sound
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "appeal_s_r",
    animcmd = "sound_appealsr")]
    pub fn Monado_Ganon_Right_Taunt_Sound(fighter: &mut L2CFighterCommon) 
    {
        acmd!({
            frame(Frame=2)
            if(is_excute){
                sv_animcmd::PLAY_SE(hash40("vc_ganon_appeal_s01"));
            }
        });
    }
//Accelerate Left
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "appeal_lw_l",
    animcmd = "game_appeallwl")]
    pub fn Monado_Ganon_Accelerate_Left(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                frame(Frame=1)
                if(is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                        if MONADO_STATE[entry_id] == 1
                        && MONADO_STATE[entry_id] == 2
                        && MONADO_STATE[entry_id] == 3
                        && MONADO_STATE[entry_id] == 4 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                        }
                        if ACCELERATE_ART_COOLDOWN[entry_id] > 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                        }
                        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                    }
                }
                frame(Frame=10)
                if(is_excute){
                    rust{
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
                frame(Frame=14)
                if(is_excute){
                    rust{
                        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                    }
                }
                frame(Frame=58)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            });
    }
//Accelerate Left Sound
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "appeal_lw_l",
    animcmd = "sound_appeallwl")]
    pub fn Monado_Ganon_Accelerate_Left_Sound(fighter: &mut L2CFighterCommon) 
    {
        acmd!({
            frame(Frame=2)
            if(is_excute){
                sv_animcmd::PLAY_SE(hash40("vc_ganon_appeal_l01"));
            }
        });
    }
//Accelerate Right
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "appeal_lw_r",
    animcmd = "game_appeallwr")]
    pub fn Monado_Ganon_Accelerate_Right_Sound(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                frame(Frame=1)
                if(is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                        if MONADO_STATE[entry_id] == 1
                        && MONADO_STATE[entry_id] == 2
                        && MONADO_STATE[entry_id] == 3
                        && MONADO_STATE[entry_id] == 4 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                        }
                        if ACCELERATE_ART_COOLDOWN[entry_id] > 0 {
                            StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                        }
                        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_XLU), 0);
                    }
                }
                frame(Frame=10)
                if(is_excute){
                    rust{
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
                frame(Frame=14)
                if(is_excute){
                    rust{
                        HitModule::set_whole(module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
                    }
                }
                frame(Frame=58)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            });
    }
//Accelerate Right Sound
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "appeal_lw_r",
    animcmd = "sound_appeallwr")]
    pub fn Monado_Ganon_Accelerate_Right(fighter: &mut L2CFighterCommon) 
    {
        acmd!({
            frame(Frame=2)
            if(is_excute){
                sv_animcmd::PLAY_SE(hash40("vc_ganon_appeal_l01"));
            }
        });
    }
//Jab
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_11",
    animcmd = "game_attack11")]
    pub fn Monado_Ganon_Jab(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                FT_MOTION_RATE(FSM=0.8)
                frame(Frame=8)
                FT_MOTION_RATE(FSM=1)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=74, FKB=0, BKB=41, Size=4.4, X=0.0, Y=12.0, Z=11.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=74, FKB=0, BKB=41, Size=5.0, X=0.0, Y=12.0, Z=19.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=11.0, Angle=361, KBG=74, FKB=0, BKB=41, Size=3.5, X=0.0, Y=12.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_PUNCH)
                }
                wait(Frames=2)
                if(is_excute){
                AttackModule::clear_all()
                }
                FT_MOTION_RATE(FSM=0.9)
            });
        }
//Dash Attack
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_dash",
    animcmd = "game_attackdash")]
    pub fn Monado_Ganon_Dash_Attack(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                frame(Frame=10)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=70, KBG=85, FKB=0, BKB=50, Size=7.0, X=0.0, Y=9.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_BODY)
                }
                wait(Frames=3)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=11.0, Angle=80, KBG=60, FKB=0, BKB=45, Size=4.0, X=0.0, Y=9.0, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_BODY)
                ATK_SET_SHIELD_SETOFF_MUL(ID=0, ShieldstunMul=1.3)
                }
                wait(Frames=7)
                if(is_excute){
                AttackModule::clear_all()
                }
            });
        }
//Forward Tilt
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_s3_s",
    animcmd = "game_attacks3")]
    pub fn Monado_Ganon_Forward_Tilt(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                frame(Frame=10)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=13.0, Angle=22, KBG=82, FKB=0, BKB=31, Size=4.8, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=13.0, Angle=22, KBG=82, FKB=0, BKB=31, Size=5.0, X=-0.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=14.0, Angle=22, KBG=82, FKB=0, BKB=31, Size=5.5, X=5.3, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                }
                wait(Frames=3)
                if(is_excute){
                AttackModule::clear_all()
                }
            });
        }
//Up Tilt
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_hi3",
    animcmd = "game_attackhi3")]
    pub fn Monado_Ganon_Up_Tilt(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            if(is_excute){
                rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
            }
            frame(Frame=15)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=100, KBG=85, FKB=120, BKB=0, Size=5.0, X=0.0, Y=3.0, Z=6.0, X2=0.0, Y2=6.0, Z2=6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            }
            frame(Frame=16)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=22)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("footl"), Damage=16.0, Angle=85, KBG=65, FKB=0, BKB=75, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=14.0, Angle=85, KBG=65, FKB=0, BKB=75, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("legl"), Damage=12.0, Angle=85, KBG=65, FKB=0, BKB=75, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            }
            wait(Frames=3)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=58)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
        });
    }
//Down Tilt
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_lw3",
    animcmd = "game_attacklw3")]
    pub fn Monado_Ganon_Down_Tilt(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                frame(Frame=10)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("legr"), Damage=14.0, Angle=60, KBG=94, FKB=0, BKB=30, Size=3.0, X=0.0, Y=0.0, Z=3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.35, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=14.0, Angle=70, KBG=94, FKB=0, BKB=30, Size=4.8, X=5.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.35, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=14.0, Angle=80, KBG=94, FKB=0, BKB=30, Size=4.8, X=8.5, Y=-0.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.35, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                wait(Frames=3)
                if(is_excute){
                AttackModule::clear_all()
                }
            });
        }
//Forward Smash
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_s4_s",
    animcmd = "game_attacks4")]
    pub fn Monado_Ganon_Forward_Smash(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                        if MONADO_STATE[entry_id] == 4 {
                            MotionModule::set_rate(module_accessor, 1.2);
                        }
                    }
                }
                frame(Frame=15)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
                }
                frame(Frame=29)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=24.0, Angle=40, KBG=75, FKB=0, BKB=61, Size=4.5, X=0.0, Y=4.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=24.0, Angle=40, KBG=75, FKB=0, BKB=61, Size=4.0, X=0.0, Y=12.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                }
                frame(Frame=30)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=24.0, Angle=40, KBG=75, FKB=0, BKB=61, Size=5.0, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=24.0, Angle=40, KBG=75, FKB=0, BKB=61, Size=4.5, X=0.0, Y=14.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("haver"), Damage=24.0, Angle=40, KBG=75, FKB=0, BKB=61, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                }
                wait(Frames=2)
                if(is_excute){
                AttackModule::clear_all()
                }
            });
        }
//Up Smash
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_hi4",
    animcmd = "game_attackhi4")]
    pub fn Monado_Ganon_Up_Smash(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                        if MONADO_STATE[entry_id] == 4 {
                            MotionModule::set_rate(module_accessor, 1.2);
                        }
                    }
                }
                frame(Frame=10)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
                }
                frame(Frame=20)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("haver"), Damage=24.0, Angle=85, KBG=71, FKB=0, BKB=40, Size=5.0, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("haver"), Damage=24.0, Angle=78, KBG=71, FKB=0, BKB=40, Size=4.5, X=0.0, Y=14.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("armr"), Damage=21.0, Angle=75, KBG=75, FKB=0, BKB=40, Size=4.0, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                }
                wait(Frames=6)
                if(is_excute){
                AttackModule::clear_all()
                }
            });
        }
//Down Smash
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_lw4",
    animcmd = "game_attacklw4")]
    pub fn Monado_Ganon_Down_Smash(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                        ArticleModule::remove_exist(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, smash::app::ArticleOperationTarget(0));
                        ArticleModule::generate_article(module_accessor, *FIGHTER_GANON_GENERATE_ARTICLE_SWORD as i32, false, 0);
                        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                        if MONADO_STATE[entry_id] == 4 {
                            MotionModule::set_rate(module_accessor, 1.2);
                        }
                    }
                }
                frame(Frame=5)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
                }
                frame(Frame=15)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=5.0, Angle=160, KBG=90, FKB=90, BKB=0, Size=3.0, X=0.0, Y=4.0, Z=11.0, X2=0.0, Y2=4.0, Z2=6.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=175, KBG=92, FKB=95, BKB=0, Size=3.0, X=0.0, Y=4.0, Z=19.0, X2=0.0, Y2=4.0, Z2=6.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=5.0, Angle=147, KBG=90, FKB=90, BKB=0, Size=4.0, X=0.0, Y=4.0, Z=11.0, X2=0.0, Y2=4.0, Z2=6.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=5.0, Angle=150, KBG=92, FKB=95, BKB=0, Size=4.0, X=0.0, Y=4.0, Z=19.0, X2=0.0, Y2=4.0, Z2=6.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_SWORD)
                }
                wait(Frames=4)
                if(is_excute){
                AttackModule::clear_all()
                JostleModule::set_status(false)
                }
                frame(Frame=35)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=35, KBG=81, FKB=0, BKB=61, Size=5.0, X=0.0, Y=4.8, Z=-21.0, X2=0.0, Y2=4.8, Z2=-6.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                }
                wait(Frames=4)
                if(is_excute){
                AttackModule::clear_all()
                JostleModule::set_status(true)
                }
            });
        }
//Nair
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_air_n",
    animcmd = "game_attackairn")]
    pub fn Monado_Ganon_Nair(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                frame(Frame=4)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=7)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=7.0, Angle=55, KBG=30, FKB=0, BKB=20, Size=5.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=70, KBG=30, FKB=0, BKB=20, Size=5.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=55, KBG=110, FKB=0, BKB=50, Size=4.3, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=7.0, Angle=100, KBG=30, FKB=0, BKB=20, Size=4.3, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                }
                wait(Frames=2)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("hip"), Damage=5.25, Angle=55, KBG=30, FKB=0, BKB=20, Size=5.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=5.25, Angle=70, KBG=30, FKB=0, BKB=20, Size=5.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=5.25, Angle=55, KBG=110, FKB=0, BKB=50, Size=4.3, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=5.25, Angle=100, KBG=30, FKB=0, BKB=20, Size=4.3, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                }
                wait(Frames=4)
                if(is_excute){
                AttackModule::clear_all()
                }
                FT_MOTION_RATE(FSM=0.5)
                frame(Frame=20)
                FT_MOTION_RATE(FSM=1)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=12.0, Angle=361, KBG=106, FKB=0, BKB=25, Size=7.8, X=6.5, Y=0.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=12.0, Angle=361, KBG=106, FKB=0, BKB=25, Size=7.0, X=0.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=12.0, Angle=361, KBG=106, FKB=0, BKB=25, Size=6.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                }
                wait(Frames=2)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("kneel"), Damage=9.0, Angle=361, KBG=106, FKB=0, BKB=25, Size=6.0, X=6.0, Y=0.0, Z=-3.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=9.0, Angle=361, KBG=106, FKB=0, BKB=25, Size=5.3, X=0.0, Y=0.0, Z=-1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                ATTACK(ID=2, Part=0, Bone=hash40("hip"), Damage=9.0, Angle=361, KBG=106, FKB=0, BKB=25, Size=4.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KICK)
                }
                wait(Frames=8)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=41)
                if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
            });
        }
//Fair
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_air_f",
    animcmd = "game_attackairf")]
    pub fn Monado_Ganon_Fair(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                frame(Frame=7)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=14)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("shoulderr"), Damage=17.0, Angle=361, KBG=93, FKB=0, BKB=20, Size=4.0, X=-1.1, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=18.0, Angle=361, KBG=93, FKB=0, BKB=20, Size=5.5, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.1, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PUNCH)
                }
                wait(Frames=6)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=45)
                if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
            });
        }
//Bair
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_air_b",
    animcmd = "game_attackairb")]
    pub fn Monado_Ganon_Bair(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                frame(Frame=7)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=10)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=17.0, Angle=361, KBG=86, FKB=0, BKB=40, Size=4.0, X=0.0, Y=10.4, Z=-10.8, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=18.5, Angle=361, KBG=86, FKB=0, BKB=40, Size=4.5, X=0.0, Y=9.1, Z=-15.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=17.0, Angle=361, KBG=86, FKB=0, BKB=40, Size=3.0, X=0.0, Y=12.6, Z=-7.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                }
                wait(Frames=3)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=22)
                if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
            });
        }
//Uair
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_air_hi",
    animcmd = "game_attackairhi")]
    pub fn Monado_Ganon_Uair(fighter: &mut L2CFighterCommon)
    {
        acmd!({
                if(is_excute){
                    rust{
                        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                        if MONADO_STATE[entry_id] == 4 {
                            MotionModule::set_rate(module_accessor, 1.2);
                        }
                    }
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=6)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=13.0, Angle=361, KBG=100, FKB=0, BKB=35, Size=4.8, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=12.0, Angle=361, KBG=100, FKB=0, BKB=35, Size=5.8, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                wait(Frames=3)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=12.0, Angle=30, KBG=80, FKB=0, BKB=30, Size=4.8, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=10.0, Angle=30, KBG=80, FKB=0, BKB=30, Size=5.8, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                wait(Frames=3)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("legl"), Damage=8.0, Angle=0, KBG=70, FKB=0, BKB=20, Size=4.8, X=3.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("kneel"), Damage=6.0, Angle=0, KBG=70, FKB=0, BKB=20, Size=5.8, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                wait(Frames=5)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=25)
                if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=57)
                if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
                }
        });
    }
//Dair
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "attack_air_lw",
    animcmd = "game_attackairlw")]
    pub fn Monado_Ganon_Dair(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
            }
                frame(Frame=4)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
                frame(Frame=16)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=19.0, Angle=270, KBG=100, FKB=0, BKB=20, Size=7.0, X=0.0, Y=1.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_KICK)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=17.0, Angle=361, KBG=100, FKB=0, BKB=20, Size=6.0, X=0.0, Y=10.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_KICK)
                }
                wait(Frames=3)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=32)
                if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
                }
            });
        }
//Grab
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "catch",
    animcmd = "game_catch")]
    pub fn Monado_Ganon_Grab(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                frame(Frame=7)
                if(is_excute){
                GrabModule::set_rebound(CanCatchRebound=true)
                }
                frame(Frame=8)
                if(is_excute){
                CATCH(ID=0, Bone=hash40("top"), Size=4.0, X=0.0, Y=9.0, Z=4.0, X2=0.0, Y2=9.0, Z2=10.0, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
                CATCH(ID=1, Bone=hash40("top"), Size=2.0, X=0.0, Y=9.0, Z=2.0, X2=0.0, Y2=9.0, Z2=12.0, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
                }
                wait(Frames=3)
                if(is_excute){
                sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
                WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
                GrabModule::set_rebound(CanCatchRebound=false)
                }
            });
        }
//Dash Grab
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "catch_dash",
    animcmd = "game_catchdash")]
    pub fn Monado_Ganon_Dash_Grab(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                frame(Frame=10)
                if(is_excute){
                GrabModule::set_rebound(CanCatchRebound=true)
                }
                frame(Frame=11)
                if(is_excute){
                CATCH(ID=0, Bone=hash40("top"), Size=3.2, X=0.0, Y=9.0, Z=4.0, X2=0.0, Y2=9.0, Z2=11.8, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
                CATCH(ID=1, Bone=hash40("top"), Size=1.6, X=0.0, Y=9.0, Z=2.4, X2=0.0, Y2=9.0, Z2=13.4, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
                }
                wait(Frames=3)
                if(is_excute){
                sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
                WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
                GrabModule::set_rebound(CanCatchRebound=false)
                }
            });
        }
//Pivot Grab
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "catch_turn",
    animcmd = "game_catchturn")]
    pub fn Monado_Ganon_Pivot_Grab(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                frame(Frame=11)
                if(is_excute){
                GrabModule::set_rebound(CanCatchRebound=true)
                }
                frame(Frame=12)
                if(is_excute){
                CATCH(ID=0, Bone=hash40("top"), Size=4.0, X=0.0, Y=9.0, Z=-4.0, X2=0.0, Y2=9.0, Z2=-15.6, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
                CATCH(ID=1, Bone=hash40("top"), Size=2.0, X=0.0, Y=9.0, Z=-2.0, X2=0.0, Y2=9.0, Z2=-17.6, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
                }
                wait(Frames=3)
                if(is_excute){
                sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
                WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
                GrabModule::set_rebound(CanCatchRebound=false)
                }                
            });
        }
//Pummel
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "catch_attack",
    animcmd = "game_catchattack")]
    pub fn Monado_Ganon_Pummel(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                frame(Frame=2)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.6, Angle=80, KBG=100, FKB=30, BKB=0, Size=6.5, X=0.0, Y=10.0, Z=8.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=2.3, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_HEAVY, Type=ATTACK_REGION_KNEE)
                AttackModule::set_catch_only_all(true, false)
                }
                wait(Frames=1)
                if(is_excute){
                AttackModule::clear_all()
                }
            });
        }
//Forward Throw
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "throw_f",
    animcmd = "game_throwf")]
    pub fn Monado_Ganon_F_Throw(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                    ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=8.0, Angle=43, KBG=70, FKB=0, BKB=68, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                    ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                }
                    frame(Frame=11)
                    if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=5.0, Angle=80, KBG=100, FKB=0, BKB=30, Size=5.0, X=4.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=1, Part=0, Bone=hash40("shoulderr"), Damage=5.0, Angle=80, KBG=100, FKB=0, BKB=30, Size=2.4, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=5.0, Angle=80, KBG=100, FKB=0, BKB=30, Size=2.4, X=-0.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                    AttackModule::set_catch_only_all(true, false)
                    CHECK_FINISH_CAMERA(17, 9)
                    }
                    frame(Frame=14)
                    if(is_excute){
                    ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
                    AttackModule::clear_all()
                    }                    
            });
        }
//Back Throw
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "throw_b",
    animcmd = "game_throwb")]
    pub fn Monado_Ganon_B_Throw(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                    ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=5.0, Angle=43, KBG=130, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                    ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                }
                    frame(Frame=11)
                    if(is_excute){
                    REVERSE_LR()
                    }
                    frame(Frame=12)
                    if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=5.0, Angle=70, KBG=130, FKB=0, BKB=30, Size=4.8, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=5.0, Angle=70, KBG=130, FKB=0, BKB=30, Size=4.3, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=2, Part=0, Bone=hash40("legr"), Damage=5.0, Angle=70, KBG=130, FKB=0, BKB=30, Size=3.7, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    AttackModule::set_catch_only_all(true, false)
                    CHECK_FINISH_CAMERA(22, 20)
                    }
                    frame(Frame=14)
                    if(is_excute){
                    ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
                    AttackModule::clear_all()
                    }
            });
        }
//Up Throw
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "throw_hi",
    animcmd = "game_throwhi")]
    pub fn Monado_Ganon_U_Throw(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                    ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=3.0, Angle=90, KBG=105, FKB=0, BKB=70, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                    ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                }
                    frame(Frame=11)
                    if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=10.0, Angle=140, KBG=100, FKB=0, BKB=30, Size=4.3, X=4.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=1, Part=0, Bone=hash40("shoulderr"), Damage=10.0, Angle=140, KBG=100, FKB=0, BKB=30, Size=3.8, X=2.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=2, Part=0, Bone=hash40("shoulderr"), Damage=10.0, Angle=140, KBG=100, FKB=0, BKB=30, Size=3.7, X=-0.8, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.2, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_PUNCH, Type=ATTACK_REGION_PUNCH)
                    AttackModule::set_catch_only_all(true, false)
                    CHECK_FINISH_CAMERA(-1, 28)
                    }
                    frame(Frame=14)
                    if(is_excute){
                    ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
                    AttackModule::clear_all()
                    }
            });
        }
//Down Throw
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "throw_lw",
    animcmd = "game_throwlw")]
    pub fn Monado_Ganon_D_Throw(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                    ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=7.0, Angle=80, KBG=65, FKB=0, BKB=50, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                    ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=3.0, Angle=361, KBG=100, FKB=0, BKB=60, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=0.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_THROW)
                }
                    frame(Frame=22)
                    if(is_excute){
                    CHECK_FINISH_CAMERA(11, 0)
                    }
                    frame(Frame=23)
                    if(is_excute){
                    ATK_HIT_ABS(FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, hash40("throw"), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO))
                    }
            });
        }
//Neutral Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_n",
    animcmd = "game_specialn")]
    pub fn Monado_Ganon_Neutral_Special(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                frame(Frame=11)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_TURN)
                sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
                }
                frame(Frame=68)
                if(is_excute){
                sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
                }
                frame(Frame=70)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("shoulderl"), Damage=30.0, Angle=50, KBG=46, FKB=0, BKB=120, Size=5.0, X=2.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("bust"), Damage=30.0, Angle=50, KBG=46, FKB=0, BKB=120, Size=4.7, X=0.0, Y=1.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=30.0, Angle=50, KBG=46, FKB=0, BKB=120, Size=4.8, X=5.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                }
                frame(Frame=74)
                if(is_excute){
                AttackModule::clear_all()
                }                
            });
        }
//Reverse Neutral Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_n_turn",
    animcmd = "game_specialnturn")]
    pub fn Monado_Ganon_Reverse_Neutral_Special(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                frame(Frame=11)
                if(is_excute){
                REVERSE_LR()
                sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_ALWAYS, 0)
                }
                frame(Frame=65)
                if(is_excute){
                sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
                }
                frame(Frame=70)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("shoulderl"), Damage=37.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=5.0, X=2.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("bust"), Damage=37.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.7, X=0.0, Y=1.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=37.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=4.8, X=5.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                }
                frame(Frame=74)
                if(is_excute){
                AttackModule::clear_all()
                }
            });
        }
//Aerial Neutral Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_air_n",
    animcmd = "game_specialairn")]
    pub fn Monado_Ganon_Aerial_Neutral_Special(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                frame(Frame=11)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_TURN)
                }
                frame(Frame=66)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_PUNCH_DIR_DECIDE)
                WorkModule::set_int(1, FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_PUNCH_AIR_PHASE)
                }
                frame(Frame=70)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("shoulderl"), Damage=38.0, Angle=30, KBG=100, FKB=0, BKB=30, Size=5.0, X=2.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("bust"), Damage=38.0, Angle=30, KBG=100, FKB=0, BKB=30, Size=4.7, X=0.0, Y=1.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=38.0, Angle=30, KBG=100, FKB=0, BKB=30, Size=4.8, X=5.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                }
                frame(Frame=74)
                if(is_excute){
                AttackModule::clear_all()
                WorkModule::set_int(2, FIGHTER_GANON_STATUS_WORK_ID_INT_GANON_PUNCH_AIR_PHASE)
                }
            });
        }
//Aerial Reverse Neutral Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_air_n_turn",
    animcmd = "game_specialairnturn")]
    pub fn Monado_Ganon_Aerial_Reverse_Neutral_Special(fighter: &mut L2CFighterCommon)
    {
            acmd!({
                if(is_excute){
                    rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
                }
                frame(Frame=11)
                if(is_excute){
                REVERSE_LR()
                }
                frame(Frame=70)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("shoulderl"), Damage=40.0, Angle=30, KBG=100, FKB=0, BKB=40, Size=5.0, X=2.4, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("bust"), Damage=40.0, Angle=30, KBG=100, FKB=0, BKB=40, Size=4.7, X=0.0, Y=1.0, Z=1.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=2, Part=0, Bone=hash40("arml"), Damage=40.0, Angle=30, KBG=100, FKB=0, BKB=40, Size=4.8, X=5.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                }
                frame(Frame=74)
                if(is_excute){
                AttackModule::clear_all()
                }
            });
        }
//Grounded Down Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_lw",
    animcmd = "game_speciallw")]
    pub fn Monado_Ganon_Grounded_Down_Special(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            if(is_excute){
                rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
            }
                frame(Frame=8)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=2.0, Angle=340, KBG=60, FKB=150, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=1.5, Angle=340, KBG=60, FKB=150, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=2, Part=0, Bone=hash40("legr"), Damage=1.0, Angle=340, KBG=60, FKB=150, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                frame(Frame=9)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=10)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=2.0, Angle=340, KBG=60, FKB=150, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=1.5, Angle=340, KBG=60, FKB=150, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=2, Part=0, Bone=hash40("legr"), Damage=1.0, Angle=340, KBG=60, FKB=150, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                frame(Frame=11)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=12)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=2.0, Angle=340, KBG=60, FKB=150, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=1.5, Angle=340, KBG=60, FKB=150, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=2, Part=0, Bone=hash40("legr"), Damage=1.0, Angle=340, KBG=60, FKB=150, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                frame(Frame=13)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=14)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=2.0, Angle=340, KBG=60, FKB=150, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_elec"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=1.5, Angle=340, KBG=60, FKB=150, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=2, Part=0, Bone=hash40("legr"), Damage=1.0, Angle=340, KBG=60, FKB=150, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                frame(Frame=15)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=16)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=12.0, Angle=270, KBG=70, FKB=0, BKB=35, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=3, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_bury"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=11.0, Angle=361, KBG=85, FKB=0, BKB=55, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=2, Part=0, Bone=hash40("legr"), Damage=10.0, Angle=361, KBG=85, FKB=0, BKB=55, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                frame(Frame=18)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=58)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
        });
    }
//Aerial Down Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_air_lw",
    animcmd = "game_specialairlw")]
    pub fn Monado_Ganon_Aerial_Down_Special(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            if(is_excute){
                rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
            }
                frame(Frame=18)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=4.0, Angle=270, KBG=80, FKB=140, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=2.5, Angle=270, KBG=80, FKB=140, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=2.5, Angle=270, KBG=80, FKB=140, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                frame(Frame=21)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=22)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=4.0, Angle=270, KBG=80, FKB=140, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=2.5, Angle=270, KBG=80, FKB=140, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=2.5, Angle=270, KBG=80, FKB=140, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                frame(Frame=25)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=26)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=4.0, Angle=270, KBG=80, FKB=140, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=2.5, Angle=270, KBG=80, FKB=140, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=2.5, Angle=270, KBG=80, FKB=140, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                frame(Frame=29)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=30)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=4.0, Angle=270, KBG=80, FKB=140, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=2.5, Angle=270, KBG=80, FKB=140, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=2.5, Angle=270, KBG=80, FKB=140, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                frame(Frame=33)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=34)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=4.0, Angle=270, KBG=80, FKB=140, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=2.5, Angle=270, KBG=80, FKB=140, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=2.5, Angle=270, KBG=80, FKB=140, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                frame(Frame=37)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=40)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=4.0, Angle=270, KBG=80, FKB=140, BKB=0, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=1, Part=0, Bone=hash40("legr"), Damage=2.5, Angle=270, KBG=80, FKB=140, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                    ATTACK(ID=2, Part=0, Bone=hash40("kneer"), Damage=2.5, Angle=270, KBG=80, FKB=140, BKB=0, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
                }
                frame(Frame=43)
                if(is_excute){
                AttackModule::clear_all()
                }
                frame(Frame=58)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                    }
                }
        });
    }
//Grounded Up Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_hi",
    animcmd = "game_specialhi")]
    pub fn Monado_Ganon_Grounded_Up_Special(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            if(is_excute){
                rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
            }
            frame(Frame=12)
            if(is_excute){
            rust {
                let speedVec = Vector3f{x: 0.65, y: 0.0, z: 0.0};
                KineticModule::add_speed(module_accessor, &speedVec);
              }
            WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
            }
            frame(Frame=14)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=75, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=12.0, X2=0.0, Y2=5.0, Z2=12.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=17)
                if(is_excute){
                    AttackModule::clear_all()
                }
            frame(Frame=18)
            if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS)
            }
            frame(Frame=19)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=75, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=12.0, X2=0.0, Y2=5.0, Z2=12.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=22)
                if(is_excute){
                    AttackModule::clear_all()
                }
            frame(Frame=24)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=75, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=12.0, X2=0.0, Y2=5.0, Z2=12.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=27)
                if(is_excute){
                    AttackModule::clear_all()
                }
            frame(Frame=29)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=75, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=12.0, X2=0.0, Y2=5.0, Z2=12.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
            }
            frame(Frame=32)
                if(is_excute){
                    AttackModule::clear_all()
                }
            frame(Frame=34)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=10.0, Angle=70, KBG=90, FKB=0, BKB=70, Size=9.0, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=10.0, Angle=70, KBG=90, FKB=0, BKB=70, Size=6.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
            }
            wait(Frames=3)
            if(is_excute){
            AttackModule::clear_all()
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
            }
            frame(Frame=46)
            if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE)
            }            
        });
    }
//Aerial Up Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_air_hi",
    animcmd = "game_specialairhi")]
    pub fn Monado_Ganon_Aerial_Up_Special(fighter: &mut L2CFighterCommon)
    {
        acmd!({    
            if(is_excute){
                rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
            } 
                frame(Frame=12)
                if(is_excute){
                rust {
                    let speedVec = Vector3f{x: 0.65, y: 0.0, z: 0.0};
                    KineticModule::add_speed(module_accessor, &speedVec);
                  }
                WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
                } 
                frame(Frame=14)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=75, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=12.0, X2=0.0, Y2=5.0, Z2=12.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
                }
                frame(Frame=17)
                if(is_excute){
                    AttackModule::clear_all()
                }
                frame(Frame=18)
                if(is_excute){
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS)
                }
                frame(Frame=19)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=75, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=12.0, X2=0.0, Y2=5.0, Z2=12.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
                }
                frame(Frame=22)
                if(is_excute){
                    AttackModule::clear_all()
                }
                frame(Frame=24)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=75, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=12.0, X2=0.0, Y2=5.0, Z2=12.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
                }
                frame(Frame=27)
                if(is_excute){
                    AttackModule::clear_all()
                }
                frame(Frame=29)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=3.0, Angle=367, KBG=75, FKB=200, BKB=0, Size=7.0, X=0.0, Y=12.0, Z=12.0, X2=0.0, Y2=5.0, Z2=12.0, Hitlag=1.0, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
                }
                frame(Frame=32)
                if(is_excute){
                    AttackModule::clear_all()
                }
                frame(Frame=34)
                if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("armr"), Damage=10.0, Angle=70, KBG=90, FKB=0, BKB=70, Size=9.0, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=10.0, Angle=70, KBG=90, FKB=0, BKB=70, Size=6.0, X=-1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_PUNCH)
                }
                wait(Frames=3)
                if(is_excute){
                AttackModule::clear_all()
                sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
                }
                frame(Frame=46)
                if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_SPECIAL_HI_FLAG_IS_CHECK_DIVE)
                }
        });
    }
//Grounded Side Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_s_start",
    animcmd = "game_specialsstart")]
    pub fn Monado_Ganon_Grounded_Side_Special(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            if(is_excute){
                rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
            }
            frame(Frame=1)
                if(is_excute){
                    rust{
                        let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                        let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                        EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.2, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.2, true, 0, 0, 0, 0, 0, true, true);
                    }
                    LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
                }
            frame(Frame=5)
            if(is_excute){
                rust{
                    let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                    let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.3, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.4, true, 0, 0, 0, 0, 0, true, true);
                }
                LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
                FT_MOTION_RATE(FSM=0.5)
                HitModule::set_status_joint(smash::phx::Hash40::new("footr"), smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                HitModule::set_status_joint(smash::phx::Hash40::new("kneer"), smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                HitModule::set_status_joint(smash::phx::Hash40::new("legr"), smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                HitModule::set_status_joint(smash::phx::Hash40::new("hip"), smash::app::HitStatus(*HIT_STATUS_XLU), 0)
            }
            frame(Frame=9)
                if(is_excute){
                    rust{
                        let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                        let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                        EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.4, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.6, true, 0, 0, 0, 0, 0, true, true);
                    }
                    LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
                }
            frame(Frame=11)
            if(is_excute){
                FT_MOTION_RATE(FSM=5.0)
            }
            frame(Frame=13)
            if(is_excute){
                rust{
                    let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                    let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.5, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.8, true, 0, 0, 0, 0, 0, true, true);
                }
                LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
                FT_MOTION_RATE(FSM=1.0)
            }
            frame(Frame=15)
            if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("footr"), Damage=12.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("kneer"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("legr"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            }
            wait(Frames=1)
            if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK)
            }
            frame(Frame=17)
                if(is_excute){
                    rust{
                        let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                        let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                        EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.5, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.8, true, 0, 0, 0, 0, 0, true, true);
                    }
                    LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
                }
            frame(Frame=21)
                if(is_excute){
                    rust{
                        let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                        let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                        EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.5, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.8, true, 0, 0, 0, 0, 0, true, true);
                    }
                    LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
                }
            frame(Frame=25)
                if(is_excute){
                    rust{
                        let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                        let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                        EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.5, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.8, true, 0, 0, 0, 0, 0, true, true);
                    }
                    LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
                }
            frame(Frame=29)
                if(is_excute){
                    rust{
                        let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                        let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                        EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.5, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.8, true, 0, 0, 0, 0, 0, true, true);
                    }
                    LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
                }
            frame(Frame=33)
                if(is_excute){
                    rust{
                        let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                        let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                        EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.5, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.8, true, 0, 0, 0, 0, 0, true, true);
                    }
                    LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
                }
            frame(Frame=37)
                if(is_excute){
                    rust{
                        let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                        let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                        EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.5, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.8, true, 0, 0, 0, 0, 0, true, true);
                    }
                    LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
                }
            frame(Frame=40)
            if(is_excute){
                AttackModule::clear_all()
                HitModule::set_status_all(smash::app::HitStatus(*HIT_STATUS_NORMAL), 0)
            }
            frame(Frame=71)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
                    }
                }
        });
    }
//Aerial Side Special
#[acmd_func(
    battle_object_category = BATTLE_OBJECT_CATEGORY_FIGHTER,
    battle_object_kind = FIGHTER_KIND_GANON,
    animation = "special_air_s_start",
    animcmd = "game_specialairsstart")]
    pub fn Monado_Ganon_Aerial_Side_Special(fighter: &mut L2CFighterCommon)
    {
        acmd!({
            if(is_excute){
                rust{
                    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
                    if MONADO_STATE[entry_id] == 4 {
                        MotionModule::set_rate(module_accessor, 1.2);
                    }
                }
            }
            frame(Frame=1)
            if(is_excute){
                rust{
                    let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                    let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.2, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.2, true, 0, 0, 0, 0, 0, true, true);
                }
                LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
            }
            frame(Frame=5)
            if(is_excute){
                rust{
                    let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                    let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.3, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.4, true, 0, 0, 0, 0, 0, true, true);
                }
                LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
            }
            frame(Frame=9)
            if(is_excute){
                rust{
                    let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                    let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.4, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.6, true, 0, 0, 0, 0, 0, true, true);
                }
                LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
            }
            frame(Frame=13)
            if(is_excute){
                rust{
                    let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                    let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.5, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.8, true, 0, 0, 0, 0, 0, true, true);
                }
                LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
            }
            frame(Frame=15)
            if(is_excute){
                HitModule::set_status_joint(smash::phx::Hash40::new("footr"), smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                HitModule::set_status_joint(smash::phx::Hash40::new("kneer"), smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                HitModule::set_status_joint(smash::phx::Hash40::new("legr"), smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                HitModule::set_status_joint(smash::phx::Hash40::new("hip"), smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                HitModule::set_status_joint(smash::phx::Hash40::new("footl"), smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                HitModule::set_status_joint(smash::phx::Hash40::new("kneel"), smash::app::HitStatus(*HIT_STATUS_XLU), 0)
                HitModule::set_status_joint(smash::phx::Hash40::new("legl"), smash::app::HitStatus(*HIT_STATUS_XLU), 0)
            WorkModule::on_flag(Flag=FIGHTER_GANON_STATUS_WORK_ID_FLAG_GANON_KICK_WALL_CHECK)
            ATTACK(ID=0, Part=0, Bone=hash40("footl"), Damage=12.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("footr"), Damage=12.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=11.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=4, Part=0, Bone=hash40("legl"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=5, Part=0, Bone=hash40("legr"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            }
            frame(Frame=17)
            if(is_excute){
                rust{
                    let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                    let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.5, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.8, true, 0, 0, 0, 0, 0, true, true);
                }
                LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
            }
            frame(Frame=21)
            if(is_excute){
                rust{
                    let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                    let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.5, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.8, true, 0, 0, 0, 0, 0, true, true);
                }
                LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
            }
            frame(Frame=25)
            if(is_excute){
                rust{
                    let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                    let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.5, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.8, true, 0, 0, 0, 0, 0, true, true);
                }
            LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
            ATTACK(ID=0, Part=0, Bone=hash40("footl"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=1, Part=0, Bone=hash40("footr"), Damage=10.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=5.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=2, Part=0, Bone=hash40("kneel"), Damage=9.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=3, Part=0, Bone=hash40("kneer"), Damage=9.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=4.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=4, Part=0, Bone=hash40("legl"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            ATTACK(ID=5, Part=0, Bone=hash40("legr"), Damage=8.0, Angle=361, KBG=100, FKB=0, BKB=50, Size=3.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_purple"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            }
            frame(Frame=29)
            if(is_excute){
                rust{
                    let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                    let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.5, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.8, true, 0, 0, 0, 0, 0, true, true);
                }
                LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
            }
            frame(Frame=33)
            if(is_excute){
                rust{
                    let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                    let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("sys_attack_elec"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.5, true, 0, 0, 0, 0, 0, true, true);
                    EffectModule::req_follow(module_accessor, smash::phx::Hash40::new("ganon_rekkikyaku"), smash::phx::Hash40::new("footr"), &pos, &pos, 0.8, true, 0, 0, 0, 0, 0, true, true);
                }
                LAST_EFFECT_SET_COLOR(0.0, 0.0, 2.0);
            }
            frame(Frame=37)
            if(is_excute){
            AttackModule::clear_all()
            }
            frame(Frame=73)
                if(is_excute){
                    rust{
                        StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
                    }
                }
        });
    }
pub fn once_per_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = StatusModule::status_kind(module_accessor);
        let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        let pos = Vector3f{x: 0.0, y: 0.0, z: 0.0};
        let zeros = Vector3f{ x: 0.0, y: 0.0, z: 0.0};
        let fighter_kinetic_energy_motion = mem::transmute::<u64, &mut app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));  
        if kind == *FIGHTER_KIND_GANON {
            if sv_information::is_ready_go() == false {
                MONADO_STATE[entry_id] = 0;
                SHIELD_ART_COOLDOWN[entry_id] = 0;
                BUSTER_ART_COOLDOWN[entry_id] = 0;
                SMASH_ART_COOLDOWN[entry_id] = 0;
                ACCELERATE_ART_COOLDOWN[entry_id] = 0;
            }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DEAD {
                MONADO_STATE[entry_id] = 0;
                SHIELD_ART_COOLDOWN[entry_id] = 0;
                BUSTER_ART_COOLDOWN[entry_id] = 0;
                SMASH_ART_COOLDOWN[entry_id] = 0;
                ACCELERATE_ART_COOLDOWN[entry_id] = 0;
            }
            if motion_kind == hash40("special_air_s_start") 
            && situation_kind == *SITUATION_KIND_GROUND {
                {
                  LANDING[entry_id] = true;
                }
                if LANDING[entry_id] == true {
                  StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, true);
                }
              }
              else {
                LANDING[entry_id] = false;
              }
            if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_APPEAL 
            && MotionModule::frame(module_accessor) < 66.0
            && ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) {
                  MotionModule::set_frame(module_accessor, 67.0, true);
            }
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI)
            && MotionModule::frame(module_accessor) == 0.0
            && MONADO_STATE[entry_id] != 1 // I added this check to make sure that you can't use taunt again to reset the Art timer.
            && SHIELD_ART_COOLDOWN[entry_id] == 0 
            && can_monado_arts(module_accessor) { // Added this as well to make sure you can't activate an art that's on cooldown.
                if MONADO_STATE[entry_id] == 2 {
                    BUSTER_ART_TIMER[entry_id] = 0; // This exists to check if a Monado Art is active, and if it is, sets it to 0.
                }
                if MONADO_STATE[entry_id] == 3 {
                    SMASH_ART_TIMER[entry_id] = 0; // This is added to every activation check.
                }
                if MONADO_STATE[entry_id] == 4 {
                    ACCELERATE_ART_TIMER[entry_id] = 0;
                }
                MONADO_STATE[entry_id] = 1;
                SHIELD_ART_TIMER[entry_id] = 360; // Before, you were setting the Art timers as long as the MONADO_STATE was a certain number. However, because this is once_per_fighter_frame, that means the timer will be set every frame, so it never gets to count down.
                println!("Shield Active");
            }
            if SHIELD_ART_TIMER[entry_id] > 0 {
                SHIELD_ART_TIMER[entry_id] = SHIELD_ART_TIMER[entry_id] - 1;
            }
            if MONADO_STATE[entry_id] == 1 {
                AttackModule::set_power_up(module_accessor, 0.5);
                DamageModule::set_damage_mul(module_accessor, 0.5);
                DamageModule::set_reaction_mul(module_accessor, 0.6);
                FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, 0.67);
                SHIELD_GFX_COUNTER[entry_id] += 1;
            }
            if SHIELD_GFX_COUNTER[entry_id] >= 6 {
            EffectModule::req_follow(module_accessor, Hash40{hash: hash40("sys_hit_purple")}, Hash40{hash: hash40("handr")}, &pos, &zeros, 0.3, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::req_follow(module_accessor, Hash40{hash: hash40("sys_hit_purple")}, Hash40{hash: hash40("handl")}, &pos, &zeros, 0.3, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            SHIELD_GFX_COUNTER[entry_id] = 0;
            }
            if SHIELD_ART_TIMER[entry_id] <= 0 
            && MONADO_STATE[entry_id] == 1 {
                MONADO_STATE[entry_id] = 0;
                SHIELD_ART_COOLDOWN[entry_id] = 1080;
            }
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) // The previous comments I made have all been applied to the other three arts.
            && MotionModule::frame(module_accessor) == 67.0
            && MONADO_STATE[entry_id] != 2
            && BUSTER_ART_COOLDOWN[entry_id] == 0
            && can_monado_arts(module_accessor) {
                if MONADO_STATE[entry_id] == 1 {
                    SHIELD_ART_TIMER[entry_id] = 0;
                }
                if MONADO_STATE[entry_id] == 3 {
                    SMASH_ART_TIMER[entry_id] = 0;
                }
                if MONADO_STATE[entry_id] == 4 {
                    ACCELERATE_ART_TIMER[entry_id] = 0;
                }
                MONADO_STATE[entry_id] = 2;
                BUSTER_ART_TIMER[entry_id] = 600;
                println!("Buster Active");
            }
            if BUSTER_ART_TIMER[entry_id] > 0 {
                BUSTER_ART_TIMER[entry_id] = BUSTER_ART_TIMER[entry_id] - 1;
            }
            if MONADO_STATE[entry_id] == 2 {
                AttackModule::set_power_up(module_accessor, 1.4);
                AttackModule::set_reaction_mul(module_accessor, 0.63);
                DamageModule::set_damage_mul(module_accessor, 1.3);
                BUSTER_GFX_COUNTER[entry_id] += 1;
            }
            if BUSTER_GFX_COUNTER[entry_id] >= 10 {
            EffectModule::req_follow(module_accessor, Hash40{hash: hash40("sys_hit_elec")}, Hash40{hash: hash40("handr")}, &pos, &zeros, 0.3, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::req_follow(module_accessor, Hash40{hash: hash40("sys_hit_elec")}, Hash40{hash: hash40("handl")}, &pos, &zeros, 0.3, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            BUSTER_GFX_COUNTER[entry_id] = 0;
            }
            if BUSTER_ART_TIMER[entry_id] <= 0 
            && MONADO_STATE[entry_id] == 2 {
                MONADO_STATE[entry_id] = 0;
                BUSTER_ART_COOLDOWN[entry_id] = 840;
            }
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)
            && MotionModule::frame(module_accessor) == 0.0
            && MONADO_STATE[entry_id] != 3
            && SMASH_ART_COOLDOWN[entry_id] == 0 
            && can_monado_arts(module_accessor) {
                if MONADO_STATE[entry_id] == 2 {
                    BUSTER_ART_TIMER[entry_id] = 0;
                }
                if MONADO_STATE[entry_id] == 1 {
                    SHIELD_ART_TIMER[entry_id] = 0;
                }
                if MONADO_STATE[entry_id] == 4 {
                    ACCELERATE_ART_TIMER[entry_id] = 0;
                }
                MONADO_STATE[entry_id] = 3;
                SMASH_ART_TIMER[entry_id] = 480;
                println!("Smash Active");
            }
            if SMASH_ART_TIMER[entry_id] > 0 {
                SMASH_ART_TIMER[entry_id] = SMASH_ART_TIMER[entry_id] - 1;
            }
            if MONADO_STATE[entry_id] == 3 {
                AttackModule::set_power_up(module_accessor, 0.3);
                AttackModule::set_reaction_mul(module_accessor, 1.25);
                DamageModule::set_reaction_mul(module_accessor, 1.2);
                SMASH_GFX_COUNTER[entry_id] += 1;
            }
            if SMASH_GFX_COUNTER[entry_id] >= 8 {
            EffectModule::req_follow(module_accessor, Hash40{hash: hash40("sys_hit_fire")}, Hash40{hash: hash40("handr")}, &pos, &zeros, 0.3, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::req_follow(module_accessor, Hash40{hash: hash40("sys_hit_fire")}, Hash40{hash: hash40("handl")}, &pos, &zeros, 0.3, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            SMASH_GFX_COUNTER[entry_id] = 0;
            }
            if SMASH_ART_TIMER[entry_id] <= 0 
            && MONADO_STATE[entry_id] == 3 {
                MONADO_STATE[entry_id] = 0;
                BUSTER_ART_COOLDOWN[entry_id] = 960;
            }
            if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW)
            && MotionModule::frame(module_accessor) == 0.0
            && MONADO_STATE[entry_id] != 4
            && ACCELERATE_ART_COOLDOWN[entry_id] == 0 
            && can_monado_arts(module_accessor) {
                if MONADO_STATE[entry_id] == 2 {
                    BUSTER_ART_TIMER[entry_id] = 0;
                }
                if MONADO_STATE[entry_id] == 1 {
                    SHIELD_ART_TIMER[entry_id] = 0;
                }
                if MONADO_STATE[entry_id] == 3 {
                    SMASH_ART_TIMER[entry_id] = 0;
                }
                MONADO_STATE[entry_id] = 4;
                ACCELERATE_ART_TIMER[entry_id] = 420;
                println!("Accelerate Active");
            }
            if ACCELERATE_ART_TIMER[entry_id] > 0 {
                ACCELERATE_ART_TIMER[entry_id] = ACCELERATE_ART_TIMER[entry_id] - 1;
            }
            if MONADO_STATE[entry_id] == 4 {
                FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, 1.2);
                AttackModule::set_power_up(module_accessor, 0.8);
                DamageModule::set_damage_mul(module_accessor, 1.2);
                ACCELERATE_GFX_COUNTER[entry_id] += 1;
            }
            if ACCELERATE_GFX_COUNTER[entry_id] >= 7 {
            EffectModule::req_follow(module_accessor, Hash40{hash: hash40("sys_hit_ice")}, Hash40{hash: hash40("handr")}, &pos, &zeros, 0.3, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            EffectModule::req_follow(module_accessor, Hash40{hash: hash40("sys_hit_ice")}, Hash40{hash: hash40("handl")}, &pos, &zeros, 0.3, true, *EFFECT_SUB_ATTRIBUTE_NO_JOINT_SCALE as u32 | *EFFECT_SUB_ATTRIBUTE_FOLLOW as u32 | *EFFECT_SUB_ATTRIBUTE_CONCLUDE_STATUS as u32, 0, 0, 0, 0, true, true);
            ACCELERATE_GFX_COUNTER[entry_id] = 0;
            }
            if ACCELERATE_ART_TIMER[entry_id] <= 0 
            && MONADO_STATE[entry_id] == 4 {
                MONADO_STATE[entry_id] = 0;
                ACCELERATE_ART_COOLDOWN[entry_id] = 1020;
            }
            if MONADO_STATE[entry_id] == 0 {
                AttackModule::set_power_up(module_accessor, 1.0);
                AttackModule::set_reaction_mul(module_accessor, 1.0);
                DamageModule::set_damage_mul(module_accessor, 1.0);
                DamageModule::set_reaction_mul(module_accessor, 1.0);
                FighterKineticEnergyMotion::set_speed_mul(fighter_kinetic_energy_motion, 1.0);
                EffectModule::kill_kind(module_accessor, Hash40{hash: hash40("sys_hit_purple")}, false, true);
                EffectModule::kill_kind(module_accessor, Hash40{hash: hash40("sys_hit_elec")}, false, true);
                EffectModule::kill_kind(module_accessor, Hash40{hash: hash40("sys_hit_fire")}, false, true);
                EffectModule::kill_kind(module_accessor, Hash40{hash: hash40("sys_hit_ice")}, false, true);
            }
            if SHIELD_ART_COOLDOWN[entry_id] > 0 {
                SHIELD_ART_COOLDOWN[entry_id] = SHIELD_ART_COOLDOWN[entry_id] - 1; // Also, you forgot to add this to make the art cooldowns work.
            }
            if BUSTER_ART_COOLDOWN[entry_id] > 0 {
                BUSTER_ART_COOLDOWN[entry_id] = BUSTER_ART_COOLDOWN[entry_id] - 1;
            }
            if SMASH_ART_COOLDOWN[entry_id] > 0 {
                SMASH_ART_COOLDOWN[entry_id] = SMASH_ART_COOLDOWN[entry_id] - 1;
            }
            if ACCELERATE_ART_COOLDOWN[entry_id] > 0 {
                ACCELERATE_ART_COOLDOWN[entry_id] = ACCELERATE_ART_COOLDOWN[entry_id] - 1;
            }
        }
    }
}
pub unsafe fn can_monado_arts(module_accessor: &mut BattleObjectModuleAccessor) -> bool {
    if StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ICE
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_RUN
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_WIN
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_BURY
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_BURY_WAIT
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DASH
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_LOSE
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SLIP
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SWIM
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_TURN
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_WALK
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CATCH
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ENTRY
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FINAL
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_GUARD
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SLEEP
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SQUAT
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_THROW
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ATTACK
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ESCAPE
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_LADDER
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_THROWN
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_LANDING
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_REBIRTH
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_REBOUND
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN_EAT
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ESCAPE_B
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ESCAPE_F
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FURAFURA
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_GUARD_ON
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_TURN_RUN
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_BURY_JUMP
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN_WAIT
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_WARPSTAR
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_GUARD_OFF
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_RUN_BRAKE
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SLEEP_END
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SLIP_WAIT
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_STOP_CEIL
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_STOP_WALL
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SWALLOWED
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_TURN_DASH
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_WALL_JUMP
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_YOSHI_EGG
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CATCH_DASH
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CATCH_JUMP
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CATCH_PULL
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CATCH_TURN
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CATCH_WAIT
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_WAIT
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DOWN_STAND
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_ESCAPE_AIR
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_CATCH
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_CLIMB
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_JUMP1
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_JUMP2
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_JUMP3
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLUNG_DIDDY
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CATCH_ATTACK
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_ATTACK
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_ESCAPE
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_CLIFF_ROBBED
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FALL_SPECIAL
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FURAFURA_END
    || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_MEWTWO_THROWN {
      return false;
    }
    else {
      return true;
    }
}
                pub fn install() {
                    acmd::add_hooks!(
                        Monado_Ganon_Jab,
                        Monado_Ganon_Dash_Attack,
                        Monado_Ganon_Forward_Tilt,
                        Monado_Ganon_Up_Tilt,
                        Monado_Ganon_Down_Tilt,
                        Monado_Ganon_Forward_Smash,
                        Monado_Ganon_Up_Smash,
                        Monado_Ganon_Down_Smash,
                        Monado_Ganon_Nair,
                        Monado_Ganon_Fair,
                        Monado_Ganon_Bair,
                        Monado_Ganon_Uair,
                        Monado_Ganon_Dair,
                        Monado_Ganon_Grab,
                        Monado_Ganon_Dash_Grab,
                        Monado_Ganon_Pivot_Grab,
                        Monado_Ganon_Pummel,
                        Monado_Ganon_F_Throw,
                        Monado_Ganon_B_Throw,
                        Monado_Ganon_U_Throw,
                        Monado_Ganon_D_Throw,
                        Monado_Ganon_Neutral_Special,
                        Monado_Ganon_Reverse_Neutral_Special,
                        Monado_Ganon_Aerial_Neutral_Special,
                        Monado_Ganon_Aerial_Reverse_Neutral_Special,
                        Monado_Ganon_Grounded_Side_Special,
                        Monado_Ganon_Aerial_Side_Special,
                        Monado_Ganon_Grounded_Up_Special,
                        Monado_Ganon_Aerial_Up_Special,
                        Monado_Ganon_Grounded_Down_Special,
                        Monado_Ganon_Aerial_Down_Special,
                        Monado_Ganon_Shield_Left,
                        Monado_Ganon_Shield_Right,
                        Monado_Ganon_Left_Taunt,
                        Monado_Ganon_Right_Taunt,
                        Monado_Ganon_Accelerate_Left,
                        Monado_Ganon_Accelerate_Right,
                        Monado_Ganon_Shield_Left_Sound,
                        Monado_Ganon_Shield_Right_Sound,
                        Monado_Ganon_Left_Taunt_Sound,
                        Monado_Ganon_Right_Taunt_Sound,
                        Monado_Ganon_Accelerate_Left_Sound,
                        Monado_Ganon_Accelerate_Right_Sound,
                    );
                    acmd::add_custom_hooks!(
                        once_per_fighter_frame,
                    );
                }