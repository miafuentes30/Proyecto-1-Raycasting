use crate::util::{WIDTH, HEIGHT};

pub fn put(frame: &mut [u8], x: i32, y: i32, rgb: [u8;3]) {
    if x<0 || y<0 { return; }
    let (x,y) = (x as u32, y as u32);
    if x>=WIDTH || y>=HEIGHT { return; }
    let i = ((y*WIDTH + x)*4) as usize;
    frame[i..i+3].copy_from_slice(&rgb);
    frame[i+3] = 255;
}

pub fn rect(frame:&mut [u8], x:i32,y:i32,w:i32,h:i32, rgb:[u8;3]) {
    for yy in y..y+h { for xx in x..x+w { put(frame, xx, yy, rgb); } }
}

fn blend_color(c1: [u8;3], c2: [u8;3], t: f32) -> [u8;3] {
    [
        (c1[0] as f32 * (1.0-t) + c2[0] as f32 * t) as u8,
        (c1[1] as f32 * (1.0-t) + c2[1] as f32 * t) as u8,
        (c1[2] as f32 * (1.0-t) + c2[2] as f32 * t) as u8,
    ]
}

pub fn gradient_vertical(frame:&mut [u8], x:i32, y:i32, w:i32, h:i32, c1:[u8;3], c2:[u8;3]) {
    for yy in 0..h {
        let t = yy as f32 / h.max(1) as f32;
        let color = blend_color(c1, c2, t);
        rect(frame, x, y+yy, w, 1, color);
    }
}

pub fn draw_minimap(frame:&mut [u8], map:&crate::map::Map, pl:(f32,f32,f32)) {
    let (px,py,ang) = pl;
    let scale = 8;
    let margin = 20;
    let padding = 10;
    
    let map_w = (map.w * scale) as i32;
    let map_h = (map.h * scale) as i32;
    
    // sombra difuminada
    for offset in 0..6 {
        let alpha = (40 - offset * 6) as u8;
        let shadow = [0, 0, 0];
        draw_rect_alpha(frame, margin + offset, margin + offset, map_w + padding*2, map_h + padding*2, shadow, alpha);
    }
    
    // fondo + borde
    gradient_vertical(frame, margin, margin, map_w + padding*2, map_h + padding*2, [25, 28, 35], [18, 20, 26]);
    draw_border(frame, margin, margin, map_w + padding*2, map_h + padding*2, [100, 110, 130], 2);
    
    // tiles del mapa
    for y in 0..map.h {
        for x in 0..map.w {
            let id = map.at(x as i32, y as i32);
            let c = match id {
                0 => [22, 24, 28],
                1 => [200, 70, 70],
                2 => [70, 190, 85],
                3 => [70, 120, 210],
                4 => [210, 190, 70],
                5 => [255, 130, 50],
                9 => [200, 200, 255],
                _ => [80, 80, 90],
            };
            
            let tx = margin + padding + (x * scale) as i32;
            let ty = margin + padding + (y * scale) as i32;
            
            rect(frame, tx, ty, scale as i32, scale as i32, c);
            
            // borde oscuro para dar profundidad
            if id != 0 {
                let darker = [
                    (c[0] as f32 * 0.7) as u8,
                    (c[1] as f32 * 0.7) as u8,
                    (c[2] as f32 * 0.7) as u8,
                ];
                rect(frame, tx, ty, scale as i32, 1, darker);
                rect(frame, tx, ty, 1, scale as i32, darker);
            }
        }
    }
    
    // jugador
    let player_size = (scale - 2) as i32;
    let px_screen = margin + padding + ((px as usize) * scale) as i32 + 1;
    let py_screen = margin + padding + ((py as usize) * scale) as i32 + 1;
    
    rect(frame, px_screen - 1, py_screen - 1, player_size + 2, player_size + 2, [180, 80, 255]);
    rect(frame, px_screen, py_screen, player_size, player_size, [230, 120, 255]);
    
    // dirección
    let ray_len = scale * 2;
    let cx = px_screen + player_size / 2;
    let cy = py_screen + player_size / 2;
    let ex = cx + (ang.cos() * ray_len as f32) as i32;
    let ey = cy + (ang.sin() * ray_len as f32) as i32;
    
    line_thick(frame, cx, cy, ex, ey, [255, 230, 255], 2);
}

pub fn draw_fps(frame: &mut [u8], fps: u32) {
    let s = format!("FPS: {}", fps);
    let x0 = 20; 
    let y0 = HEIGHT as i32 - 50;
    let w = 110;
    let h = 40;
    
    for i in 0..4 {
        draw_rect_alpha(frame, x0 + i, y0 + i, w, h, [0,0,0], 30);
    }
    
    gradient_vertical(frame, x0, y0, w, h, [15, 18, 25], [10, 12, 18]);
    draw_border(frame, x0, y0, w, h, [60, 70, 90], 2);
    
    // colores según performance
    let color = if fps >= 55 { [120, 255, 140] } 
                else if fps >= 40 { [255, 230, 120] }
                else { [255, 120, 120] };
    
    draw_text(frame, &s, x0 + 10, y0 + 12, color, 2);
}

pub fn draw_life(frame:&mut [u8], life:i32) {
    let x0 = WIDTH as i32 - 220;
    let y0 = 20;
    let w = 200;
    let h = 50;
    
    for i in 0..4 {
        draw_rect_alpha(frame, x0 + i, y0 + i, w, h, [0,0,0], 30);
    }
    
    gradient_vertical(frame, x0, y0, w, h, [30, 15, 18], [20, 10, 12]);
    draw_border(frame, x0, y0, w, h, [120, 60, 70], 2);
    
    // los 5 corazones
    for i in 0..5 {
        let x = x0 + 15 + i * 36;
        let y = y0 + 10;
        
        if i < life {
            draw_heart_filled(frame, x, y, [240, 70, 90]);
        } else {
            draw_heart_empty(frame, x, y, [60, 60, 65]);
        }
    }
}

