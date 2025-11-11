pub struct Map {
    pub w: usize,
    pub h: usize,
    pub tiles: Vec<u8>,
}

impl Map {
    pub fn at(&self, x: i32, y: i32) -> u8 {
        if x < 0 || y < 0 { return 1; }
        let (x, y) = (x as usize, y as usize);
        if x >= self.w || y >= self.h { return 1; }
        self.tiles[y * self.w + x]
    }

    pub fn demo() -> Self { Self::level1() }

    pub fn level1() -> Self {
        // Nivel 1
        let raw = [
            "1111111111111",
            "1...........1",
            "1.222...333.1",
            "1...........1",
            "1...........1",
            "1...555555..1",
            "1...555555..1",
            "1...........1",
            "1.........4.1",
            "1.........9.1",
            "1...........1",
            "1111111111111",
        ];
        to_map(&raw)
    }

    pub fn level2() -> Self {
        // Nivel 2
        let raw = [
            "222222222222222",
            "2.............2",
            "2.3333..4444..2",
            "2.3..3..4..4..2",
            "2.3..3..4..4..2",
            "2.3..3........2",
            "2.3..555555...2",
            "2.3..555555...2",
            "2....555555...2",
            "2.........9...2",
            "2.............2",
            "222222222222222",
        ];
        to_map(&raw)
    }
}

fn to_map(lines: &[&str]) -> Map {
    let h = lines.len();
    let w = lines[0].len();
    let mut t = Vec::with_capacity(w*h);
    
    for row in lines {
        for ch in row.chars() {
            t.push(match ch {
                '.'|'0' => 0,  
                '1' => 1,       // ladrillo 
                '2' => 2,       // grama
                '3' => 3,       // vidrio
                '4' => 4,       // oro
                '5' => 5,       // lava (daño)
                '9' => 9,       // portal
                _ => 1,
            });
        }
    }
    
    Map { w, h, tiles: t }
}