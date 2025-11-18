use raylib::math::Vector2;

pub struct Player {
    pub pos: Vector2,
    pub a: f32,
    pub fov: f32,
    pub health: i32,
    pub max_health: i32,
    pub has_flashlight: bool,
    pub flashlight_battery: f32,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            pos: Vector2::new(x, y),
            a: std::f32::consts::PI / 3.0,
            fov: std::f32::consts::PI / 3.0,
            health: 100,
            max_health: 100,
            has_flashlight: true,
            flashlight_battery: 100.0,
        }
    }

    pub fn rotate(&mut self, angle: f32) {
        self.a += angle;
        self.a = self.a % (2.0 * std::f32::consts::PI);
        if self.a < 0.0 {
            self.a += 2.0 * std::f32::consts::PI;
        }
    }

    pub fn move_forward(&mut self, distance: f32, maze: &super::maze::Maze) -> bool {
        let new_x = self.pos.x + distance * self.a.cos();
        let new_y = self.pos.y + distance * self.a.sin();
        self.try_move(new_x, new_y, maze)
    }

    pub fn move_backward(&mut self, distance: f32, maze: &super::maze::Maze) -> bool {
        let new_x = self.pos.x - distance * self.a.cos();
        let new_y = self.pos.y - distance * self.a.sin();
        self.try_move(new_x, new_y, maze)
    }

    pub fn try_move(&mut self, new_x: f32, new_y: f32, maze: &super::maze::Maze) -> bool {
        let steps = 6;
        let dx = (new_x - self.pos.x) / steps as f32;
        let dy = (new_y - self.pos.y) / steps as f32;
        let mut nx = self.pos.x;
        let mut ny = self.pos.y;

        for _ in 0..steps {
            nx += dx;
            ny += dy;

            let block_size = 20.0;
            let i = (nx / block_size) as isize;
            let j = (ny / block_size) as isize;

            if j < 0 || i < 0 {
                return false;
            }

            let j_usize = j as usize;
            let i_usize = i as usize;

            if j_usize >= maze.len() || i_usize >= maze[0].len() {
                return false;
            }

            let cell = maze[j_usize][i_usize];

            // Detectar puerta o salida
            if cell == '$' || cell == 'E' {
                self.pos.x = nx;
                self.pos.y = ny;
                return cell == 'E'; // Solo retorna true si es salida 'E'
            }

            if cell == '#' || cell == 'L' {
                return false;
            }
        }

        self.pos.x = new_x;
        self.pos.y = new_y;
        false
    }

    pub fn take_damage(&mut self, damage: i32) {
        self.health = (self.health - damage).max(0);
    }

    pub fn heal(&mut self, amount: i32) {
        self.health = (self.health + amount).min(self.max_health);
    }

    pub fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn use_flashlight(&mut self, delta_time: f32) {
        if self.has_flashlight && self.flashlight_battery > 0.0 {
            self.flashlight_battery = (self.flashlight_battery - 10.0 * delta_time).max(0.0);
        }
    }

    pub fn recharge_flashlight(&mut self, amount: f32) {
        self.flashlight_battery = (self.flashlight_battery + amount).min(100.0);
    }
}