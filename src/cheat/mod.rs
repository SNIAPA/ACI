use std::collections::HashMap;

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
    pub player_list: *mut [*mut Ent; 255],
    pub modules: HashMap<String, Box<dyn CheatModule>>,
}

pub trait CheatModule {
    fn cheat(&self) -> *mut Cheat;
    unsafe fn run(&self);
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

        let esp = Esp::new(self as *mut Cheat);
        self.modules.insert("esp".to_owned(), Box::new(esp));
    }

    pub fn run(&mut self) {
        self.modules.values().for_each(|module|{
            unsafe{
                module.run()
            }
        })
    }
}
