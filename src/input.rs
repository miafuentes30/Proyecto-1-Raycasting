use raylib::prelude::*;
use crate::player::Player;
use crate::maze::Maze;

pub fn process_events(
    window: &RaylibHandle,
    player: &mut Player,
    maze: &Maze,
    _block_size: usize,
    mouse_dx: f32,
) -> bool {
    const MOVE_SPEED: f32 = 4.0;
    const ROTATION_SPEED_KEY: f32 = 0.04;
    const MOUSE_SENSITIVITY: f32 = 0.002;

    let mut level_changed = false;

    // Rotación por teclado
    if window.is_key_down(KeyboardKey::KEY_LEFT) {
        player.rotate(-ROTATION_SPEED_KEY);
    }
    if window.is_key_down(KeyboardKey::KEY_RIGHT) {
        player.rotate(ROTATION_SPEED_KEY);
    }

    // Rotación por mouse
    if mouse_dx.abs() > 0.0 {
        player.rotate(mouse_dx * MOUSE_SENSITIVITY);
    }

    // Movimiento
    if window.is_key_down(KeyboardKey::KEY_UP) || window.is_key_down(KeyboardKey::KEY_W) {
        level_changed = player.move_forward(MOVE_SPEED, maze);
    }
    if window.is_key_down(KeyboardKey::KEY_DOWN) || window.is_key_down(KeyboardKey::KEY_S) {
        level_changed = player.move_backward(MOVE_SPEED, maze) || level_changed;
    }

    // Strafe izquierda/derecha
    if window.is_key_down(KeyboardKey::KEY_A) {
        let strafe_angle = player.a - std::f32::consts::FRAC_PI_2;
        let nx = player.pos.x + MOVE_SPEED * strafe_angle.cos();
        let ny = player.pos.y + MOVE_SPEED * strafe_angle.sin();
        level_changed = player.try_move(nx, ny, maze) || level_changed;
    }
    if window.is_key_down(KeyboardKey::KEY_D) {
        let strafe_angle = player.a + std::f32::consts::FRAC_PI_2;
        let nx = player.pos.x + MOVE_SPEED * strafe_angle.cos();
        let ny = player.pos.y + MOVE_SPEED * strafe_angle.sin();
        level_changed = player.try_move(nx, ny, maze) || level_changed;
    }

    level_changed
}