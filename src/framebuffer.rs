use raylib::prelude::*;

pub struct Framebuffer {
    pub width: i32,
    pub height: i32,
    pub buffer: Image,
    background_color: Color,
    current_color: Color,
}

impl Framebuffer {
    pub fn new_buffer(width: i32, height: i32, background_color: Color) -> Self {
        let buffer = Image::gen_image_color(width, height, background_color);
        Framebuffer { 
            width, 
            height, 
            buffer,
            background_color,
            current_color: Color::BLACK,
        }
    }
    
    pub fn clear(&mut self) {
        self.buffer.clear_background(self.background_color);
    }
    
    pub fn set_pixel(&mut self, x: i32, y: i32) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.buffer.draw_pixel(x, y, self.current_color);
        }
    }
    
    pub fn draw_rect(&mut self, x: i32, y: i32, width: i32, height: i32) {
        for i in x..x + width {
            for j in y..y + height {
                self.set_pixel(i, j);
            }
        }
    }
    
    pub fn set_background_color(&mut self, color: Color) {
        self.background_color = color;
    }
    
    pub fn set_current_color(&mut self, color: Color) {
        self.current_color = color;
    }
    
    pub fn render_to_file(&self, file_path: &str) {
        self.buffer.export_image(file_path);
        println!("Se guardo en: {file_path}")
    }
}
