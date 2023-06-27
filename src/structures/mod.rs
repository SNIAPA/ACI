
pub mod ent;

pub type ViewMatrix = [[f32; 4]; 4];

#[derive(Debug, Clone, Copy)]
pub struct Pos{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

#[derive(Debug, Clone, Copy)]
pub struct ViewAngles {
    pub yaw: f32,
    pub pitch: f32
}
