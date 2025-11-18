use raylib::prelude::*;
use crate::player::Player;
use crate::maze::Maze;
use crate::enemy::Enemy;

/// HUD: vida, FPS, cofres
pub fn draw_hud(d: &mut RaylibDrawHandle, player: &Player, fps: u32, show_fps: bool, chests_collected: usize, total_chests: usize) {
    let screen_width = d.get_screen_width();
    let screen_height = d.get_screen_height();

    // Barra de vida
    let health_bar_width = 200;
    let health_bar_height = 20;
    let health_x = 20;
    let health_y = screen_height - 40;

    d.draw_rectangle(health_x, health_y, health_bar_width, health_bar_height, Color::DARKGRAY);
    
    let health_percentage = player.health as f32 / 100.0;
    let current_health_width = (health_bar_width as f32 * health_percentage) as i32;
    
    let health_color = if health_percentage > 0.6 {
        Color::GREEN
    } else if health_percentage > 0.3 {
        Color::YELLOW
    } else {
        Color::RED
    };
    
    d.draw_rectangle(health_x, health_y, current_health_width, health_bar_height, health_color);
    d.draw_rectangle_lines(health_x, health_y, health_bar_width, health_bar_height, Color::WHITE);
    
    d.draw_text(
        &format!("Vida: {}/100", player.health),
        health_x + 5,
        health_y + 3,
        14,
        Color::WHITE,
    );

    // Cofres recolectados
    let chest_text = format!("Cofres: {}/{}", chests_collected, total_chests);
    let chest_color = if chests_collected == total_chests {
        Color::GOLD
    } else {
        Color::WHITE
    };
    d.draw_text(&chest_text, health_x, health_y - 30, 20, chest_color);

    // FPS
    if show_fps {
        d.draw_text(
            &format!("FPS: {}", fps),
            screen_width - 100,
            10,
            20,
            Color::LIME,
        );
    }

    // Controles
    d.draw_text("WASD/Flechas (arriba y abajo): Mover | Mouse/Flechas (izquierda y derecha): Mirar | ESC: Menú", 
                screen_width / 2 - 200, screen_height - 20, 12, Color::LIGHTGRAY);
}

/// Minimapa superior derecha
pub fn draw_minimap(
    d: &mut RaylibDrawHandle,
    maze: &Maze,
    player: &Player,
    enemies: &[Enemy],
    block_size: usize,
) {
    let minimap_size = 200;
    let screen_width = d.get_screen_width();
    let minimap_x = screen_width - minimap_size - 20;
    let minimap_y = 20;

    // Fondo
    d.draw_rectangle(minimap_x, minimap_y, minimap_size, minimap_size, Color::new(0, 0, 0, 180));
    d.draw_rectangle_lines(minimap_x, minimap_y, minimap_size, minimap_size, Color::WHITE);

    // Escala del maze
    let maze_width = maze[0].len() * block_size;
    let maze_height = maze.len() * block_size;
    let scale = minimap_size as f32 / maze_width.max(maze_height) as f32;

    // Celdas del maze
    for (row_idx, row) in maze.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            let x = minimap_x as f32 + (col_idx * block_size) as f32 * scale;
            let y = minimap_y as f32 + (row_idx * block_size) as f32 * scale;
            let w = (block_size as f32 * scale).max(1.0);

            let color = match cell {
                '#' | 'L' | '$' => Color::GRAY,
                'E' => Color::GREEN,
                'F' => Color::RED,
                'C' => Color::GOLD,
                _ => Color::DARKGRAY,
            };

            d.draw_rectangle(x as i32, y as i32, w as i32, w as i32, color);
        }
    }

    // Jugador (círculo rojo)
    let player_x = minimap_x as f32 + player.pos.x * scale;
    let player_y = minimap_y as f32 + player.pos.y * scale;
    d.draw_circle(player_x as i32, player_y as i32, 3.0, Color::RED);

    // Enemigos (círculos naranja; gris si inactivos)
    for e in enemies.iter() {
        let ex = minimap_x as f32 + e.pos.x * scale;
        let ey = minimap_y as f32 + e.pos.y * scale;
        let color = if e.active { Color::ORANGE } else { Color::GRAY };
        d.draw_circle(ex as i32, ey as i32, 3.0, color);
    }

    // Dirección
    let dir_x = player_x + player.a.cos() * 10.0;
    let dir_y = player_y + player.a.sin() * 10.0;
    d.draw_line(player_x as i32, player_y as i32, dir_x as i32, dir_y as i32, Color::YELLOW);
}

