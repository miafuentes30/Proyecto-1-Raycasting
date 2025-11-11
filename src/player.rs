use crate::map::Map;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub ang: f32,
    pub fov: f32,
    pub speed: f32,
    pub life: i32, 
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y, ang: 0.0, fov: 1.0, speed: 2.8, life: 5 }
    }
    
    pub fn add_yaw(&mut self, dx: f32) {
        self.ang += (dx as f32) * 0.0025;
    }

    // Funcion auxiliar para verificar si una posición es valida
    fn can_move_to(&self, map: &Map, x: f32, y: f32) -> bool {
        let tile = map.at(x.floor() as i32, y.floor() as i32);
        tile == 0 || tile == 5 || tile == 9  
    }

    pub fn try_move(&mut self, dir_x: f32, dir_y: f32, dt: f32, map: &Map) {
        let step = self.speed * dt;
        let nx = self.x + dir_x * step;
        let ny = self.y + dir_y * step;

        // Sistema de colisiones 
        if self.can_move_to(map, nx, self.y) {
            self.x = nx;
        } else {
            let small_step_x = self.x + dir_x * step * 0.1;
            if self.can_move_to(map, small_step_x, self.y) {
                self.x = small_step_x;
            }
        }

        if self.can_move_to(map, self.x, ny) {
            self.y = ny;
        } else {
            let small_step_y = self.y + dir_y * step * 0.1;
            if self.can_move_to(map, self.x, small_step_y) {
                self.y = small_step_y;
            }
        }

        // daño por lava
        let tile = map.at(self.x.floor() as i32, self.y.floor() as i32);
        if tile == 5 {
            if self.life > 0 { self.life -= 1; }
        }
    }
}