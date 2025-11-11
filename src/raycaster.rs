use crate::map::Map;
use crate::util::{WIDTH, HEIGHT, clamp01, Effects};

pub struct Raycaster { 
    pub zbuffer: Vec<f32>,
    tex_cache: Vec<Vec<[u8;3]>>,
}

impl Raycaster {
    pub fn new() -> Self { 
        // cachear texturas al inicio para mejor perf
        let mut tex_cache = Vec::new();
        for wall_id in 0..10 {
            let mut tex = Vec::new();
            for ty in 0..32 {
                for tx in 0..32 {
                    tex.push(generate_texture_pixel(wall_id, tx, ty));
                }
            }
            tex_cache.push(tex);
        }
        
        Self { 
            zbuffer: vec![0.0; WIDTH as usize],
            tex_cache,
        } 
    }

    pub fn render(&mut self, frame: &mut [u8], map: &Map, px: f32, py: f32, ang: f32, fov: f32, fx: Effects) {
        // cielo con gradiente
        for y in 0..HEIGHT/2 {
            let t = y as f32 / (HEIGHT/2) as f32;
            let t_smooth = t * t * (3.0 - 2.0 * t);
            
            let r = (135.0 * (1.0-t_smooth) + 40.0 * t_smooth) as u8;
            let g = (206.0 * (1.0-t_smooth) + 70.0 * t_smooth) as u8;
            let b = (235.0 * (1.0-t_smooth) + 130.0 * t_smooth) as u8;
            
            for x in 0..WIDTH {
                let i = ((y * WIDTH + x) * 4) as usize;
                frame[i] = r;
                frame[i+1] = g;
                frame[i+2] = b;
                frame[i+3] = 255;
                
                // algunas estrellas
                if y < HEIGHT / 4 && ((x * 73 + y * 97) % 997) < 2 {
                    let brightness = 200 + ((x + y) % 55) as u8;
                    frame[i] = brightness;
                    frame[i+1] = brightness;
                    frame[i+2] = brightness;
                }
            }
        }
        
        // suelo con textura de pasto
        for y in HEIGHT/2..HEIGHT {
            let t = (y - HEIGHT/2) as f32 / (HEIGHT/2) as f32;
            
            let base_r = (40.0 + t * 30.0) as u8;
            let base_g = (80.0 + t * 40.0) as u8;
            let base_b = (30.0 + t * 20.0) as u8;
            
            for x in 0..WIDTH {
                let i = ((y * WIDTH + x) * 4) as usize;
                
                // patrón random de hierba
                let grass = ((x * 5 + y * 7) % 13) as i8 - 6;
                
                frame[i] = (base_r as i16 + grass as i16).clamp(0, 255) as u8;
                frame[i+1] = (base_g as i16 + grass as i16).clamp(0, 255) as u8;
                frame[i+2] = (base_b as i16 + grass as i16 / 2).clamp(0, 255) as u8;
                frame[i+3] = 255;
            }
        }

        let start_ang = ang - fov * 0.5;
        
        // raycast para cada columna
        for sx in 0..WIDTH as usize {
            let ray_ang = start_ang + (sx as f32 / WIDTH as f32) * fov;
            let (dist, wall_id, side, hit_x, hit_y) = cast(map, px, py, ray_ang);

            let d = dist.max(0.1).min(100.0);
            self.zbuffer[sx] = d;

            // proyección con corrección fish-eye
            let wall_h = ((HEIGHT as f32 / d) as i32).min(HEIGHT as i32 * 2);
            let top = ((HEIGHT as i32 / 2) - wall_h / 2).max(0);
            let bot = (top + wall_h).min(HEIGHT as i32);

            // coord de textura horizontal
            let tex_x = (hit_x * 32.0) as u32 % 32;

            for y in top..bot {
                if y >= 0 && (y as u32) < HEIGHT {
                    // coord de textura vertical
                    let tex_y = ((y - top) as f32 / wall_h.max(1) as f32 * 32.0) as u32 % 32;
                    
                    // sacar color del cache
                    let mut color = if wall_id < 10 {
                        self.tex_cache[wall_id as usize][(tex_y * 32 + tex_x) as usize]
                    } else {
                        [200, 200, 200]
                    };

                    // oscurecer caras laterales
                    if side == 0 {
                        for c in &mut color {
                            *c = ((*c as f32) * 0.75) as u8;
                        }
                    }

                    // fog exponencial
                    let fog = if fx.fog_density > 0.0 {
                        let fog_raw = (-(fx.fog_density * d)).exp();
                        fog_raw * fog_raw * (3.0 - 2.0 * fog_raw)
                    } else { 
                        1.0 
                    };

                    // linterna con cono
                    let mut spot = 1.0;
                    if fx.flashlight {
                        let delta = (ray_ang - ang).abs().to_degrees();
                        let cone = fx.cone_deg.max(1.0);
                        let t = clamp01(1.0 - (delta / cone));
                        spot = t * t * (3.0 - 2.0 * t);
                        spot = spot * 0.7 + 0.3;
                    }
                    
                    // pulsación animada para lava
                    let mut intensity_mod = 1.0;
                    if wall_id == 5 {
                        intensity_mod = 0.85 + ((fx.time * 4.0 + hit_y * 10.0).sin() * 0.15);
                    }

                    let shade = clamp01(fog * spot * intensity_mod);
                    
                    let fog_color = [90, 140, 200];
                    
                    let i = (((y as u32) * WIDTH + sx as u32) * 4) as usize;
                    frame[i+0] = ((color[0] as f32 * shade) + (fog_color[0] as f32 * (1.0 - fog))) as u8;
                    frame[i+1] = ((color[1] as f32 * shade) + (fog_color[1] as f32 * (1.0 - fog))) as u8;
                    frame[i+2] = ((color[2] as f32 * shade) + (fog_color[2] as f32 * (1.0 - fog))) as u8;
                    frame[i+3] = 255;
                }
            }
        }
    }
}

