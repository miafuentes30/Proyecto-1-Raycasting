use raylib::math::Vector2;
use crate::player::Player;
use crate::maze::Maze;
use crate::caster::is_blocked_by_wall;

#[derive(Clone)]
pub struct Enemy {
    pub pos: Vector2,
    pub texture_key: char,
    pub anim_offset: f32,
    pub active: bool,
    pub detection_range: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32) -> Self {
        Enemy {
            pos: Vector2::new(x, y),
            texture_key: 'F',
            anim_offset: 0.0,
            active: false,
            detection_range: 150.0,
        }
    }
    
    /// Actualiza el enemigo 
    pub fn update(&mut self, player: &Player, maze: &Maze, block_size: usize, _delta_time: f32) {
        let dist_to_player = distance(&self.pos, &player.pos);
        
        if !self.active {
            if dist_to_player < self.detection_range {
                let blocked = is_blocked_by_wall(
                    self.pos.x,
                    self.pos.y,
                    player.pos.x,
                    player.pos.y,
                    maze,
                    block_size
                );
                
                if !blocked {
                    self.active = true;
                }
            }
        }
        
        if self.active {
            self.move_towards_player(player, maze);
        }
        
        self.anim_offset += 0.12;
        if self.anim_offset > std::f32::consts::PI * 2.0 {
            self.anim_offset -= std::f32::consts::PI * 2.0;
        }
    }
    
    fn move_towards_player(&mut self, player: &Player, maze: &Maze) {
        let speed = 1.2;
        let dir_x = player.pos.x - self.pos.x;
        let dir_y = player.pos.y - self.pos.y;
        let len = (dir_x * dir_x + dir_y * dir_y).sqrt().max(0.001);
        
        let nx = self.pos.x + (dir_x / len) * speed;
        let ny = self.pos.y + (dir_y / len) * speed;
        
        let block_size = 20.0;
        let i = (nx / block_size) as isize;
        let j = (ny / block_size) as isize;
        
        if j >= 0 && i >= 0 {
            let j_u = j as usize;
            let i_u = i as usize;
            
            if j_u < maze.len() && i_u < maze[0].len() {
                let cell = maze[j_u][i_u];
                
                if cell != '#' && cell != 'L' && cell != '$' {
                    self.pos.x = nx;
                    self.pos.y = ny;
                } else {
                    let nx_only = self.pos.x + (dir_x / len) * speed;
                    let ny_only = self.pos.y + (dir_y / len) * speed;
                    
                    let i_x = (nx_only / block_size) as isize;
                    let j_y = (ny_only / block_size) as isize;
                    
                    let can_x = i_x >= 0 && (j as usize) < maze.len() 
                        && (i_x as usize) < maze[0].len() 
                        && maze[j as usize][i_x as usize] != '#'
                        && maze[j as usize][i_x as usize] != 'L'
                        && maze[j as usize][i_x as usize] != '$';
                    
                    let can_y = j_y >= 0 && (j_y as usize) < maze.len() 
                        && (i as usize) < maze[0].len() 
                        && maze[j_y as usize][i as usize] != '#'
                        && maze[j_y as usize][i as usize] != 'L'
                        && maze[j_y as usize][i as usize] != '$';
                    
                    if can_x {
                        self.pos.x = nx_only;
                    } else if can_y {
                        self.pos.y = ny_only;
                    }
                }
            }
        }
    }
}

/// Funcion para calcular distancia entre dos vectores
pub fn distance(v1: &Vector2, v2: &Vector2) -> f32 {
    ((v1.x - v2.x).powi(2) + (v1.y - v2.y).powi(2)).sqrt()
}