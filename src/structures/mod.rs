
pub mod ent;

pub type ViewMatrix = [[f32; 4]; 4];

#[derive(Debug, Clone, Copy)]
pub struct Pos{
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl Pos {
    pub fn dist(&self, target: &Pos) -> f32{
        f32::sqrt((self.x-target.x).powi(2) + (self.y-target.y).powi(2) + (self.z-target.z).powi(2))
    }
}


#[derive(Debug, Clone, Copy)]
pub struct ViewAngles {
    pub yaw: f32,
    pub pitch: f32
}