fn draw_heart_filled(frame: &mut [u8], x: i32, y: i32, color: [u8;3]) {
    let shadow = [(color[0] as f32 * 0.5) as u8, (color[1] as f32 * 0.5) as u8, (color[2] as f32 * 0.5) as u8];
    let highlight = [255, (color[1] as f32 * 1.3).min(255.0) as u8, (color[2] as f32 * 1.3).min(255.0) as u8];
    
    let pattern = [
        "  ###  ###  ",
        " ####  #### ",
        "############",
        "############",
        " ########## ",
        "  ########  ",
        "   ######   ",
        "    ####    ",
        "     ##     ",
    ];
    
    for (dy, row) in pattern.iter().enumerate() {
        for (dx, ch) in row.chars().enumerate() {
            if ch == '#' {
                let c = if dy < 3 && dx > 2 && dx < 6 { highlight }
                       else if dy > 5 { shadow }
                       else { color };
                put(frame, x + dx as i32 * 2, y + dy as i32 * 3, c);
                put(frame, x + dx as i32 * 2 + 1, y + dy as i32 * 3, c);
                put(frame, x + dx as i32 * 2, y + dy as i32 * 3 + 1, c);
                put(frame, x + dx as i32 * 2 + 1, y + dy as i32 * 3 + 1, c);
            }
        }
    }
}

fn draw_heart_empty(frame: &mut [u8], x: i32, y: i32, color: [u8;3]) {
    let pattern = [
        "  ###  ###  ",
        " #  #  #  # ",
        "#          #",
        "#          #",
        " #        # ",
        "  #      #  ",
        "   #    #   ",
        "    #  #    ",
        "     ##     ",
    ];
    
    for (dy, row) in pattern.iter().enumerate() {
        for (dx, ch) in row.chars().enumerate() {
            if ch == '#' {
                put(frame, x + dx as i32 * 2, y + dy as i32 * 3, color);
                put(frame, x + dx as i32 * 2 + 1, y + dy as i32 * 3, color);
            }
        }
    }
}

pub fn draw_torch_anim(frame:&mut [u8], t: f32) {
    let x0 = WIDTH as i32 / 2;
    let y0 = HEIGHT as i32 - 120;
    
    // palo de la antorcha
    let wood_w = 16;
    let wood_h = 80;
    let wood_x = x0 - wood_w / 2;
    let wood_y = y0 + 40;
    
    gradient_vertical(frame, wood_x, wood_y, wood_w, wood_h, [100, 75, 50], [70, 50, 35]);
    
    // vetas
    for i in (0..wood_h).step_by(8) {
        rect(frame, wood_x, wood_y + i, wood_w, 2, [60, 45, 30]);
    }
    rect(frame, wood_x - 2, wood_y - 8, wood_w + 4, 8, [90, 70, 45]);
    
    // llama con 3 capas
    for layer in 0..3 {
        let layer_offset = layer as f32 * 0.3;
        for i in 0..30 {
            let phase = t * 8.0 + i as f32 * 0.25 + layer_offset;
            let flicker = (phase.sin() * 6.0 + (phase * 2.0).cos() * 3.0) as i32;
            let sway = ((phase * 0.5).sin() * 4.0) as i32;
            
            let height_base = 30 - i as i32;
            let height = height_base + flicker / 2;
            let width = ((6.0 - i as f32 * 0.15).max(1.0)) as i32;
            
            let i_norm = i as f32 / 30.0;
            let r = 255;
            let g = (220.0 - i_norm * 120.0) as u8;
            let b = (50.0 + i_norm * 30.0) as u8;
            
            let alpha = (255.0 * (1.0 - i_norm * 0.7)) as u8;
            
            let px = x0 + sway - width / 2;
            let py = y0 + 40 - height;
            
            if layer == 0 {
                draw_rect_alpha(frame, px, py, width, height.max(2), [255, 255, 180], alpha);
            } else if layer == 1 {
                draw_rect_alpha(frame, px - 1, py - 2, width + 2, height.max(2) + 2, [r, g, b], alpha);
            } else {
                draw_rect_alpha(frame, px - 2, py - 4, width + 4, height.max(2) + 4, [(r as f32 * 0.8) as u8, (g as f32 * 0.6) as u8, b/2], alpha / 2);
            }
        }
    }
    
    // chispas que suben
    for i in 0..15 {
        let offset = (t * 15.0 + i as f32 * 2.5) % 8.0;
        let spark_x = x0 + ((t * 3.0 + i as f32).sin() * 8.0) as i32;
        let spark_y = y0 + 40 - 30 - (offset * 15.0) as i32;
        
        if spark_y > y0 - 60 && spark_y < y0 + 40 {
            let brightness = ((1.0 - offset / 8.0) * 255.0) as u8;
            let size = (3.0 - offset * 0.3).max(1.0) as i32;
            draw_rect_alpha(frame, spark_x - size/2, spark_y, size, size, [255, 220, 150], brightness);
        }
    }
    
    // glow alrededor
    let glow_radius = 40;
    for r in (10..glow_radius).step_by(5) {
        let alpha = (30.0 * (1.0 - r as f32 / glow_radius as f32)) as u8;
        draw_circle_alpha(frame, x0, y0 + 20, r, [255, 180, 80], alpha);
    }
}

// espaciado y paleta de colores
const LETTER_SPACING_1: i32 = 1;
const LETTER_SPACING_2: i32 = 1;
const LETTER_SPACING_L: i32 = 1;
const KERN_FACT: f32      = 0.90;  // juntar las letras un poco

