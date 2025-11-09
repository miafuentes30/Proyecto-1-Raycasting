use std::collections::HashSet;

use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{
    DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent,
};
use winit::event_loop::{ControlFlow, EventLoop};

mod util;
mod map;
mod player;
mod raycaster;
mod draw;

use util::*;

fn main() {
    env_logger::init();

    // Ventana
    let event_loop = EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Raycaster UVG")
        .with_inner_size(LogicalSize::new(WIDTH as f64, HEIGHT as f64))
        .build(&event_loop)
        .expect("No se pudo crear la ventana");

    // pixels
    let surface_texture = SurfaceTexture::new(WIDTH, HEIGHT, &window);
    let mut pixels =
        Pixels::new(WIDTH, HEIGHT, surface_texture).expect("No se pudo crear Pixels");

    // Estado del juego
    let map = map::Map::demo();
    let mut player = player::Player::new(2.5, 2.5);
    player.fov = util::deg2rad(60.0);
    let mut rc = raycaster::Raycaster::new();

    // Mouse 
    let mut mouse_captured = true;
    window.set_cursor_visible(false);

    // Teclado 
    let mut keys_down: HashSet<VirtualKeyCode> = HashSet::new();

    // FPS
    let mut last = std::time::Instant::now();
    let mut acc = 0.0f32;
    let mut frames = 0u32;
    let mut fps = 0u32;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::DeviceEvent {
                event: DeviceEvent::MouseMotion { delta },
                ..
            } => {
                if mouse_captured {
                    player.add_yaw(delta.0 as f32);
                }
            }

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state,
                            virtual_keycode: Some(key),
                            ..
                        },
                    ..
                } => {
                    // ESC del mouse
                    if key == VirtualKeyCode::Escape && state == ElementState::Pressed {
                        mouse_captured = !mouse_captured;
                        window.set_cursor_visible(!mouse_captured);
                    }
                    // Set de teclas presionadas
                    match state {
                        ElementState::Pressed => {
                            keys_down.insert(key);
                        }
                        ElementState::Released => {
                            keys_down.remove(&key);
                        }
                    }
                }
                WindowEvent::Resized(size) => {
                    if let Err(e) = pixels.resize_surface(size.width, size.height) {
                        eprintln!("resize_surface error: {e}");
                    }
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    if let Err(e) = pixels.resize_surface(new_inner_size.width, new_inner_size.height)
                    {
                        eprintln!("resize_surface error: {e}");
                    }
                }
                _ => {}
            },

            Event::RedrawRequested(_id) => {
                // Tiempo / FPS
                let now = std::time::Instant::now();
                let dt = (now - last).as_secs_f32();
                last = now;
                acc += dt;
                frames += 1;
                if acc >= 1.0 {
                    fps = frames;
                    frames = 0;
                    acc = 0.0;
                    window.set_title(&format!("Raycaster UVG – {} FPS", fps));
                }

                // Input de teclado 
                let forward = (player.ang.cos(), player.ang.sin());
                let right_ang = player.ang + std::f32::consts::FRAC_PI_2;
                let right_v = (right_ang.cos(), right_ang.sin());

                let mut dir_x = 0.0f32;
                let mut dir_y = 0.0f32;

                if keys_down.contains(&VirtualKeyCode::W) {
                    dir_x += forward.0; dir_y += forward.1;
                }
                if keys_down.contains(&VirtualKeyCode::S) {
                    dir_x -= forward.0; dir_y -= forward.1;
                }
                if keys_down.contains(&VirtualKeyCode::A) {
                    dir_x -= right_v.0; dir_y -= right_v.1;
                }
                if keys_down.contains(&VirtualKeyCode::D) {
                    dir_x += right_v.0; dir_y += right_v.1;
                }

                let len = (dir_x * dir_x + dir_y * dir_y).sqrt();
                if len > 0.0 {
                    dir_x /= len; dir_y /= len;
                }
                player.try_move(dir_x, dir_y, dt, &map);

                // Render
                let frame = pixels.frame_mut();
                rc.render(frame, &map, player.x, player.y, player.ang, player.fov);
                draw::draw_minimap(frame, &map, (player.x, player.y, player.ang));
                draw::draw_fps(frame, fps);

                if let Err(e) = pixels.render() {
                    eprintln!("pixels.render error: {e}");
                    *control_flow = ControlFlow::Exit;
                }
            }

            // Bucle de 60 FPS
            Event::MainEventsCleared => {
                window.request_redraw();
            }

            _ => {}
        }
    });
}
