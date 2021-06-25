#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
use skyline::hooks::{getRegionAddress, Region};
use smash::lib::lua_const::*;
use smash::app::BattleObjectModuleAccessor;
use smash::app::lua_bind::*;

pub static mut FLOAT_OFFSET: usize = 0x4dedc0;
pub static mut MONADO_STATE: [i32; 8] = [0; 8];

static FLOAT_SEARCH_CODE: &[u8] = &[0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,];

fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
  haystack.windows(needle.len()).position(|window| window == needle)
}

#[skyline::hook(offset = FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(boma: u64, param_type: u64, param_hash: u64) -> f32 {
    let module_accessor = &mut *(*((boma as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor);
    let ret = original!()(boma, param_type, param_hash);
    let fighter_kind = smash::app::utility::get_kind(module_accessor);
    let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if fighter_kind == *FIGHTER_KIND_GANON {
        if param_hash == smash::hash40("shield_max") {
            if MONADO_STATE[entry_id] == 1 {
                return 100.0;
            }
            else if MONADO_STATE[entry_id] == 0 {
                return 50.0;
            }
            else {
                return ret;
            }
        }
        else if param_hash == smash::hash40("jump_y") {
            if MONADO_STATE[entry_id] == 1 {
                return 17.08;
            }
            else if MONADO_STATE[entry_id] == 4 {
                return 30.58;
            }
            else if MONADO_STATE[entry_id] == 0 {
                return 25.49;
            }
            else {
                return ret;
            }
        }
        else if param_hash == smash::hash40("air_speed_x_stable") {
            if MONADO_STATE[entry_id] == 1 {
                return 0.56;
            }
            else if MONADO_STATE[entry_id] == 4 {
                return 1.0;
            }
            else if MONADO_STATE[entry_id] == 0 {
                return 0.83;
            }
            else {
               return ret;
            }
        }
        else {
            return ret;
        }
    }
    else {
        return ret;
    }
}

mod ganondorf;

#[skyline::main(name = "GM")]
pub fn main() {
    unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
        ganondorf::install();
        skyline::install_hook!(get_param_float_replace);
    }
}