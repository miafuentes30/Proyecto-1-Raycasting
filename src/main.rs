mod framebuffer;
mod line;
mod maze;
mod caster;
mod player;
mod input;
mod renderer;
mod intersect;
mod texture;
mod enemy;
mod audio;
mod ui;

use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::maze::{find_player_start, load_maze, print_maze};
use crate::input::process_events;
use crate::renderer::{render_world_3d, draw_sprite_billboard};
use crate::texture::TextureManager;
use crate::enemy::{Enemy, distance};
use crate::caster::is_blocked_by_wall;
use crate::audio::Audio;

use raylib::prelude::*;
use std::time::Instant;

enum GameState {
    Menu,
    Playing,
    Victory,
    GameOver,
}

#[derive(Clone, Copy)]
enum MessageType {
    ChestOpened,
    NeedAllChests,
}

#[derive(Clone)]
struct Chest {
    pos: Vector2,
    opened: bool,
}

impl Chest {
    fn new(x: f32, y: f32) -> Self {
        Chest {
            pos: Vector2::new(x, y),
            opened: false,
        }
    }
}

fn find_positions_in_maze(maze: &maze::Maze, target: char, block_size: usize) -> Vec<(f32, f32)> {
    let mut positions = Vec::new();
    for (j, row) in maze.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            if cell == target {
                let x = (i * block_size) as f32 + (block_size as f32 / 2.0);
                let y = (j * block_size) as f32 + (block_size as f32 / 2.0);
                positions.push((x, y));
            }
        }
    }
    positions
}