const COL_LEVEL:        [u8;3] = [170,210,255];
const COL_LEVEL_SHADOW: [u8;3] = [18,24,36];

const COL_LEVEL_SEL:       [u8;3] = [255,255,255];
const COL_LEVEL_SEL_GLOW:  [u8;3] = [ 64,200,255];
const COL_LEVEL_SEL_OUT:   [u8;3] = [  0,140,255];

const COL_HDR:          [u8;3] = [210,218,230];
const COL_TIPS:         [u8;3] = [188,198,210];

const COL_CTA:          [u8;3] = [210,255,255];
const COL_CTA_GLOW:     [u8;3] = [120,230,230];

const COL_TITLE_MAIN:   [u8;3] = [218,236,255];
const COL_TITLE_GLOW1:  [u8;3] = [110,170,230];
const COL_TITLE_GLOW2:  [u8;3] = [ 70,120,200];
const COL_TITLE_SHADOW: [u8;3] = [ 14, 20, 32];

const GLYPH_W_L: i32 = 30;
const GLYPH_H_L: i32 = 48;

#[inline]
fn measure_line(text:&str, glyph_w:i32, glyph_h:i32, spacing:i32, kern:f32) -> (i32,i32) {
    let n = text.chars().count() as i32;
    if n<=0 { return (0, glyph_h); }
    let adv = (glyph_w + spacing) as f32;
    (((adv * n as f32 * kern).ceil() as i32) - spacing, glyph_h)
}

fn draw_line(
    frame:&mut [u8], text:&str, mut x:i32, y:i32,
    color:[u8;3], glyph_w:i32, spacing:i32, kern:f32, scale:i32
){
    let step = ((glyph_w + spacing) as f32 * kern).ceil() as i32;
    for ch in text.chars() {
        draw_char(frame, x, y, ch, color, scale);
        x += step;
    }
}

#[inline]
fn measure_line_large(text:&str) -> (i32,i32) {
    measure_line(text, GLYPH_W_L, GLYPH_H_L, LETTER_SPACING_L, KERN_FACT)
}

fn draw_line_large(frame:&mut [u8], text:&str, mut x:i32, y:i32, color:[u8;3]) {
    let step = ((GLYPH_W_L + LETTER_SPACING_L) as f32 * KERN_FACT).ceil() as i32;
    for ch in text.chars() {
        draw_char_large(frame, x, y, ch, color);
        x += step;
    }
}

// halo difuminado para texto seleccionado
fn soft_halo(
    frame:&mut [u8],
    x:i32, y:i32, w:i32, h:i32,
    base:[u8;3],
    base_alpha:u8,
    layers:u8,
    step:i32
){
    #[inline] fn blend(dst:u8, src:u8, a:u8)->u8{
        let ia = 255u16 - a as u16;
        let v = (dst as u16 * ia + src as u16 * a as u16) / 255u16;
        v as u8
    }
    let w_u = WIDTH as usize;

    for l in 0..layers {
        let grow = step * (l as i32 + 1);
        let alpha = base_alpha.saturating_sub(l * (base_alpha/ (layers.max(1))));
        let rx = (x - grow).max(0);
        let ry = (y - grow).max(0);
        let rw = (w + grow*2).min(WIDTH as i32 - rx);
        let rh = (h + grow*2).min(HEIGHT as i32 - ry);
        if rw<=0 || rh<=0 { continue; }

        let a = alpha;
        for yy in ry..(ry+rh) {
            let yidx = yy as usize * w_u;
            for xx in rx..(rx+rw) {
                let idx = ((yidx + xx as usize) * 4) as usize;
                frame[idx]   = blend(frame[idx],   base[0], a);
                frame[idx+1] = blend(frame[idx+1], base[1], a);
                frame[idx+2] = blend(frame[idx+2], base[2], a);
                frame[idx+3] = 255;
            }
        }
    }
}

