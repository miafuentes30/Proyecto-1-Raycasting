pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub offset: f32, 
}

impl Intersect {
    pub fn new(distance: f32, impact: char, offset: f32) -> Self {
        Intersect {
            distance,
            impact,
            offset,
        }
    }
}