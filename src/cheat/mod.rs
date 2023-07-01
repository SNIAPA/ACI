use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
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
    pub player_list: *mut [u64; 255],
    pub modules: HashMap<String, Box<dyn CheatModule>>,
}

pub trait CheatModule {
    unsafe fn run(&self) -> Result<()>;
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

    pub fn load_modules(arcrl: Arc<RwLock<Cheat>>) -> Result<()>{

        
        let aimbot = Aimbot::new(arcrl.clone());
        arcrl.write().unwrap().modules.insert("aimbot".to_owned(), Box::new(aimbot));

        let esp = Esp::new(arcrl.clone());
        arcrl.write().unwrap().modules.insert("esp".to_owned(), Box::new(esp));
        Ok(())
    }

    pub fn run(arcrl: Arc<RwLock<Cheat>>) -> Result<()>{
        arcrl.read().unwrap().modules.values().for_each(|module|{
            unsafe{
                module.run();
            }
        });
        Ok(())
    }
}
