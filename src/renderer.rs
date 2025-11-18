use raylib::color::Color;
use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::Maze;
use crate::caster::cast_ray;
use crate::texture::TextureManager;
use raylib::math::Vector2;

pub fn render_world_3d(
    framebuffer: &mut Framebuffer,
    maze: &Maze,
    player: &Player,
    block_size: usize,
    textures: &TextureManager,
) {
    let ray_step: usize = 2; 
    let num_rays = (framebuffer.width as usize + ray_step - 1) / ray_step;
    let hh = framebuffer.height as f32 / 2.0;

    let distance_to_projection_plane = (framebuffer.width as f32 / (2.0 * (player.fov / 2.0).tan())).abs();

    for i in 0..num_rays {
        let screen_x = (i * ray_step) as i32;
        let current_ray = i as f32 / num_rays as f32;
        let ray_angle = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(maze, player, ray_angle, block_size);

        let safe_distance = intersect.distance.max(0.1);
        let stake_height = block_size as f32;
        let adjusted_height = (stake_height / safe_distance) * distance_to_projection_plane;

        let stake_top = (hh - (adjusted_height / 2.0)) as i32;
        let stake_bottom = (hh + (adjusted_height / 2.0)) as i32;

        let cell_char = intersect.impact;
        
        // Mapeo de caracteres a texturas
        let texture_key = match cell_char {
            '#' => "caja2",      
            'L' => "caja3",      
            '$' => "caja5",      
            'M' => "caja4",      
            'T' => "tuveria1",   
            'P' => "tuveria2",   
            _ => "caja2",        
        };

        if let Some(image) = textures.get(texture_key) {
            let pixel_data = image.get_image_data();
            let width = image.width as usize;
            let height = image.height as usize;

            for y in stake_top..stake_bottom {
                if y >= 0 && y < framebuffer.height {
                    let texture_y = ((y - stake_top) as f32 / (stake_bottom - stake_top) as f32) * height as f32;
                    let texture_x = (intersect.offset * width as f32).min((width - 1) as f32);

                    let tx = texture_x as usize;
                    let ty = (texture_y as usize).min(height - 1);

                    let index = ty * width + tx;
                    if index < pixel_data.len() {
                        let pixel_color = pixel_data[index];
                        
                        // Aplicar iluminación basada en distancia
                        let distance_factor = 1.0 / (safe_distance / 80.0 + 1.0);
                        let color = Color::new(
                            (pixel_color.r as f32 * distance_factor) as u8,
                            (pixel_color.g as f32 * distance_factor) as u8,
                            (pixel_color.b as f32 * distance_factor) as u8,
                            255,
                        );

                        framebuffer.set_current_color(color);
                        for sx in 0..(ray_step as i32) {
                            let px = screen_x + sx;
                            if px >= 0 && px < framebuffer.width {
                                framebuffer.set_pixel(px, y);
                            }
                        }
                    }
                }
            }
        } else {
            // Fallback
            let fallback_color = match cell_char {
                '#' => Color::GRAY,
                'L' => Color::BROWN,
                '$' => Color::DARKGRAY,
                'T' | 'P' => Color::DARKGREEN,
                _ => Color::PURPLE,
            };
            
            framebuffer.set_current_color(fallback_color);
            for y in stake_top..stake_bottom {
                for sx in 0..(ray_step as i32) {
                    let px = screen_x + sx;
                    if px >= 0 && px < framebuffer.width && y >= 0 && y < framebuffer.height {
                        framebuffer.set_pixel(px, y);
                    }
                }
            }
        }

        // Suelo 
        let floor_distance_factor = 1.0 / (safe_distance / 50.0 + 1.0);
        framebuffer.set_current_color(Color::new(
            (40.0 * floor_distance_factor) as u8,
            (25.0 * floor_distance_factor) as u8,
            (15.0 * floor_distance_factor) as u8,
            255,
        ));
        for y in stake_bottom..framebuffer.height {
            for sx in 0..(ray_step as i32) {
                let px = screen_x + sx;
                if px >= 0 && px < framebuffer.width {
                    framebuffer.set_pixel(px, y);
                }
            }
        }

        // Cielo 
        let sky_distance_factor = 1.0 / (safe_distance / 70.0 + 1.0);
        framebuffer.set_current_color(Color::new(
            (20.0 * sky_distance_factor) as u8,
            (30.0 * sky_distance_factor) as u8,
            (60.0 * sky_distance_factor) as u8,
            255,
        ));
        for y in 0..stake_top {
            for sx in 0..(ray_step as i32) {
                let px = screen_x + sx;
                if px >= 0 && px < framebuffer.width {
                    framebuffer.set_pixel(px, y);
                }
            }
        }
    }
}

pub fn draw_sprite_billboard(
    framebuffer: &mut Framebuffer,
    sprite_pos: Vector2,
    player: &Player,
    block_size: usize,
    textures: &TextureManager,
    key: &str,
) {
    if let Some(image) = textures.get(key) {
        let pixel_data = image.get_image_data();
        let tw = image.width as usize;
        let th = image.height as usize;

        let dx = sprite_pos.x - player.pos.x;
        let dy = sprite_pos.y - player.pos.y;
        let distance = (dx * dx + dy * dy).sqrt().max(0.001);

        let angle_to_sprite = dy.atan2(dx);
        let mut rel_angle = angle_to_sprite - player.a;
        while rel_angle > std::f32::consts::PI { rel_angle -= 2.0 * std::f32::consts::PI; }
        while rel_angle < -std::f32::consts::PI { rel_angle += 2.0 * std::f32::consts::PI; }

        // Aumentar el rango de visión para sprites
        if rel_angle.abs() > player.fov / 2.0 + 0.5 {
            return;
        }

        let framebuffer_w = framebuffer.width as f32;
        let framebuffer_h = framebuffer.height as f32;
        let distance_to_projection_plane = (framebuffer_w / (2.0 * (player.fov / 2.0).tan())).abs();

        // Hacer sprites + grandes multiplicando por 1.2
        let sprite_height = (block_size as f32 / distance) * distance_to_projection_plane * 1.2;
        let sprite_width = sprite_height * (tw as f32 / th as f32);

        let center_x = (0.5 + (rel_angle / player.fov)) * framebuffer_w;
        let top = framebuffer_h / 2.0 - sprite_height / 2.0;
        let left = center_x - sprite_width / 2.0;

        for sy in 0..(sprite_height as i32) {
            let v = sy as f32 / sprite_height;
            let ty = ((v * th as f32).clamp(0.0, (th - 1) as f32)) as usize;
            let py = (top + sy as f32) as i32;
            if py < 0 || py >= framebuffer.height { continue; }

            for sx in 0..(sprite_width as i32) {
                let u = sx as f32 / sprite_width;
                let tx = ((u * tw as f32).clamp(0.0, (tw - 1) as f32)) as usize;
                let px = (left + sx as f32) as i32;
                if px < 0 || px >= framebuffer.width { continue; }

                let index = ty * tw + tx;
                if index >= pixel_data.len() { continue; }
                let pix = pixel_data[index];

                if pix.a > 5 {
                    let df = 1.0 / (distance / 100.0 + 1.0);
                    let color = Color::new(
                        (pix.r as f32 * df).min(255.0) as u8,
                        (pix.g as f32 * df).min(255.0) as u8,
                        (pix.b as f32 * df).min(255.0) as u8,
                        255,
                    );
                    framebuffer.set_current_color(color);
                    framebuffer.set_pixel(px, py);
                }
            }
        }
    }
}