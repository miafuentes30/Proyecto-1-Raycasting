pub struct Player {
pub x: f32,
pub y: f32,
pub ang: f32, 
pub fov: f32, 
pub speed: f32,
pub rot_speed: f32,
}


impl Player {
pub fn new(x: f32, y: f32) -> Self {
Self { x, y, ang: 0.0, fov: 1.0, speed: 3.0, rot_speed: 0.0025 }
}


pub fn add_yaw(&mut self, dx_pixels: f32) {
self.ang += dx_pixels * self.rot_speed;
while self.ang > std::f32::consts::PI { self.ang -= 2.0*std::f32::consts::PI; }
while self.ang < -std::f32::consts::PI { self.ang += 2.0*std::f32::consts::PI; }
}


pub fn try_move(&mut self, dir_x: f32, dir_y: f32, dt: f32, map: &crate::map::Map) {
let nx = self.x + dir_x * self.speed * dt;
let ny = self.y + dir_y * self.speed * dt;
let pad = 0.15f32; 
let tx = nx as i32; let ty = self.y as i32;
if !map.is_wall(tx, ty) && !map.is_wall((nx+pad) as i32, ty) && !map.is_wall((nx-pad) as i32, ty) {
self.x = nx;
}
let tx2 = self.x as i32; let ty2 = ny as i32;
if !map.is_wall(tx2, ty2) && !map.is_wall(tx2, (ny+pad) as i32) && !map.is_wall(tx2, (ny-pad) as i32) {
self.y = ny;
}
}
}