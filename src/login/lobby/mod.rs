mod cache;
mod greet;

use crate::login::*;

use self::greet::{handle_enter_lobby, handle_leave_lobby};

pub fn init(world_helper: &mut WorldHelper) {
    world_helper
        .add_system(handle_enter_lobby)
        .add_system(handle_leave_lobby);
}