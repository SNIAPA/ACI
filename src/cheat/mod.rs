use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use crate::log;
use crate::util::Result;

use crate::structures::ent::Ent;
use crate::{
    mem::follow_offsets,
    offsets::{PLAYER, PLAYER_COUNT, PLAYER_LIST, VIEW_MATRIX},
    structures::ViewMatrix,
};

use self::aimbot::Aimbot;
use self::esp::Esp;

pub mod aimbot;
pub mod esp;

pub struct Cheat {
    pub view_matrix: *mut ViewMatrix,
    pub local_player: *mut Ent,
    pub player_count: *mut u32,
}


pub trait CheatModule {
    unsafe fn run(&self, writable: &mut Cheat) -> Result<()>;
}

impl Cheat {
    pub fn init() -> Cheat {
        Cheat {
            view_matrix: follow_offsets(VIEW_MATRIX, [0]),
            local_player: follow_offsets(PLAYER, [0]),
            player_count: follow_offsets(PLAYER_COUNT, []),
        }
    }
    unsafe fn get_players(&self) -> *mut [*mut Ent; 32] {
        follow_offsets::<[*mut Ent;32]>(PLAYER_LIST, [0x8])
    }

}
