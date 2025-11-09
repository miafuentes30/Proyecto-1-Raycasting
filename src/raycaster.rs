use crate::map::Map;
use crate::util::{WIDTH, HEIGHT};

pub struct Raycaster {
    pub zbuffer: Vec<f32>,
}

impl Raycaster {
    pub fn new() -> Self {
        Self { zbuffer: vec![0.0; WIDTH as usize] }
    }

    pub fn render(&mut self, frame: &mut [u8], map: &Map, px: f32, py: f32, ang: f32, fov: f32) {
        // cielo/suelo
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let i = ((y * WIDTH + x) * 4) as usize;
                let sky = y < HEIGHT / 2;
                frame[i + 0] = if sky { 90 } else { 30 };
                frame[i + 1] = if sky { 140 } else { 30 };
                frame[i + 2] = if sky { 200 } else { 30 };
                frame[i + 3] = 255;
            }
        }

        let start_ang = ang - fov * 0.5;
        for sx in 0..WIDTH as usize {
            let ray_ang = start_ang + (sx as f32 / WIDTH as f32) * fov;
            let (dist, wall_id, shade) = cast(map, px, py, ray_ang);
            self.zbuffer[sx] = dist;

            let d = dist.max(0.0001);
            let wall_h = (HEIGHT as f32 / d) as i32;
            let top = (HEIGHT as i32 / 2) - wall_h / 2;
            let bot = top + wall_h;

            let color = id_to_color(wall_id, shade);
            for y in top..bot {
                if y >= 0 && (y as u32) < HEIGHT {
                    let i = (((y as u32) * WIDTH + sx as u32) * 4) as usize;
                    frame[i..i + 3].copy_from_slice(&color);
                    frame[i + 3] = 255;
                }
            }
        }
    }
}

fn id_to_color(id: u8, shade: f32) -> [u8; 3] {
    let base = match id {
        1 => [200, 40, 40],
        2 => [40, 200, 40],
        3 => [40, 40, 220],
        4 => [210, 210, 60],
        _ => [200, 200, 200],
    };
    [
        (base[0] as f32 * shade) as u8,
        (base[1] as f32 * shade) as u8,
        (base[2] as f32 * shade) as u8,
    ]
}

fn cast(map: &Map, px: f32, py: f32, ang: f32) -> (f32, u8, f32) {
    let dx = ang.cos();
    let dy = ang.sin();

    let mut map_x = px.floor() as i32;
    let mut map_y = py.floor() as i32;

    let delta_dist_x = if dx == 0.0 { 1e30 } else { (1.0 / dx).abs() };
    let delta_dist_y = if dy == 0.0 { 1e30 } else { (1.0 / dy).abs() };

    let (step_x, mut side_dist_x) = if dx < 0.0 {
        (-1, (px - map_x as f32) * delta_dist_x)
    } else {
        (1, ((map_x as f32 + 1.0) - px) * delta_dist_x)
    };

    let (step_y, mut side_dist_y) = if dy < 0.0 {
        (-1, (py - map_y as f32) * delta_dist_y)
    } else {
        (1, ((map_y as f32 + 1.0) - py) * delta_dist_y)
    };

    let mut side = 0;
    let mut wall_id = 0u8;

    loop {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = 0;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = 1;
        }
        wall_id = map.at(map_x, map_y);
        if wall_id != 0 { break; }
        if (map_x as usize) >= map.w || (map_y as usize) >= map.h { break; }
    }

    let dist = if side == 0 {
        (map_x as f32 - px + (1.0 - step_x as f32) / 2.0) / dx
    } else {
        (map_y as f32 - py + (1.0 - step_y as f32) / 2.0) / dy
    };

    let shade = if side == 0 { 0.85 } else { 1.0 };
    (dist.abs().max(0.001), wall_id, shade)
}
