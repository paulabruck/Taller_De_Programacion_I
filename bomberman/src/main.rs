mod file;
//mod bomberman;
use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt::Display;
use std::fs::read_to_string;
use file::read_file;
use std::env;

fn parse_arguments() -> Result<(String, String, usize, usize), Box<dyn Error>> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 5 {
        return Err("Cantidad incorrecta de argumentos".into());
    }

    let input_file = arguments[1].clone();
    let output_directory = arguments[2].clone();
    let coordinate_x = arguments[3].parse()?;
    let coordinate_y = arguments[4].parse()?;

    Ok((input_file, output_directory, coordinate_x, coordinate_y))
}

fn main() -> Result<(), Box<dyn Error>> {
   
    let (input_file, output_directory, coordinate_x, coordinate_y) = parse_arguments()?;
    
    println!("Ruta del archivo de entrada: {}", input_file);
    println!("Ruta del directorio de salida: {}", output_directory);
    println!("Coordenada X: {}", coordinate_x);
    println!("Coordenada Y: {}", coordinate_y);
    
    // Llamar a la función read_file
    let file_contents = match read_file(&input_file) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Error al leer el archivo: {}", error);
            return Err(error);
        }
    };


    //proceso cada caracter 

     // Llamar a la función para crear objetos a partir del laberinto
    /*let objects = match create_objects(&mut file_contents) {
        Ok(result) => result,
        Err(error) => return Err(error.into()), // Manejar el error aquí como se mencionó anteriormente
    };
    */
    Ok(())
}