fn main() {
    let level_files = vec!["assets/levels/maze.txt", "assets/levels/maze2.txt"];
    let mut current_level = 0usize;
    let block_size = 20usize;
    
    let mut maze = load_maze(level_files[current_level]);
    println!("Laberinto cargado: {}", level_files[current_level]);
    print_maze(&maze);
    
    let (start_x, start_y) = find_player_start(&maze)
        .expect(" No se encontró posición inicial del jugador");
    let mut player = Player::new(start_x, start_y);
    
    let enemy_positions = find_positions_in_maze(&maze, 'F', block_size);
    let mut enemies: Vec<Enemy> = enemy_positions
        .iter()
        .map(|(x, y)| Enemy::new(*x, *y))
        .collect();

    let chest_positions = find_positions_in_maze(&maze, 'C', block_size);
    let mut chests: Vec<Chest> = chest_positions
        .iter()
        .map(|(x, y)| Chest::new(*x, *y))
        .collect();

    println!("Enemigos: {} | Cofres: {}", enemies.len(), chests.len());
    
    let window_width = 1280;
    let window_height = 720;
    
    let (mut rl, thread) = raylib::init()
        .size(window_width, window_height)
        .title("Raycaster - Proyecto Gráficas")
        .build();
    
    rl.set_target_fps(60);

    let audio = Audio::new();
    let mut last_health = player.health;
    let texture_manager = TextureManager::new(&mut rl);
    let mut prev_mouse_x = rl.get_mouse_position().x;
    let mut state = GameState::Menu;
    let mut flashlight_on: bool = false;
    let mut damage_overlay_alpha: f32 = 0.0;
    let mut message_timer: Option<(Instant, MessageType)> = None;
    let mut last_frame_time = Instant::now();

    while !rl.window_should_close() {
        let delta_time = last_frame_time.elapsed().as_secs_f32();
        last_frame_time = Instant::now();

        match state {
            GameState::Menu => {
                rl.show_cursor();
                if rl.is_key_pressed(KeyboardKey::KEY_ONE) { current_level = 0; }
                if rl.is_key_pressed(KeyboardKey::KEY_TWO) && level_files.len() > 1 { current_level = 1; }
                
                if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
                    maze = load_maze(level_files[current_level]);
                    let (sx, sy) = find_player_start(&maze).expect("No se encontró posición inicial del jugador");
                    player.pos.x = sx;
                    player.pos.y = sy;
                    player.a = std::f32::consts::PI / 3.0;
                    player.health = 100;
                    damage_overlay_alpha = 0.0;

                    enemies = find_positions_in_maze(&maze, 'F', block_size)
                        .iter()
                        .map(|(x, y)| Enemy::new(*x, *y))
                        .collect();
                    
                    chests = find_positions_in_maze(&maze, 'C', block_size)
                        .iter()
                        .map(|(x, y)| Chest::new(*x, *y))
                        .collect();
                    
                    state = GameState::Playing;
                    flashlight_on = false;
                    
                    // Centrar mouse
                    rl.set_mouse_position(Vector2::new((window_width / 2) as f32, (window_height / 2) as f32));
                    prev_mouse_x = (window_width / 2) as f32;
                }
                
                if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                    break;
                }
                
                let mut d = rl.begin_drawing(&thread);
                ui::draw_menu_screen(&mut d, current_level, level_files.len());
            }

            GameState::Playing => {
                rl.hide_cursor();
                let mouse_pos = rl.get_mouse_position();
                let mouse_dx = mouse_pos.x - prev_mouse_x;
                prev_mouse_x = mouse_pos.x;

                let mut level_changed = false;
                if player.health > 0 {
                    level_changed = process_events(&rl, &mut player, &maze, block_size, mouse_dx);
                }

                // Linterna on/off
                if rl.is_key_pressed(KeyboardKey::KEY_F) {
                    if player.flashlight_battery > 0.0 {
                        flashlight_on = !flashlight_on;
                    }
                }

                // Verificar cofres recogidos
                let chests_collected = chests.iter().filter(|c| c.opened).count();
                let all_chests_collected = chests_collected == chests.len();

                if level_changed && all_chests_collected {
                    if current_level == level_files.len() - 1 {
                        // Último nivel completado
                        state = GameState::Victory;
                        continue;
                    } else {
                        // Siguiente nivel
                        current_level += 1;
                        maze = load_maze(level_files[current_level]);
                        let (nx, ny) = find_player_start(&maze).expect("No start in next level");
                        player.pos.x = nx;
                        player.pos.y = ny;
                        player.health = 100;
                        damage_overlay_alpha = 0.0;

                        enemies = find_positions_in_maze(&maze, 'F', block_size)
                            .iter()
                            .map(|(x, y)| Enemy::new(*x, *y))
                            .collect();
                        
                        chests = find_positions_in_maze(&maze, 'C', block_size)
                            .iter()
                            .map(|(x, y)| Chest::new(*x, *y))
                            .collect();
                    }
                } else if level_changed && !all_chests_collected {
                    // Mensaje: faltan cofres
                    message_timer = Some((Instant::now(), MessageType::NeedAllChests));
                }

                for e in enemies.iter_mut() {
                    e.update(&player, &maze, block_size, delta_time);
                    if distance(&e.pos, &player.pos) < 12.0 && player.health > 0 {
                        player.health = (player.health - 1).max(0);
                        audio.play_hit();
                    }
                }

                for c in chests.iter_mut() {
                    if !c.opened && distance(&c.pos, &player.pos) < 15.0 {
                        c.opened = true;
                        audio.play_chest();
                        message_timer = Some((Instant::now(), MessageType::ChestOpened));
                    }
                }

                if player.health < last_health {
                    damage_overlay_alpha = 0.6;
                }
                last_health = player.health;

                // Batería de linterna
                if flashlight_on {
                    player.use_flashlight(delta_time);
                    if player.flashlight_battery <= 0.0 {
                        flashlight_on = false;
                    }
                } else {
                    // Recarga cuando está apagada
                    player.recharge_flashlight(10.0 * delta_time);
                }

                if player.health > 10 {
                    damage_overlay_alpha = (damage_overlay_alpha - 0.02).max(0.0);
                } else {
                    damage_overlay_alpha = 0.8;
                }

                if player.health <= 0 {
                    state = GameState::GameOver;
                    continue;
                }

                let mut fb = Framebuffer::new_buffer(window_width, window_height, Color::BLACK);
                render_world_3d(&mut fb, &maze, &player, block_size, &texture_manager);
                
                for e in enemies.iter() {
                    let blocked = is_blocked_by_wall(player.pos.x, player.pos.y, e.pos.x, e.pos.y, &maze, block_size);
                    if !blocked {
                        draw_sprite_billboard(&mut fb, e.pos, &player, block_size, &texture_manager, "enemy");
                    }
                }

                for c in chests.iter() {
                    if !c.opened {
                        let blocked = is_blocked_by_wall(player.pos.x, player.pos.y, c.pos.x, c.pos.y, &maze, block_size);
                        if !blocked {
                            draw_sprite_billboard(&mut fb, c.pos, &player, block_size, &texture_manager, "chest");
                        }
                    }
                }

                let texture = rl.load_texture_from_image(&thread, &fb.buffer)
                    .expect("Error creando textura principal");
                
                let current_fps = rl.get_fps();
                let chests_collected = chests.iter().filter(|c| c.opened).count();
                let total_chests = chests.len();
                
                let mut d = rl.begin_drawing(&thread);
                d.clear_background(Color::BLACK);
                d.draw_texture(&texture, 0, 0, Color::WHITE);
                
                ui::draw_minimap(&mut d, &maze, &player, &enemies, block_size);
                ui::draw_hud(&mut d, &player, current_fps, true, chests_collected, total_chests);
                ui::draw_vignette(&mut d, 0.25);

                // Indicador de linterna
                let fl_txt = if flashlight_on { "Linterna: " } else { "Linterna: " };
                d.draw_text(
                    &format!("{}  ({:.0}%)", fl_txt, player.flashlight_battery),
                    10,
                    70,
                    20,
                    if flashlight_on { Color::YELLOW } else { Color::GRAY },
                );
                
                d.draw_text(&format!("Nivel: {}/2", current_level + 1), 10, 40, 24, Color::YELLOW);

                if let Some((start, msg_type)) = message_timer {
                    if start.elapsed().as_secs_f32() < 2.0 {
                        let (msg, color) = match msg_type {
                            MessageType::ChestOpened => ("Cofre abierto!", Color::GOLD),
                            MessageType::NeedAllChests => ("¡Debes recoger TODOS los cofres primero!", Color::RED),
                        };
                        
                        let text_width = d.measure_text(msg, 36);
                        d.draw_text(msg, (window_width - text_width) / 2, window_height / 2 - 30, 36, color);
                    } else {
                        message_timer = None;
                    }
                }

                if damage_overlay_alpha > 0.01 {
                    let color = Color::new(255, 0, 0, (damage_overlay_alpha * 255.0) as u8);
                    d.draw_rectangle_lines_ex(
                        Rectangle::new(0.0, 0.0, window_width as f32, window_height as f32),
                        25.0,
                        color
                    );
                }

                // Overlay de linterna
                if flashlight_on {
                    ui::draw_flashlight_overlay(&mut d, player.flashlight_battery);
                }
            }

            GameState::Victory => {
                let key_m_pressed = rl.is_key_pressed(KeyboardKey::KEY_M);
                
                let mut d = rl.begin_drawing(&thread);
                d.draw_rectangle_gradient_v(0, 0, window_width, window_height, Color::new(0, 60, 30, 255), Color::new(0, 30, 10, 255));
                d.draw_text("¡VICTORIA!", 400, 200, 80, Color::GOLD);
                d.draw_text("¡Completaste todos los niveles!", 320, 320, 36, Color::WHITE);
                d.draw_text("¡Felicidades, estrellita JAJA :3!", 380, 380, 28, Color::LIGHTGRAY);
                d.draw_text("Presiona M para menú", 450, 500, 24, Color::SKYBLUE);
                
                drop(d);
                
                if key_m_pressed {
                    state = GameState::Menu;
                }
            }

            GameState::GameOver => {
                let key_r_pressed = rl.is_key_pressed(KeyboardKey::KEY_R);
                let key_m_pressed = rl.is_key_pressed(KeyboardKey::KEY_M);
                
                let mut d = rl.begin_drawing(&thread);
                d.draw_rectangle_gradient_v(0, 0, window_width, window_height, Color::new(60, 0, 0, 255), Color::new(20, 0, 0, 255));
                d.draw_text("FIN DEL JUEGO", 300, 220, 80, Color::RED);
                d.draw_text("Has sido derrotado...", 380, 340, 32, Color::WHITE);
                d.draw_text("Presiona R para reiniciar", 420, 450, 26, Color::LIGHTGRAY);
                d.draw_text("Presiona M para menú", 450, 490, 24, Color::GRAY);
                
                drop(d);

                if key_r_pressed {
                    let (nx, ny) = find_player_start(&maze).unwrap();
                    player.pos.x = nx;
                    player.pos.y = ny;
                    player.health = 100;
                    damage_overlay_alpha = 0.0;
                    
                    enemies = find_positions_in_maze(&maze, 'F', block_size)
                        .iter()
                        .map(|(x, y)| Enemy::new(*x, *y))
                        .collect();
                    
                    state = GameState::Playing;
                }
                
                if key_m_pressed {
                    state = GameState::Menu;
                }
            }
        }
    }
}