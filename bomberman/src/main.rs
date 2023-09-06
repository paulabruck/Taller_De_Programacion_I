use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt::Display;

fn main() -> Result<(), Box<dyn Error>> {
   
    // Leer los argumentos de la línea de comandos
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() != 5 {
        eprintln!("Uso: cargo run -- <ruta_archivo_input> <ruta_directorio_output> <coordenada_x> <coordenada_y>");
        return Err("Cantidad incorrecta de argumentos".into());
    }

    // Obtener los argumentos
    let input_file = &arguments[1];
    let output_file = &arguments[2];
    let coordinate_x: usize = arguments[3].parse()?;
    let coordinate_y: usize = arguments[4].parse()?;
    
    println!("{}", input_file);
    println!("{}", output_file);
    println!("{}", coordinate_x);
    println!("{}", coordinate_y);
    
    //leer el archivo input
    let file = File::open(input_file)?;
    let reader = io::BufReader::new(file);
    let mut maze: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        println!("{}", line);
        let row: Vec<char> = line?.chars().collect();
        maze.push(row);
    }


    Ok(())
}