pub fn draw_menu(frame:&mut [u8], selected: usize, t: f32) {
    // fondo con estrellas
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let i = ((y * WIDTH + x) * 4) as usize;
            let v = y as f32 / HEIGHT as f32;
            let r = (10.0 + 20.0 * (1.0 - v)) as u8;
            let g = (14.0 + 35.0 * (1.0 - v)) as u8;
            let b = (24.0 + 80.0 * (1.0 - v)) as u8;
            frame[i]=r; frame[i+1]=g; frame[i+2]=b; frame[i+3]=255;

            let seed = (x as u32).wrapping_mul(73) ^ (y as u32).wrapping_mul(97);
            let chance = (seed.wrapping_add((t * 6.0) as u32)) % 1100 == 0;
            if chance {
                let tw = ((t * 0.7 + x as f32 * 0.01 + y as f32 * 0.02).sin()*0.5+0.5)*170.0;
                frame[i]   = (frame[i]   as f32 + tw * 0.55) as u8;
                frame[i+1] = (frame[i+1] as f32 + tw * 0.65) as u8;
                frame[i+2] = (frame[i+2] as f32 + tw * 0.95) as u8;
            }
            let neb = ((x as f32 * 0.012 + t * 0.05).sin()*0.08 +
                       (y as f32 * 0.010 - t * 0.04).cos()*0.06).max(0.0);
            if neb>0.0 {
                frame[i]   = (frame[i]   as f32 + neb * 18.0) as u8;
                frame[i+1] = (frame[i+1] as f32 + neb * 22.0) as u8;
                frame[i+2] = (frame[i+2] as f32 + neb * 36.0) as u8;
            }
        }
    }

    // título grande
    let title = "RAYCASTING UWU";
    let (tw, th) = measure_line_large(title);
    let title_x = (WIDTH as i32 - tw) / 2;
    let mut y = (HEIGHT as i32 - (th + 16 + 28 + 8 + 28 + 26 + (5 * (16 + 6) - 6) + 28 + 28)) / 2;

    // glow de varias pasadas
    for off in 2..=3 {
        draw_line_large(frame, title, title_x + off, y,     COL_TITLE_GLOW2);
        draw_line_large(frame, title, title_x - off, y,     COL_TITLE_GLOW2);
        draw_line_large(frame, title, title_x,       y + off, COL_TITLE_GLOW2);
        draw_line_large(frame, title, title_x,       y - off, COL_TITLE_GLOW2);
    }
    draw_line_large(frame, title, title_x + 1, y,     COL_TITLE_GLOW1);
    draw_line_large(frame, title, title_x - 1, y,     COL_TITLE_GLOW1);
    draw_line_large(frame, title, title_x,     y + 1, COL_TITLE_GLOW1);
    draw_line_large(frame, title, title_x,     y - 1, COL_TITLE_GLOW1);

    draw_line_large(frame, title, title_x + 2, y + 2, COL_TITLE_SHADOW);
    draw_line_large(frame, title, title_x, y, COL_TITLE_MAIN);
    y += th + 16;

    // opciones de nivel
    let opt1 = "NIVEL 1 - EASY";
    let opt2 = "NIVEL 2 - COMPLICADO";
    let (_o1w, o1h) = measure_line(opt1, 18, 28, LETTER_SPACING_2, KERN_FACT);
    let (_o2w, o2h) = measure_line(opt2, 18, 28, LETTER_SPACING_2, KERN_FACT);

    draw_menu_option_text_only(
        frame, opt1, (WIDTH as i32)/2, y, selected==1, t,
        18, LETTER_SPACING_2, KERN_FACT, COL_LEVEL, COL_LEVEL_SEL
    );
    y += o1h + 8;

    draw_menu_option_text_only(
        frame, opt2, (WIDTH as i32)/2, y, selected==2, t,
        18, LETTER_SPACING_2, KERN_FACT, COL_LEVEL, COL_LEVEL_SEL
    );
    y += o2h + 26;

    // sección de controles
    let tips = [
        "CONTROLES",
        "WASD  -  MOVIMIENTO",
        "MOUSE -  CAMARA",
        "ESC   -  PAUSA",
        "TAB   -  LIBERAR MOUSE",
    ];
    let mut tips_max_w = 0;
    for line in tips.iter() {
        let (w, _) = measure_line(line, 9, 16, LETTER_SPACING_1, KERN_FACT);
        tips_max_w = tips_max_w.max(w);
    }
    let tips_x = (WIDTH as i32 - tips_max_w) / 2;
    let tips_line_gap = 6;

    let (hdr_w, _) = measure_line(tips[0], 9, 16, LETTER_SPACING_1, KERN_FACT);
    let hdr_x = tips_x + (tips_max_w - hdr_w)/2;
    draw_line(frame, tips[0], hdr_x+1, y+1, [12,16,24], 9, LETTER_SPACING_1, KERN_FACT, 1);
    draw_line(frame, tips[0], hdr_x,   y,   COL_HDR,     9, LETTER_SPACING_1, KERN_FACT, 1);
    let mut ty = y + 16 + tips_line_gap;

    for line in tips.iter().skip(1) {
        let (lw, _) = measure_line(line, 9, 16, LETTER_SPACING_1, KERN_FACT);
        let lx = tips_x + (tips_max_w - lw)/2;
        draw_line(frame, line, lx+1, ty+1, [12,16,24], 9, LETTER_SPACING_1, KERN_FACT, 1);
        draw_line(frame, line, lx,   ty,   COL_TIPS,     9, LETTER_SPACING_1, KERN_FACT, 1);
        ty += 16 + tips_line_gap;
    }
    let tips_block_h = ty - y;
    y += tips_block_h + 28;

    // call to action
    let cta = "PRESIONA ENTER";
    let (ctaw, ctah) = measure_line(cta, 18, 28, LETTER_SPACING_2, KERN_FACT);
    let cta_x = (WIDTH as i32 - ctaw) / 2;
    let pulse = (t * 4.6).sin() * 0.45 + 0.55;
    let base_a: u8 = (170.0 * pulse) as u8;
    soft_halo(frame, cta_x, y, ctaw, ctah, COL_CTA_GLOW, base_a, 4, 2);
    draw_line(frame, cta, cta_x+2, y+2, [16,22,30], 18, LETTER_SPACING_2, KERN_FACT, 2);
    draw_line(frame, cta, cta_x,   y,   COL_CTA,     18, LETTER_SPACING_2, KERN_FACT, 2);
}

fn draw_menu_option_text_only(
    frame:&mut [u8],
    text:&str,
    center_x:i32,
    y:i32,
    selected:bool,
    t:f32,
    glyph_w:i32,
    spacing:i32,
    kern:f32,
    col_normal:[u8;3],
    col_selected:[u8;3],
){
    let (text_w, h) = measure_line(text, glyph_w, 28, spacing, kern);
    let x = center_x - text_w/2;

    if selected {
        let p = (t * 3.6).sin() * 0.5 + 0.5;
        let a0: u8 = (180.0 + 60.0 * p) as u8;
        soft_halo(frame, x, y, text_w, h, COL_LEVEL_SEL_GLOW, a0, 5, 2);

        // outline + texto
        draw_line(frame, text, x,   y,   col_selected,      glyph_w, spacing, kern, 2);
    } else {
        draw_line(frame, text, x+1, y+1, COL_LEVEL_SHADOW, glyph_w, spacing, kern, 2);
        draw_line(frame, text, x,   y,   col_normal,       glyph_w, spacing, kern, 2);
    }
}


