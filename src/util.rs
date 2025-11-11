pub const WIDTH:  u32 = 1024;
pub const HEIGHT: u32 = 576;

pub fn deg2rad(d: f32) -> f32 { d.to_radians() }

pub fn clamp01(x: f32) -> f32 { x.max(0.0).min(1.0) }

// Colores 
pub const WHITE: [u8;3] = [255,255,255];
pub const BLACK: [u8;3] = [0,0,0];
pub const RED:   [u8;3] = [220,64,64];
pub const GREEN: [u8;3] = [64,220,64];
pub const BLUE:  [u8;3] = [64,64,220];

// Estados de juego
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GameState { Menu, Playing, Win }

// Efectos visuales 
#[derive(Copy, Clone)]
pub struct Effects {
    pub fog_density: f32,    
    pub flashlight:  bool,   
    pub cone_deg:    f32,    
    pub time:        f32,    
}

impl Default for Effects {
    fn default() -> Self {
        Self { 
            fog_density: 0.05,  
            flashlight: true, 
            cone_deg: 70.0,     
            time: 0.0,
        }
    }
}