pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;


pub fn clamp(v: f32, lo: f32, hi: f32) -> f32 { v.max(lo).min(hi) }


pub fn deg2rad(d: f32) -> f32 { d * std::f32::consts::PI / 180.0 }