#[inline]
fn set_px(frame:&mut [u8], x:i32, y:i32, c:[u8;3]) {
    if x<0 || y<0 || x>=WIDTH as i32 || y>=HEIGHT as i32 { return; }
    let idx = ((y as usize * WIDTH as usize + x as usize) * 4) as usize;
    frame[idx]   = c[0];
    frame[idx+1] = c[1];
    frame[idx+2] = c[2];
    frame[idx+3] = 255;
}

// bresenham básico
fn line_basic(frame:&mut [u8], mut x0:i32, mut y0:i32, x1:i32, y1:i32, c:[u8;3]) {
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        set_px(frame, x0, y0, c);
        if x0 == x1 && y0 == y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; }
        if e2 <= dx { err += dx; y0 += sy; }
    }
}

// estrella de 5 puntas outline
fn draw_star_outline(frame:&mut [u8], cx:i32, cy:i32, r:i32, color:[u8;3], rot:f32) {
    let outer_r = r as f32;
    let inner_r = r as f32 * 0.45;
    let mut pts: [(i32,i32); 10] = [(0,0); 10];

    for k in 0..10 {
        let radius = if k % 2 == 0 { outer_r } else { inner_r };
        let base_ang = -std::f32::consts::FRAC_PI_2;
        let ang = base_ang + rot + (k as f32) * (std::f32::consts::PI / 5.0);
        let x = cx as f32 + ang.cos() * radius;
        let y = cy as f32 + ang.sin() * radius;
        pts[k] = (x.round() as i32, y.round() as i32);
    }
    for k in 0..10 {
        let (x0,y0) = pts[k];
        let (x1,y1) = pts[(k+1)%10];
        line_basic(frame, x0, y0, x1, y1, color);
    }
}

pub fn draw_win(frame:&mut [u8], t: f32) {
    // fondo victoria con tonos dorados
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let i = ((y * WIDTH + x) * 4) as usize;

            let depth = y as f32 / HEIGHT as f32;
            let r = (12.0 + depth * 42.0) as u8;
            let g = (34.0 + depth * 70.0) as u8;
            let b = (12.0 + depth * 28.0) as u8;

            frame[i]   = r;
            frame[i+1] = g;
            frame[i+2] = b;
            frame[i+3] = 255;

            // puntitos de brillo
            let seed = (x as u32).wrapping_mul(47) ^ (y as u32).wrapping_mul(73);
            if seed % 900 == 0 {
                let tw = 160u8;
                frame[i]   = frame[i].saturating_add(tw/6);
                frame[i+1] = frame[i+1].saturating_add(tw/4);
                frame[i+2] = frame[i+2].saturating_add(tw/8);
            }
        }
    }

    // título victoria
    let title = "VICTORIA!";
    let (tw, th) = measure_line_large(title);
    let mut y = (HEIGHT as i32) / 2 - 90;
    let title_x = (WIDTH as i32 - tw) / 2;

    let glow1 = [255, 210, 120];
    let glow2 = [210, 165, 70];
    let shadow = [20, 15, 6];
    for off in 2..=3 {
        draw_line_large(frame, title, title_x + off, y,     glow2);
        draw_line_large(frame, title, title_x - off, y,     glow2);
        draw_line_large(frame, title, title_x,       y + off, glow2);
        draw_line_large(frame, title, title_x,       y - off, glow2);
    }
    draw_line_large(frame, title, title_x + 1, y,     glow1);
    draw_line_large(frame, title, title_x - 1, y,     glow1);
    draw_line_large(frame, title, title_x,     y + 1, glow1);
    draw_line_large(frame, title, title_x,     y - 1, glow1);
    
    draw_line_large(frame, title, title_x + 2, y + 2, shadow);
    draw_line_large(frame, title, title_x, y, [255, 232, 160]);

    // 3 estrellas rotando
    let rot = t * 0.9;
    let cy = y - 20;
    let cx = WIDTH as i32 / 2;
    draw_star_outline(frame, cx,            cy - 40, 24, [255, 245, 180], rot);
    draw_star_outline(frame, cx - 95,       cy + 10, 18, [255, 245, 180], -rot*1.2);
    draw_star_outline(frame, cx + 95,       cy + 10, 18, [255, 245, 180],  rot*1.3);

    // instrucción para volver
    let hint = "ESPACIO - VOLVER AL MENU";
    let (hw, _) = measure_line(hint, 9, 16, 1, 0.95);
    let hx = (WIDTH as i32 - hw) / 2;
    let hy = y + th + 60;
    draw_line(frame, hint, hx+1, hy+1, [14,18,12], 9, 1, 0.95, 1);
    draw_line(frame, hint, hx,   hy,   [210, 255, 200], 9, 1, 0.95, 1);
}


pub fn draw_damage_overlay(frame: &mut [u8], cooldown: f32) {
    let intensity = (cooldown * 2.0 * 255.0).min(100.0) as u8;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let i = ((y * WIDTH + x) * 4) as usize;
            frame[i] = frame[i].saturating_add(intensity);
        }
    }
}

// helpers de rendering
fn draw_rect_alpha(frame: &mut [u8], x: i32, y: i32, w: i32, h: i32, color: [u8;3], alpha: u8) {
    for yy in y..y+h {
        for xx in x..x+w {
            if xx >= 0 && yy >= 0 && (xx as u32) < WIDTH && (yy as u32) < HEIGHT {
                let i = ((yy as u32 * WIDTH + xx as u32) * 4) as usize;
                let a = alpha as f32 / 255.0;
                frame[i] = (frame[i] as f32 * (1.0 - a) + color[0] as f32 * a) as u8;
                frame[i+1] = (frame[i+1] as f32 * (1.0 - a) + color[1] as f32 * a) as u8;
                frame[i+2] = (frame[i+2] as f32 * (1.0 - a) + color[2] as f32 * a) as u8;
            }
        }
    }
}

