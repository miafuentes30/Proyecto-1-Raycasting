use raylib::prelude::*;
use std::collections::HashMap;

pub struct TextureManager {
    pub images: HashMap<String, Image>,
}

impl TextureManager {
    pub fn new(_rl: &mut RaylibHandle) -> Self {
        let mut images = HashMap::new();

        // Cargar texturas de cajas 
        let wall_textures = vec![
            ("caja2", "assets/textures/caja2.png"),
            ("caja3", "assets/textures/caja3.png"),
            ("caja4", "assets/textures/caja4.png"),
            ("caja5", "assets/textures/caja5.png"),
        ];

        for (key, path) in wall_textures {
            match Image::load_image(path) {
                Ok(image) => {
                    images.insert(key.to_string(), image);
                    println!("Textura cargada: {} -> {}", key, path);
                }
                Err(e) => {
                    eprintln!("Error cargando textura {}: {:?}", path, e);
                }
            }
        }

        // Cargar texturas de tuberias
        let pipe_textures = vec![
            ("tuveria1", "assets/textures/tuveria1.png"),
            ("tuveria2", "assets/textures/tuveria2.png"),
        ];

        for (key, path) in pipe_textures {
            match Image::load_image(path) {
                Ok(image) => {
                    images.insert(key.to_string(), image);
                    println!("Tubería cargada: {} -> {}", key, path);
                }
                Err(e) => {
                    eprintln!("Error cargando tubería {}: {:?}", path, e);
                }
            }
        }

        // Cargar sprites del juego
        let sprite_list = vec![
            ("enemy", "assets/sprites/enemy.png"),
            ("chest", "assets/sprites/chest.png"),
            ("player", "assets/sprites/player_anim.png"),
            ("worker", "assets/sprites/work.png"),
        ];

        for (key, path) in sprite_list {
            match Image::load_image(path) {
                Ok(img) => {
                    images.insert(key.to_string(), img);
                    println!("Sprite cargado: {} -> {}", key, path);
                }
                Err(e) => {
                    eprintln!("Error cargando sprite {}: {:?}", path, e);
                }
            }
        }

        // Textura por defecto
        if images.is_empty() {
            eprintln!("ADVERTENCIA: No se cargó ninguna textura");
        }

        TextureManager { images }
    }

    pub fn get(&self, name: &str) -> Option<&Image> {
        self.images.get(name)
    }
}