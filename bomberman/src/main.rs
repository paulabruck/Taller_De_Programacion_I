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
    println!();

    //leer el archivo input
    let file = File::open(input_file)?;
    let reader = io::BufReader::new(file);
    let mut maze: Vec<Vec<char>> = Vec::new();
    
    for line in reader.lines() {
        let row: Vec<char> = line?.chars().collect();
        
        maze.push(row);
    }

    // Obtener las dimensiones del laberinto
    let num_rows = maze.len();
    let mut num_columns =  0 ;
    println!("{}", num_columns);
    println!("{}", num_rows);

    // Recorrer cada fila
    for (row_index, row) in maze.iter().enumerate() {
        // Recorrer cada columna en la fila actual
        for (col_index, &cell) in row.iter().enumerate() {
            // Aquí 'cell' contiene el carácter en la posición (fila_index, col_index)
            if cell != ' '{
                num_columns+=1
            }
            println!("En la posición ({}, {}), encontré el carácter: {}", row_index, col_index, cell);
        }
    }
    println!("{}", num_columns/num_rows);
    
    //mostrar maze cargado
    for row in &maze {
        for &cell in row {
            print!("{}", cell);
        }
        println!(); // Salto de línea para separar las filas
    }
    
    //validar el laberinto
    //chequear si en la coordenada por consola hay bomba 
    
    let character_at_xy = maze[coordinate_x][coordinate_y];

    // Imprimir el carácter en la posición (x, y)
    println!("El carácter en la posición ({}, {}) es: {}", coordinate_x, coordinate_y, character_at_xy);
    if character_at_xy != 'B'|| character_at_xy != 'S' {
        return Err("error_piezas_invalidas()".into());
    }
    

    

    Ok(())
}