fn draw_circle_alpha(frame: &mut [u8], cx: i32, cy: i32, radius: i32, color: [u8;3], alpha: u8) {
    for y in -radius..=radius {
        for x in -radius..=radius {
            if x*x + y*y <= radius*radius {
                draw_rect_alpha(frame, cx + x, cy + y, 1, 1, color, alpha);
            }
        }
    }
}

fn draw_border(frame: &mut [u8], x: i32, y: i32, w: i32, h: i32, color: [u8;3], thickness: i32) {
    rect(frame, x, y, w, thickness, color);
    rect(frame, x, y + h - thickness, w, thickness, color);
    rect(frame, x, y, thickness, h, color);
    rect(frame, x + w - thickness, y, thickness, h, color);
}

fn draw_star(frame: &mut [u8], cx: i32, cy: i32, size: i32, color: [u8;3]) {
    let points = 5;
    let outer = size;
    let inner = size / 2;
    
    for i in 0..points * 2 {
        let angle1 = (i as f32 * 3.14159 / points as f32) - 1.5708;
        let angle2 = ((i + 1) as f32 * 3.14159 / points as f32) - 1.5708;
        
        let r1 = if i % 2 == 0 { outer } else { inner };
        let r2 = if (i + 1) % 2 == 0 { outer } else { inner };
        
        let x1 = cx + (angle1.cos() * r1 as f32) as i32;
        let y1 = cy + (angle1.sin() * r1 as f32) as i32;
        let x2 = cx + (angle2.cos() * r2 as f32) as i32;
        let y2 = cy + (angle2.sin() * r2 as f32) as i32;
        
        line_thick(frame, x1, y1, x2, y2, color, 2);
    }
    
    rect(frame, cx - 3, cy - 3, 6, 6, color);
}

fn line_thick(frame: &mut [u8], x0: i32, y0: i32, x1: i32, y1: i32, color: [u8;3], thickness: i32) {
    for t in 0..thickness {
        let offset = t - thickness / 2;
        line(frame, x0 + offset, y0, x1 + offset, y1, color);
        line(frame, x0, y0 + offset, x1, y1 + offset, color);
    }
}

fn line(frame: &mut [u8], x0: i32, y0: i32, x1: i32, y1: i32, color: [u8;3]) {
    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;
    let mut x = x0;
    let mut y = y0;
    
    for _ in 0..dx.max(dy) + 1 {
        put(frame, x, y, color);
        if x == x1 && y == y1 { break; }
        let e2 = 2 * err;
        if e2 > -dy { err -= dy; x += sx; }
        if e2 < dx { err += dx; y += sy; }
    }
}

fn draw_text(frame: &mut [u8], text: &str, x: i32, y: i32, color: [u8;3], scale: i32) {
    for (i, ch) in text.chars().enumerate() {
        draw_char(frame, x + (i as i32) * 14 * scale, y, ch, color, scale);
    }
}

fn draw_text_large(frame: &mut [u8], text: &str, x: i32, y: i32, color: [u8;3]) {
    for (i, ch) in text.chars().enumerate() {
        draw_char_large(frame, x + (i as i32) * 32, y, ch, color);
    }
}

fn draw_text_large_alpha(frame: &mut [u8], text: &str, x: i32, y: i32, color: [u8;3], alpha: u8) {
    for (i, ch) in text.chars().enumerate() {
        draw_char_large_alpha(frame, x + (i as i32) * 32, y, ch, color, alpha);
    }
}