// texturas procedurales para cada tipo de pared
fn generate_texture_pixel(wall_id: u8, tx: u32, ty: u32) -> [u8; 3] {
    match wall_id {
        1 => {
            // ladrillos rojos
            let brick_w = 8;
            let brick_h = 4;
            let mortar = 1;
            
            let bx = tx % (brick_w + mortar);
            let by = ty % (brick_h + mortar);
            
            let offset = if (ty / (brick_h + mortar)) % 2 == 0 { 0 } else { brick_w / 2 };
            let bx_offset = (tx + offset) % (brick_w + mortar);
            
            if bx < mortar || by < mortar || bx_offset < mortar {
                [120, 110, 100]
            } else {
                let variation = ((tx * 7 + ty * 13) % 20) as i16 - 10;
                let base_r = (210 + variation).clamp(0, 255) as u8;
                let base_g = (55 + variation / 2).clamp(0, 255) as u8;
                let base_b = (50 + variation / 2).clamp(0, 255) as u8;
                [base_r, base_g, base_b]
            }
        }
        2 => {
            // piedra verde con musgo
            let moss = ((tx + ty * 3) % 7 == 0) as u8;
            let crack = ((tx % 16 == 0 || ty % 16 == 0) && (tx + ty) % 8 < 2) as u8;
            
            if moss == 1 {
                let var = ((tx * ty) % 15) as u8;
                [40 + var, 200 + var/2, 80 + var/3]
            } else if crack == 1 {
                [30, 130, 50]
            } else {
                let var = ((tx * 11 + ty * 17) % 25) as i16 - 12;
                [
                    (60 + var).clamp(0, 255) as u8,
                    (190 + var).clamp(0, 255) as u8,
                    (85 + var / 2).clamp(0, 255) as u8,
                ]
            }
        }
        3 => {
            // azulejos azules brillantes
            let tile_size = 8;
            let tx_tile = tx % tile_size;
            let ty_tile = ty % tile_size;
            
            let border = (tx_tile == 0 || ty_tile == 0 || tx_tile == tile_size - 1 || ty_tile == tile_size - 1) as u8;
            let shine = (tx_tile < 3 && ty_tile < 3) as u8;
            
            if border == 1 {
                [35, 65, 180]
            } else if shine == 1 {
                [120, 160, 255]
            } else {
                let checker = ((tx / tile_size + ty / tile_size) % 2) as u8;
                if checker == 0 {
                    [60, 110, 230]
                } else {
                    [50, 95, 215]
                }
            }
        }
        4 => {
            // oro metálico
            let pattern = ((tx / 4) ^ (ty / 4)) % 3;
            let shine = ((tx % 8 < 2) && (ty % 8 < 2)) as u8;
            
            if shine == 1 {
                [255, 240, 180]
            } else {
                match pattern {
                    0 => [230, 200, 70],
                    1 => [210, 180, 50],
                    _ => [190, 165, 40],
                }
            }
        }
        5 => {
            // lava que fluye
            let flow_pattern = ((tx + ty * 2) % 12) as f32 / 12.0;
            let bubble = ((tx * ty) % 37 < 5) as u8;
            
            let heat_base = 200 + (flow_pattern * 55.0) as u8;
            
            if bubble == 1 {
                [255, 255, 180]
            } else {
                [255, heat_base, 40 + (flow_pattern * 30.0) as u8]
            }
        }
        9 => {
            // portal cristalino
            let crystal = ((tx + ty) % 4) as u8;
            let glow = ((tx % 8 < 4) ^ (ty % 8 < 4)) as u8;
            
            if glow == 1 {
                [240 + crystal * 4, 240 + crystal * 4, 255]
            } else {
                [200 + crystal * 10, 200 + crystal * 10, 255]
            }
        }
        _ => {
            // piedra genérica gris
            let var = ((tx * 13 + ty * 17) % 30) as i16 - 15;
            [
                (180 + var).clamp(0, 255) as u8,
                (180 + var).clamp(0, 255) as u8,
                (180 + var).clamp(0, 255) as u8,
            ]
        }
    }
}

