#[derive(Clone)]
pub struct Map {
pub w: usize,
pub h: usize,
pub cells: Vec<u8>, 
}


impl Map {
pub fn demo() -> Self {
let w = 16; let h = 16;
let mut cells = vec![0u8; w*h];
// bordes
for x in 0..w { cells[x] = 1; cells[(h-1)*w + x] = 1; }
for y in 0..h { cells[y*w] = 1; cells[y*w + (w-1)] = 1; }
// cuartos
for x in 3..13 { cells[5*w + x] = 2; }
for y in 6..13 { cells[y*w + 3] = 3; }
for x in 4..12 { cells[10*w + x] = 4; }
Self { w, h, cells }
}


#[inline]
pub fn at(&self, x: i32, y: i32) -> u8 {
if x < 0 || y < 0 { return 1 } 
let (x, y) = (x as usize, y as usize);
if x >= self.w || y >= self.h { return 1 }
self.cells[y*self.w + x]
}


pub fn is_wall(&self, x: i32, y: i32) -> bool { self.at(x,y) != 0 }
}