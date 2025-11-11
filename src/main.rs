use std::time::{Duration, Instant};
use pixels::{Error, Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{DeviceEvent, ElementState, Event, WindowEvent, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

mod util;
mod map;
mod player;
mod raycaster;
mod draw;

use util::{WIDTH, HEIGHT, Effects};
use map::Map;
use player::Player;
use raycaster::Raycaster;
use draw::{draw_minimap, draw_fps, draw_life, draw_torch_anim, draw_menu, draw_win, draw_damage_overlay};

use rodio::Source;

#[derive(Default, Clone, Copy)]
struct PlayerInput {
    forward: bool,
    backward: bool,
    left: bool,
    right: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameState {
    Menu { selected: usize },
    Playing,
    Win,
}

// verifica si el jugador puede moverse a esta posición
fn can_move_to(map: &Map, x: f32, y: f32) -> bool {
    let tile = map.at(x.floor() as i32, y.floor() as i32);
    tile == 0 || tile == 5 || tile == 9  // vacío, lava o meta
}

struct Audio {
    _stream: rodio::OutputStream,
    stream_handle: rodio::OutputStreamHandle,
    music_sink: rodio::Sink,
}

impl Audio {
    fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (_stream, stream_handle) = rodio::OutputStream::try_default()?;
        let music_sink = rodio::Sink::try_new(&stream_handle)?;
        
        Ok(Audio {
            _stream,
            stream_handle,
            music_sink,
        })
    }
    
    fn play_music(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.music_sink.stop();
        
        let file = std::fs::File::open(file_path)?;
        let source = rodio::Decoder::new(std::io::BufReader::new(file))?.repeat_infinite();
        self.music_sink.append(source);
        self.music_sink.set_volume(0.3);
        Ok(())
    }
    
    fn play_ding(&self) {
        if let Ok(file) = std::fs::File::open("assets/audio/ding.wav") {
            if let Ok(source) = rodio::Decoder::new(std::io::BufReader::new(file)) {
                let sink = rodio::Sink::try_new(&self.stream_handle).unwrap();
                sink.append(source);
                sink.set_volume(0.7);
                sink.detach();
            }
        }
    }
    
    fn play_dead(&self) {
        if let Ok(file) = std::fs::File::open("assets/audio/dead.wav") {
            if let Ok(source) = rodio::Decoder::new(std::io::BufReader::new(file)) {
                let sink = rodio::Sink::try_new(&self.stream_handle).unwrap();
                sink.append(source);
                sink.set_volume(0.8);
                sink.detach();
            }
        }
    }
    
    fn set_music_volume(&self, volume: f32) {
        self.music_sink.set_volume(volume);
    }
    
    fn stop_music(&self) {
        self.music_sink.stop();
    }
    
    fn pause_music(&self) {
        self.music_sink.pause();
    }
    
    fn resume_music(&self) {
        self.music_sink.play();
    }
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Raycaster :3")
        .with_inner_size(LogicalSize::new(WIDTH as f64, HEIGHT as f64))
        .with_resizable(false)
        .build(&event_loop)
        .unwrap();

    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels = Pixels::new(WIDTH, HEIGHT, surface_texture)?;

    let audio = Audio::new().unwrap_or_else(|e| {
        eprintln!("Error inicializando audio: {}. Continuando sin sonido.", e);
        if let Ok((_stream, stream_handle)) = rodio::OutputStream::try_default() {
            if let Ok(music_sink) = rodio::Sink::try_new(&stream_handle) {
                return Audio {
                    _stream,
                    stream_handle,
                    music_sink,
                };
            }
        }
        panic!("No se pudo inicializar el audio");
    });

    let mut map = Map::level1();
    let mut player = Player::new(1.5, 1.5);
    player.ang = 0.0;

    let mut rc = Raycaster::new();
    let mut fx = Effects::default();

    let mut input = PlayerInput::default();
    let mut state = GameState::Menu { selected: 1 };
    let mut life: i32 = 5;
    let mut damage_cooldown = 0.0f32;
    let mut previous_life = life;
    let mut music_started = false;

    // timing
    let mut last = Instant::now();
    let mut fps_counter_last = Instant::now();
    let mut frames: u32 = 0;
    let mut fps: u32 = 0;
    let mut ui_time_start = Instant::now();

    let fov: f32 = std::f32::consts::FRAC_PI_3;
    let mut mouse_captured = false;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }

                WindowEvent::KeyboardInput { input: key_input, .. } => {
                    let pressed = key_input.state == ElementState::Pressed;
                    if let Some(key) = key_input.virtual_keycode {
                        match state {
                            GameState::Menu { ref mut selected } => {
                                if pressed {
                                    match key {
                                        VirtualKeyCode::Key1 | VirtualKeyCode::Numpad1 => {
                                            *selected = 1;
                                        }
                                        VirtualKeyCode::Key2 | VirtualKeyCode::Numpad2 => {
                                            *selected = 2;
                                        }
                                        VirtualKeyCode::Return => {
                                            // cargar mapa según nivel elegido
                                            match *selected {
                                                1 => {
                                                    map = Map::level1();
                                                    player = Player::new(1.5, 1.5);
                                                }
                                                2 => {
                                                    map = Map::level2();
                                                    player = Player::new(1.5, 1.5);
                                                }
                                                _ => {
                                                    map = Map::level1();
                                                    player = Player::new(1.5, 1.5);
                                                }
                                            }
                                            player.ang = 0.0;
                                            life = 5;
                                            previous_life = life;
                                            damage_cooldown = 0.0;
                                            
                                            input = PlayerInput::default();
                                            
                                            state = GameState::Playing;
                                            ui_time_start = Instant::now();
                                            mouse_captured = true;
                                            window.set_cursor_visible(false);
                                            
                                            if !music_started {
                                                if let Err(e) = audio.play_music("assets/audio/music.ogg") {
                                                    eprintln!("Error reproduciendo música: {}", e);
                                                } else {
                                                    music_started = true;
                                                }
                                            } else {
                                                audio.resume_music();
                                            }
                                        }
                                        VirtualKeyCode::Escape => {
                                            *control_flow = ControlFlow::Exit;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            GameState::Playing => {
                                match key {
                                    VirtualKeyCode::W => input.forward  = pressed,
                                    VirtualKeyCode::S => input.backward = pressed,
                                    VirtualKeyCode::A => input.left     = pressed,
                                    VirtualKeyCode::D => input.right    = pressed,

                                    VirtualKeyCode::Escape if pressed => {
                                        state = GameState::Menu { selected: 1 };
                                        mouse_captured = false;
                                        window.set_cursor_visible(true);
                                        input = PlayerInput::default();
                                        audio.pause_music();
                                    }

                                    VirtualKeyCode::Tab if pressed => {
                                        mouse_captured = !mouse_captured;
                                        window.set_cursor_visible(!mouse_captured);
                                    }

                                    _ => {}
                                }
                            }
                            GameState::Win => {
                                if pressed {
                                    match key {
                                        VirtualKeyCode::Space => {
                                            state = GameState::Menu { selected: 1 };
                                            mouse_captured = false;
                                            window.set_cursor_visible(true);
                                            input = PlayerInput::default();
                                        }
                                        VirtualKeyCode::Escape => {
                                            *control_flow = ControlFlow::Exit;
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }

                WindowEvent::Resized(_size) => {
                    window.request_redraw();
                }

                _ => {}
            },

            Event::DeviceEvent { event, .. } => {
                if let GameState::Playing = state {
                    if let DeviceEvent::MouseMotion { delta: (dx, _dy) } = event {
                        if mouse_captured {
                            let sens = 0.003;
                            player.ang += (dx as f32) * sens;
                            if player.ang > std::f32::consts::PI {
                                player.ang -= 2.0 * std::f32::consts::PI;
                            }
                            if player.ang < -std::f32::consts::PI {
                                player.ang += 2.0 * std::f32::consts::PI;
                            }
                        }
                    }
                }
            }

            Event::RedrawRequested(_) => {
                let now = Instant::now();
                let dt = (now - last).as_secs_f32().clamp(0.0, 0.05);
                last = now;

                if damage_cooldown > 0.0 {
                    damage_cooldown -= dt;
                }

                let frame = pixels.frame_mut();

                match state {
                    GameState::Menu { selected } => {
                        let t = ui_time_start.elapsed().as_secs_f32();
                        draw_menu(frame, selected, t);
                    }
                    GameState::Playing => {
                        // procesar input
                        let mut forward = 0.0f32;
                        let mut side = 0.0f32;
                        if input.forward  { forward += 1.0; }
                        if input.backward { forward -= 1.0; }
                        if input.right    { side    += 1.0; }
                        if input.left     { side    -= 1.0; }

                        if forward != 0.0 || side != 0.0 {
                            let len = (forward * forward + side * side).sqrt();
                            if len > 0.0 {
                                forward /= len;
                                side /= len;
                            }
                        }

                        let speed = 3.0;
                        let dx = player.ang.cos();
                        let dy = player.ang.sin();

                        let step_f = forward * speed * dt;
                        let step_s = side * speed * dt;

                        let nx = player.x + dx * step_f + dy * step_s;
                        let ny = player.y + dy * step_f - dx * step_s;

                        // colisiones con sliding
                        if can_move_to(&map, nx, player.y) {
                            player.x = nx;
                        } else {
                            let small_step_x = player.x + dx * step_f * 0.1 + dy * step_s * 0.1;
                            if can_move_to(&map, small_step_x, player.y) {
                                player.x = small_step_x;
                            }
                        }

                        if can_move_to(&map, player.x, ny) {
                            player.y = ny;
                        } else {
                            let small_step_y = player.y + dy * step_f * 0.1 - dx * step_s * 0.1;
                            if can_move_to(&map, player.x, small_step_y) {
                                player.y = small_step_y;
                            }
                        }

                        let current_tile = map.at(player.x.floor() as i32, player.y.floor() as i32);
                        
                        // lava = daño
                        if current_tile == 5 && damage_cooldown <= 0.0 && life > 0 {
                            life -= 1;
                            damage_cooldown = 0.5;
                            
                            if life < previous_life {
                                audio.play_dead();
                                previous_life = life;
                            }
                        }

                        // llegaste a la meta
                        if current_tile == 9 {
                            audio.play_ding();
                            state = GameState::Win;
                            ui_time_start = Instant::now();
                            mouse_captured = false;
                            window.set_cursor_visible(true);
                            input = PlayerInput::default();
                            audio.pause_music();
                        }

                        // muerte = respawn en punto seguro
                        if life <= 0 {
                            let mut safe_x = 1.5;
                            let mut safe_y = 1.5;
                            
                            for offset_x in -1..=1 {
                                for offset_y in -1..=1 {
                                    let test_x = 1.5 + offset_x as f32;
                                    let test_y = 1.5 + offset_y as f32;
                                    if can_move_to(&map, test_x, test_y) {
                                        safe_x = test_x;
                                        safe_y = test_y;
                                        break;
                                    }
                                }
                            }
                            
                            player.x = safe_x;
                            player.y = safe_y;
                            player.ang = 0.0;
                            life = 5;
                            previous_life = life;
                            damage_cooldown = 1.0;
                            input = PlayerInput::default();
                        }

                        fx.time = ui_time_start.elapsed().as_secs_f32();

                        // render 3D
                        rc.render(frame, &map, player.x, player.y, player.ang, fov, fx);

                        // UI
                        draw_minimap(frame, &map, (player.x, player.y, player.ang));
                        draw_fps(frame, fps);
                        draw_life(frame, life);
                        let t = ui_time_start.elapsed().as_secs_f32();
                        draw_torch_anim(frame, t);

                        if damage_cooldown > 0.0 {
                            draw_damage_overlay(frame, damage_cooldown);
                        }
                    }
                    GameState::Win => {
                        let t = ui_time_start.elapsed().as_secs_f32();
                        draw_win(frame, t);
                    }
                }

                // actualizar FPS
                frames += 1;
                if fps_counter_last.elapsed() >= Duration::from_secs(1) {
                    fps = frames;
                    frames = 0;
                    fps_counter_last = Instant::now();
                }

                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }

            Event::MainEventsCleared => {
                window.request_redraw();
            }

            _ => {}
        }
    });
}