// DDA con manejo de bordes del mapa
fn cast(map: &Map, px: f32, py: f32, ang: f32) -> (f32, u8, i32, f32, f32) {
    let dx = ang.cos();
    let dy = ang.sin();

    let mut map_x = px.floor() as i32;
    let mut map_y = py.floor() as i32;

    let delta_dist_x = if dx.abs() < 0.0001 { 1e30 } else { (1.0 / dx).abs() };
    let delta_dist_y = if dy.abs() < 0.0001 { 1e30 } else { (1.0 / dy).abs() };

    let step_x: i32;
    let step_y: i32;
    let mut side_dist_x: f32;
    let mut side_dist_y: f32;

    if dx < 0.0 {
        step_x = -1;
        side_dist_x = (px - map_x as f32) * delta_dist_x;
    } else {
        step_x = 1;
        side_dist_x = (map_x as f32 + 1.0 - px) * delta_dist_x;
    }

    if dy < 0.0 {
        step_y = -1;
        side_dist_y = (py - map_y as f32) * delta_dist_y;
    } else {
        step_y = 1;
        side_dist_y = (map_y as f32 + 1.0 - py) * delta_dist_y;
    }

    let mut side = 0;
    let mut wall_id = 0;
    let mut hit_x = 0.0;
    let mut hit_y = 0.0;
    
    // DDA loop
    for _ in 0..200 {
        // fuera del mapa = pared exterior
        if map_x < 0 || map_y < 0 || map_x >= map.w as i32 || map_y >= map.h as i32 {
            wall_id = 1;
            
            // calcular hit point aproximado
            if map_x < 0 {
                hit_x = 0.0;
                hit_y = py + (map_x as f32 - px) * dy / dx;
            } else if map_x >= map.w as i32 {
                hit_x = 1.0;
                hit_y = py + (map_x as f32 - px) * dy / dx;
            } else if map_y < 0 {
                hit_x = px + (map_y as f32 - py) * dx / dy;
                hit_y = 0.0;
            } else {
                hit_x = px + (map_y as f32 - py) * dx / dy;
                hit_y = 1.0;
            }
            break;
        }

        // ver si hay pared aquí
        wall_id = map.at(map_x, map_y);
        if wall_id != 0 {
            // calcular punto exacto de impacto
            if side == 0 {
                hit_x = py + (map_x as f32 - px + (1.0 - step_x as f32) / 2.0) * dy / dx;
                hit_y = map_y as f32;
            } else {
                hit_x = map_x as f32;
                hit_y = px + (map_y as f32 - py + (1.0 - step_y as f32) / 2.0) * dx / dy;
            }
            break;
        }

        // avanzar un tile
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            map_x += step_x;
            side = 0;
        } else {
            side_dist_y += delta_dist_y;
            map_y += step_y;
            side = 1;
        }
    }

    // distancia perpendicular (sin fish-eye)
    let dist = if side == 0 {
        (map_x as f32 - px + (1.0 - step_x as f32) / 2.0) / dx
    } else {
        (map_y as f32 - py + (1.0 - step_y as f32) / 2.0) / dy
    }.abs();

    // para paredes del borde, usar coords apropiadas
    let hit_x_frac = if wall_id == 1 && (map_x < 0 || map_x >= map.w as i32 || map_y < 0 || map_y >= map.h as i32) {
        if map_x < 0 || map_x >= map.w as i32 {
            hit_y.fract().abs()
        } else {
            hit_x.fract().abs()
        }
    } else {
        if side == 0 {
            hit_x.fract()
        } else {
            hit_y.fract()
        }
    };

    (dist.max(0.01), wall_id, side, hit_x_frac, hit_y)
}