use std::collections::HashMap;

use crate::structures::ent::Ent;
use crate::{
    mem::follow_offsets,
    offsets::{PLAYER, PLAYER_COUNT, PLAYER_LIST, VIEW_MATRIX},
    structures::ViewMatrix,
};

use self::aimbot::Aimbot;

pub mod aimbot;

pub struct Cheat {
    pub view_matrix: *mut ViewMatrix,
    pub local_player: *mut Ent,
    pub player_count: *mut u32,
    pub player_list: *mut [*mut Ent; 255],
    pub modules: HashMap<String, Box<dyn CheatModule>>,
}

pub trait CheatModule {
    fn cheat(&self) -> *mut Cheat;
}

impl Cheat {
    pub fn init() -> Cheat {
        Cheat {
            view_matrix: follow_offsets(VIEW_MATRIX, []),
            local_player: follow_offsets(PLAYER, []),
            player_count: follow_offsets(PLAYER_COUNT, []),
            player_list: follow_offsets(PLAYER_LIST, []),
            modules: HashMap::new(),
        }
    }

    pub fn load_modules(&mut self) {
        let aimbot = Aimbot::new(self as *mut Cheat);
        self.modules.insert("aimbot".to_owned(), Box::new(aimbot));
    }
}
