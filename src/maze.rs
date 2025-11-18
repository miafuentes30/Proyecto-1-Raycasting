use std::fs::File;
use std::io::{BufRead, BufReader};
use raylib::prelude::*;

pub type Maze = Vec<Vec<char>>;

pub fn load_maze(filename: &str) -> Maze {
    let file = File::open(filename).expect(&format!("No se pudo abrir el archivo: {}", filename));
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn print_maze(maze: &Maze) {
    println!("Dimensiones del laberinto: {}x{}", maze[0].len(), maze.len());
    for row in maze {
        for &cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

pub fn find_player_start(maze: &Maze) -> Option<(f32, f32)> {
    for (j, row) in maze.iter().enumerate() {
        for (i, &cell) in row.iter().enumerate() {
            if cell == 'P' || cell == 'p' {
                let x = (i as f32 * 20.0) + 10.0;
                let y = (j as f32 * 20.0) + 10.0;
                return Some((x, y));
            }
        }
    }
    None
}

pub fn get_cell_color(cell: char) -> Color {
    match cell {
        '#' => Color::GRAY,          // Pared normal (caja2)
        'M' => Color::DARKGRAY,      // Pared especial (caja4)
        'L' => Color::BROWN,         // Pared x (caja3)
        '$' => Color::DARKPURPLE,    // Puerta (caja5)
        'T' => Color::DARKGREEN,     // Tuberia tipo 1
        'P' => Color::GREEN,         // Tuberia tipo 2
        'E' => Color::YELLOW,        // Salida
        'p' => Color::BLUE,          // Player spawn
        'F' => Color::RED,           // Enemy
        'C' => Color::GOLD,          // Chest
        _ => Color::BLACK,           // Espacio vacio
    }
}