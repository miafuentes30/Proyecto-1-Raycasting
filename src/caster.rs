use raylib::color::Color;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::Maze;
use crate::intersect::Intersect;

pub fn cast_ray(
    maze: &Maze,
    player: &Player,
    ray_angle: f32,
    block_size: usize,
) -> Intersect {
    let mut d = 0.0;
    
    loop {
        let cos = d * ray_angle.cos();
        let sin = d * ray_angle.sin();
        let x = player.pos.x + cos;
        let y = player.pos.y + sin;
        
        let i = (x / block_size as f32) as usize;
        let j = (y / block_size as f32) as usize;
        
        if j >= maze.len() || i >= maze[0].len() {
            return Intersect::new(d, ' ', 0.0);
        }
        
        // Solo #, L y $ son paredes que bloquean la vista
        if maze[j][i] == '#' || maze[j][i] == 'L' || maze[j][i] == '$' {
            let block_x = (i * block_size) as f32;
            let block_y = (j * block_size) as f32;
            
            let offset_x = (x - block_x) / block_size as f32;
            let offset_y = (y - block_y) / block_size as f32;
            
            let offset = if offset_x.abs() < 0.05 || (1.0 - offset_x).abs() < 0.05 {
                offset_y
            } else if offset_y.abs() < 0.05 || (1.0 - offset_y).abs() < 0.05 {
                offset_x
            } else {
                if offset_x.min(1.0 - offset_x) < offset_y.min(1.0 - offset_y) {
                    offset_y
                } else {
                    offset_x
                }
            };
            
            return Intersect::new(d, maze[j][i], offset);
        }
        
        d += 1.0;
        if d > 1000.0 {
            return Intersect::new(d, ' ', 0.0);
        }
    }
}

pub fn cast_ray_debug(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    ray_angle: f32,
    block_size: usize,
) -> Intersect {
    let mut d = 0.0;
    framebuffer.set_current_color(Color::new(255, 0, 0, 100));
    
    loop {
        let cos = d * ray_angle.cos();
        let sin = d * ray_angle.sin();
        let x = player.pos.x + cos;
        let y = player.pos.y + sin;
        
        let i = (x / block_size as f32) as usize;
        let j = (y / block_size as f32) as usize;
        
        if j >= maze.len() || i >= maze[0].len() {
            return Intersect::new(d, ' ', 0.0);
        }
        
        if maze[j][i] == '#' || maze[j][i] == 'L' || maze[j][i] == '$' {
            framebuffer.set_pixel(x as i32, y as i32);
            
            let block_x = (i * block_size) as f32;
            let block_y = (j * block_size) as f32;
            
            let offset_x = (x - block_x) / block_size as f32;
            let offset_y = (y - block_y) / block_size as f32;
            
            let offset = if offset_x.abs() < 0.05 || (1.0 - offset_x).abs() < 0.05 {
                offset_y
            } else if offset_y.abs() < 0.05 || (1.0 - offset_y).abs() < 0.05 {
                offset_x
            } else {
                if offset_x.min(1.0 - offset_x) < offset_y.min(1.0 - offset_y) {
                    offset_y
                } else {
                    offset_x
                }
            };
            
            return Intersect::new(d, maze[j][i], offset);
        }
        
        framebuffer.set_pixel(x as i32, y as i32);
        d += 1.0;
        
        if d > 1000.0 {
            return Intersect::new(d, ' ', 0.0);
        }
    }
}

/// Retorna true si hay una pared (bloqueado), false si está libre
pub fn is_blocked_by_wall(
    from_x: f32,
    from_y: f32,
    to_x: f32,
    to_y: f32,
    maze: &Maze,
    block_size: usize,
) -> bool {
    let dx = to_x - from_x;
    let dy = to_y - from_y;
    let distance = (dx * dx + dy * dy).sqrt();
    
    if distance < 1.0 {
        return false;
    }
    
    let steps = (distance / 5.0).ceil() as usize;
    let step_x = dx / steps as f32;
    let step_y = dy / steps as f32;
    
    for step in 0..steps {
        let x = from_x + step_x * step as f32;
        let y = from_y + step_y * step as f32;
        
        let i = (x / block_size as f32) as isize;
        let j = (y / block_size as f32) as isize;
        
        if j < 0 || i < 0 {
            return true;
        }
        
        let j_u = j as usize;
        let i_u = i as usize;
        
        if j_u >= maze.len() || i_u >= maze[0].len() {
            return true;
        }
        
        let cell = maze[j_u][i_u];
        if cell == '#' || cell == 'L' || cell == '$' {
            return true;
        }
    }
    
    false
}
