use crate::util::{WIDTH, HEIGHT};
#[inline]
pub fn put(px: &mut [u8], x: i32, y: i32, rgb: [u8;3]) {
if x<0 || y<0 {return}
let (x, y) = (x as u32, y as u32);
if x>=WIDTH || y>=HEIGHT {return}
let idx = ((y*WIDTH + x)*4) as usize;
px[idx..idx+3].copy_from_slice(&rgb);
px[idx+3] = 0xff;
}



pub fn hline(px: &mut [u8], x0:i32,x1:i32,y:i32, rgb:[u8;3]) {
let (mut x0, mut x1) = (x0.min(x1), x0.max(x1));
for x in x0..=x1 { put(px,x,y,rgb); }
}


pub fn vline(px: &mut [u8], x:i32,y0:i32,y1:i32, rgb:[u8;3]) {
let (mut y0, mut y1) = (y0.min(y1), y0.max(y1));
for y in y0..=y1 { put(px,x,y,rgb); }
}


// Dibujar minimapa en la esquina superior izquierda
pub fn draw_minimap(px: &mut [u8], map:&crate::map::Map, pl:(f32,f32,f32)) {
let scale = 6i32; 
let offx = 8i32; let offy = 8i32;
for my in 0..map.h as i32 {
for mx in 0..map.w as i32 {
let id = map.at(mx,my);
let c = match id { 0=>[30,30,30],1=>[200,40,40],2=>[40,200,40],3=>[40,40,220],_=>[190,190,60] };
if id!=0 {
for yy in 0..scale { for xx in 0..scale {
put(px, offx+mx*scale+xx, offy+my*scale+yy, c);
}}
} else {
for yy in 0..scale { for xx in 0..scale {
put(px, offx+mx*scale+xx, offy+my*scale+yy, [15,15,15]);
}}
}
}
}
// Jugador
let (pxf, pyf, _ang) = pl;
let jx = offx + (pxf*scale as f32) as i32;
let jy = offy + (pyf*scale as f32) as i32;
for yy in -1..=1 { for xx in -1..=1 { put(px, jx+xx, jy+yy, [255,255,255]); }}
}

// Texto para FPS: dígitos 0-9 3x5
const DIGITS: [[u8;15];10] = [
[1,1,1, 1,0,1, 1,0,1, 1,0,1, 1,1,1], //0
[0,1,0, 1,1,0, 0,1,0, 0,1,0, 1,1,1], //1
[1,1,1, 0,0,1, 1,1,1, 1,0,0, 1,1,1], //2
[1,1,1, 0,0,1, 0,1,1, 0,0,1, 1,1,1], //3
[1,0,1, 1,0,1, 1,1,1, 0,0,1, 0,0,1], //4
[1,1,1, 1,0,0, 1,1,1, 0,0,1, 1,1,1], //5
[1,1,1, 1,0,0, 1,1,1, 1,0,1, 1,1,1], //6
[1,1,1, 0,0,1, 0,1,0, 1,0,0, 1,0,0], //7
[1,1,1, 1,0,1, 1,1,1, 1,0,1, 1,1,1], //8
[1,1,1, 1,0,1, 1,1,1, 0,0,1, 1,1,1], //9
];


pub fn draw_fps(px:&mut [u8], fps:u32) {
let s = format!("FPS: {}", fps);
let mut x = 10i32; let y = (HEIGHT as i32) - 20;
for ch in s.chars() {
if let Some(d) = ch.to_digit(10) { draw_digit(px, x, y, d as usize); x+=14; }
else { x+=8; }
}
}


fn draw_digit(px:&mut [u8], x:i32, y:i32, d:usize) {
let pat = DIGITS[d];
for j in 0..5 { for i in 0..3 {
if pat[j*3+i] == 1 { for oy in 0..3 { for ox in 0..3 { put(px, x+i as i32*3+ox, y+j as i32*3+oy, [255,255,255]); }}}
}}
}