// bitmap fonts - scale pequeño
fn draw_char(frame: &mut [u8], x: i32, y: i32, ch: char, color: [u8;3], scale: i32) {
    let s = scale.max(1);
    match ch {
        'A'|'a' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x,y,2*s,12*s,color); rect(frame,x+6*s,y,2*s,12*s,color); rect(frame,x,y+5*s,8*s,2*s,color); }
        'B'|'b' => { rect(frame,x,y,2*s,12*s,color); rect(frame,x,y,6*s,2*s,color); rect(frame,x,y+5*s,6*s,2*s,color); rect(frame,x,y+10*s,6*s,2*s,color); rect(frame,x+6*s,y+2*s,2*s,3*s,color); rect(frame,x+6*s,y+7*s,2*s,3*s,color); }
        'C'|'c' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x,y,2*s,12*s,color); rect(frame,x,y+10*s,8*s,2*s,color); }
        'D'|'d' => { rect(frame,x,y,2*s,12*s,color); rect(frame,x,y,6*s,2*s,color); rect(frame,x,y+10*s,6*s,2*s,color); rect(frame,x+6*s,y+2*s,2*s,8*s,color); }
        'E'|'e' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x,y,2*s,12*s,color); rect(frame,x,y+5*s,6*s,2*s,color); rect(frame,x,y+10*s,8*s,2*s,color); }
        'F'|'f' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x,y,2*s,12*s,color); rect(frame,x,y+5*s,6*s,2*s,color); }
        'G'|'g' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x,y,2*s,12*s,color); rect(frame,x,y+10*s,8*s,2*s,color); rect(frame,x+6*s,y+6*s,2*s,6*s,color); rect(frame,x+4*s,y+6*s,4*s,2*s,color); }
        'I'|'i' => { rect(frame,x+3*s,y,2*s,12*s,color); }
        'L'|'l' => { rect(frame,x,y,2*s,12*s,color); rect(frame,x,y+10*s,8*s,2*s,color); }
        'M'|'m' => { rect(frame,x,y,2*s,12*s,color); rect(frame,x+6*s,y,2*s,12*s,color); rect(frame,x+2*s,y,2*s,4*s,color); rect(frame,x+4*s,y,2*s,4*s,color); }
        'N'|'n' => { rect(frame,x,y,2*s,12*s,color); rect(frame,x+6*s,y,2*s,12*s,color); rect(frame,x+2*s,y+3*s,2*s,6*s,color); }
        'O'|'o' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x,y,2*s,12*s,color); rect(frame,x+6*s,y,2*s,12*s,color); rect(frame,x,y+10*s,8*s,2*s,color); }
        'P'|'p' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x,y,2*s,12*s,color); rect(frame,x,y+5*s,8*s,2*s,color); rect(frame,x+6*s,y,2*s,7*s,color); }
        'R'|'r' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x,y,2*s,12*s,color); rect(frame,x,y+5*s,8*s,2*s,color); rect(frame,x+6*s,y,2*s,7*s,color); rect(frame,x+4*s,y+7*s,4*s,5*s,color); }
        'S'|'s' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x,y,2*s,6*s,color); rect(frame,x,y+5*s,8*s,2*s,color); rect(frame,x+6*s,y+5*s,2*s,7*s,color); rect(frame,x,y+10*s,8*s,2*s,color); }
        'T'|'t' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x+3*s,y,2*s,12*s,color); }
        'U'|'u' => { rect(frame,x,y,2*s,12*s,color); rect(frame,x+6*s,y,2*s,12*s,color); rect(frame,x,y+10*s,8*s,2*s,color); }
        'V'|'v' => { rect(frame,x,y,2*s,10*s,color); rect(frame,x+6*s,y,2*s,10*s,color); rect(frame,x+2*s,y+10*s,4*s,2*s,color); }
        'W'|'w' => { rect(frame,x,y,2*s,12*s,color); rect(frame,x+6*s,y,2*s,12*s,color); rect(frame,x+2*s,y+8*s,2*s,4*s,color); rect(frame,x+4*s,y+8*s,2*s,4*s,color); }
        'Y'|'y' => { rect(frame,x,y,2*s,6*s,color); rect(frame,x+6*s,y,2*s,6*s,color); rect(frame,x+3*s,y+6*s,2*s,6*s,color); }
        'Z'|'z' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x+4*s,y+3*s,4*s,4*s,color); rect(frame,x,y+7*s,4*s,3*s,color); rect(frame,x,y+10*s,8*s,2*s,color); }
        '0' => { rect(frame,x,y,8*s,12*s,color); rect(frame,x+2*s,y+2*s,4*s,8*s,[0,0,0]); }
        '1' => { rect(frame,x+3*s,y,2*s,12*s,color); rect(frame,x+1*s,y+2*s,2*s,2*s,color); }
        '2' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x+6*s,y,2*s,6*s,color); rect(frame,x,y+5*s,8*s,2*s,color); rect(frame,x,y+5*s,2*s,7*s,color); rect(frame,x,y+10*s,8*s,2*s,color); }
        '3' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x+6*s,y,2*s,12*s,color); rect(frame,x,y+5*s,8*s,2*s,color); rect(frame,x,y+10*s,8*s,2*s,color); }
        '4' => { rect(frame,x,y,2*s,7*s,color); rect(frame,x,y+5*s,8*s,2*s,color); rect(frame,x+6*s,y,2*s,12*s,color); }
        '5' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x,y,2*s,7*s,color); rect(frame,x,y+5*s,8*s,2*s,color); rect(frame,x+6*s,y+5*s,2*s,7*s,color); rect(frame,x,y+10*s,8*s,2*s,color); }
        '6' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x,y,2*s,12*s,color); rect(frame,x,y+5*s,8*s,2*s,color); rect(frame,x+6*s,y+5*s,2*s,7*s,color); rect(frame,x,y+10*s,8*s,2*s,color); }
        '7' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x+6*s,y,2*s,12*s,color); }
        '8' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x,y,2*s,12*s,color); rect(frame,x+6*s,y,2*s,12*s,color); rect(frame,x,y+5*s,8*s,2*s,color); rect(frame,x,y+10*s,8*s,2*s,color); }
        '9' => { rect(frame,x,y,8*s,2*s,color); rect(frame,x,y,2*s,7*s,color); rect(frame,x+6*s,y,2*s,12*s,color); rect(frame,x,y+5*s,8*s,2*s,color); rect(frame,x,y+10*s,8*s,2*s,color); }
        ':' => { rect(frame,x+3*s,y+3*s,2*s,2*s,color); rect(frame,x+3*s,y+7*s,2*s,2*s,color); }
        '-' => { rect(frame,x,y+5*s,8*s,2*s,color); }
        '_' => { rect(frame,x,y+10*s,8*s,2*s,color); }
        '.' => { rect(frame,x+3*s,y+9*s,2*s,2*s,color); }
        '>' => { rect(frame,x,y+3*s,2*s,6*s,color); rect(frame,x+2*s,y+4*s,2*s,4*s,color); rect(frame,x+4*s,y+5*s,2*s,2*s,color); }
        ' ' => {}
        _ => {}
    }
}