/// Pantalla de menú
pub fn draw_menu_screen(d: &mut RaylibDrawHandle, current_level: usize, total_levels: usize) {
    let w = d.get_screen_width();
    let h = d.get_screen_height();

    // Fondo degradado
    d.draw_rectangle_gradient_v(0, 0, w, h, Color::new(12, 18, 38, 255), Color::new(3, 3, 8, 255));

    // Marco central
    let panel_w = (w as f32 * 0.66) as i32;
    let panel_h = (h as f32 * 0.60) as i32;
    let panel_x = (w - panel_w) / 2;
    let panel_y = (h - panel_h) / 2;
    d.draw_rectangle(panel_x - 6, panel_y - 6, panel_w + 12, panel_h + 12, Color::new(0, 0, 0, 120));
    d.draw_rectangle(panel_x, panel_y, panel_w, panel_h, Color::new(12, 12, 20, 200));
    d.draw_rectangle_lines(panel_x, panel_y, panel_w, panel_h, Color::new(80, 80, 120, 180));

    // Título
    let title = "MAZMORRA RAYCASTER";
    let tw = d.measure_text(title, 56);
    d.draw_text(title, (w - tw) / 2, panel_y + 40, 56, Color::SKYBLUE);

    // Subtítulo
    let subtitle = "Selecciona Nivel (1 o 2)";
    let stw = d.measure_text(subtitle, 24);
    d.draw_text(subtitle, (w - stw) / 2, panel_y + 120, 24, Color::LIGHTGRAY);

    // Nivel seleccionado
    let current = format!("Nivel {}/{}", current_level + 1, total_levels);
    let cw = d.measure_text(&current, 30);
    d.draw_text(&current, (w - cw) / 2, panel_y + 160, 30, Color::YELLOW);

    // Cómo jugar
    let how_title = "CÓMO JUGAR:";
    let how_title_w = d.measure_text(how_title, 18);
    d.draw_text(how_title, (w - how_title_w) / 2, panel_y + 210, 18, Color::ORANGE);
    
    let instructions = vec![
        ("WASD/Flechas (arriba y abajo): Mover | Mouse/Flechas (izquierda y derecha): Mirar", 14, Color::LIGHTGRAY),
        ("F: Linterna  |  Objetivo: Recoger todos los cofres", 14, Color::LIGHTGRAY),
        ("Luego encuentra la salida 'E' (verde en mapa)", 14, Color::YELLOW),
    ];
    
    let mut y_offset = panel_y + 238;
    for (text, size, color) in instructions {
        let text_w = d.measure_text(text, size);
        d.draw_text(text, (w - text_w) / 2, y_offset, size, color);
        y_offset += 22;
    }
    
    // Instrucciones de inicio
    let i1 = "Presiona ENTER para comenzar";
    let i2 = "Presiona ESC para salir";
    let i1w = d.measure_text(i1, 22);
    let i2w = d.measure_text(i2, 18);
    d.draw_text(i1, (w - i1w) / 2, panel_y + panel_h - 70, 22, Color::GREEN);
    d.draw_text(i2, (w - i2w) / 2, panel_y + panel_h - 40, 18, Color::GRAY);

    // Pie de página
    let footer = "Proyecto Gráficas";
    let fw = d.measure_text(footer, 16);
    d.draw_text(footer, (w - fw) / 2, h - 28, 16, Color::GRAY);
}

/// Viñeta en bordes
pub fn draw_vignette(d: &mut RaylibDrawHandle, strength: f32) {
    let w = d.get_screen_width();
    let h = d.get_screen_height();
    let alpha = (strength.clamp(0.0, 1.0) * 140.0) as u8;

    // Esquinas
    d.draw_rectangle(0, 0, w, 20, Color::new(0, 0, 0, alpha));
    d.draw_rectangle(0, h - 20, w, 20, Color::new(0, 0, 0, alpha));
    d.draw_rectangle(0, 0, 20, h, Color::new(0, 0, 0, alpha));
    d.draw_rectangle(w - 20, 0, 20, h, Color::new(0, 0, 0, alpha));

    // Bordes graduales
    for i in 0..40 {
        let a = ((i as f32 / 40.0) * alpha as f32) as u8;
        d.draw_rectangle(20 + i, 20 + i, w - 2 * (20 + i), 1, Color::new(0, 0, 0, a));
        d.draw_rectangle(20 + i, h - 21 - i, w - 2 * (20 + i), 1, Color::new(0, 0, 0, a));
    }
}

/// Efecto de linterna
pub fn draw_flashlight_overlay(d: &mut RaylibDrawHandle, battery: f32) {
    let w = d.get_screen_width();
    let h = d.get_screen_height();

    // Oscurecer pantalla
    d.draw_rectangle(0, 0, w, h, Color::new(0, 0, 0, 150));

    // Luz central (más batería = más luz)
    let center_x = (w / 2) as f32;
    let center_y = (h / 2) as f32;
    let radius = 120.0 + 180.0 * (battery / 100.0);
    let steps = 24;
    for i in 0..steps {
        let t = i as f32 / steps as f32;
        let r = radius * (1.0 - t);
        let a = (110.0 * (1.0 - t)) as u8;
        d.draw_circle_lines(center_x as i32, center_y as i32, r.max(1.0), Color::new(0, 0, 0, a));
    }
}