// bitmap fonts - scale grande
fn draw_char_large(frame: &mut [u8], x: i32, y: i32, ch: char, color: [u8;3]) {
    match ch {
        'A'|'a' => { rect(frame,x,y,20,4,color); rect(frame,x,y,4,32,color); rect(frame,x+16,y,4,32,color); rect(frame,x,y+14,20,4,color); }
        'C'|'c' => { rect(frame,x,y,20,4,color); rect(frame,x,y,4,32,color); rect(frame,x,y+28,20,4,color); }
        'D'|'d' => { rect(frame,x,y,4,32,color); rect(frame,x,y,16,4,color); rect(frame,x,y+28,16,4,color); rect(frame,x+16,y+4,4,24,color); }
        'E'|'e' => { rect(frame,x,y,20,4,color); rect(frame,x,y,4,32,color); rect(frame,x,y+14,16,4,color); rect(frame,x,y+28,20,4,color); }
        'G'|'g' => { rect(frame,x,y,20,4,color); rect(frame,x,y,4,32,color); rect(frame,x,y+28,20,4,color); rect(frame,x+16,y+16,4,16,color); rect(frame,x+10,y+16,10,4,color); }
        'I'|'i' => { rect(frame,x+8,y,4,32,color); }
        'M'|'m' => { rect(frame,x,y,4,32,color); rect(frame,x+16,y,4,32,color); rect(frame,x+4,y,4,12,color); rect(frame,x+12,y,4,12,color); }
        'N'|'n' => { rect(frame,x,y,4,32,color); rect(frame,x+16,y,4,32,color); rect(frame,x+4,y+8,4,16,color); rect(frame,x+8,y+12,4,8,color); }
        'O'|'o' => { rect(frame,x,y,20,4,color); rect(frame,x,y,4,32,color); rect(frame,x+16,y,4,32,color); rect(frame,x,y+28,20,4,color); }
        'P'|'p' => { rect(frame,x,y,20,4,color); rect(frame,x,y,4,32,color); rect(frame,x,y+14,20,4,color); rect(frame,x+16,y,4,18,color); }
        'R'|'r' => { rect(frame,x,y,20,4,color); rect(frame,x,y,4,32,color); rect(frame,x,y+14,20,4,color); rect(frame,x+16,y,4,18,color); rect(frame,x+10,y+18,10,14,color); }
        'S'|'s' => { rect(frame,x,y,20,4,color); rect(frame,x,y,4,16,color); rect(frame,x,y+14,20,4,color); rect(frame,x+16,y+14,4,18,color); rect(frame,x,y+28,20,4,color); }
        'T'|'t' => { rect(frame,x,y,20,4,color); rect(frame,x+8,y,4,32,color); }
        'U'|'u' => { rect(frame,x,y,4,32,color); rect(frame,x+16,y,4,32,color); rect(frame,x,y+28,20,4,color); }
        'V'|'v' => { rect(frame,x,y,4,28,color); rect(frame,x+16,y,4,28,color); rect(frame,x+6,y+28,8,4,color); }
        'Y'|'y' => { rect(frame,x,y,4,16,color); rect(frame,x+16,y,4,16,color); rect(frame,x+8,y+16,4,16,color); }
        'Z'|'z' => { rect(frame,x,y,20,4,color); rect(frame,x+8,y+8,12,12,color); rect(frame,x,y+20,12,8,color); rect(frame,x,y+28,20,4,color); }
        '!' => { rect(frame,x+8,y,4,22,color); rect(frame,x+8,y+26,4,6,color); }
        ' ' => {}
        '-' => { rect(frame,x+4,y+14,12,4,color); }
        _ => { rect(frame,x+6,y+12,8,8,color); }
    }
}


fn draw_char_large_alpha(frame: &mut [u8], x: i32, y: i32, ch: char, color: [u8;3], alpha: u8) {
    match ch {
        'A'|'a' => { 
            draw_rect_alpha(frame,x,y,20,4,color,alpha); 
            draw_rect_alpha(frame,x,y,4,32,color,alpha); 
            draw_rect_alpha(frame,x+16,y,4,32,color,alpha); 
            draw_rect_alpha(frame,x,y+14,20,4,color,alpha); 
        }
        'C'|'c' => { 
            draw_rect_alpha(frame,x,y,20,4,color,alpha); 
            draw_rect_alpha(frame,x,y,4,32,color,alpha); 
            draw_rect_alpha(frame,x,y+28,20,4,color,alpha); 
        }
        'D'|'d' => { 
            draw_rect_alpha(frame,x,y,4,32,color,alpha); 
            draw_rect_alpha(frame,x,y,16,4,color,alpha); 
            draw_rect_alpha(frame,x,y+28,16,4,color,alpha); 
            draw_rect_alpha(frame,x+16,y+4,4,24,color,alpha); 
        }
        'I'|'i' => { draw_rect_alpha(frame,x+8,y,4,32,color,alpha); }
        'O'|'o' => { 
            draw_rect_alpha(frame,x,y,20,4,color,alpha); 
            draw_rect_alpha(frame,x,y,4,32,color,alpha); 
            draw_rect_alpha(frame,x+16,y,4,32,color,alpha); 
            draw_rect_alpha(frame,x,y+28,20,4,color,alpha); 
        }
        'R'|'r' => { 
            draw_rect_alpha(frame,x,y,20,4,color,alpha); 
            draw_rect_alpha(frame,x,y,4,32,color,alpha); 
            draw_rect_alpha(frame,x,y+14,20,4,color,alpha); 
            draw_rect_alpha(frame,x+16,y,4,18,color,alpha); 
            draw_rect_alpha(frame,x+10,y+18,10,14,color,alpha); 
        }
        'T'|'t' => { 
            draw_rect_alpha(frame,x,y,20,4,color,alpha); 
            draw_rect_alpha(frame,x+8,y,4,32,color,alpha); 
        }
        'V'|'v' => { 
            draw_rect_alpha(frame,x,y,4,28,color,alpha); 
            draw_rect_alpha(frame,x+16,y,4,28,color,alpha); 
            draw_rect_alpha(frame,x+6,y+28,8,4,color,alpha); 
        }
        '!' => { 
            draw_rect_alpha(frame,x+8,y,4,22,color,alpha); 
            draw_rect_alpha(frame,x+8,y+26,4,6,color,alpha); 
        }
        ' ' => {}
        _ => { draw_rect_alpha(frame,x+6,y+12,8,8,color,alpha); }
    }
}