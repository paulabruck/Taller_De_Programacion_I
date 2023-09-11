mod file;
mod bomberman;
mod object;
use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead};
use std::path::Path;
use std::fmt::Display;
use std::fs::read_to_string;
use file::read_file;
use std::env;
use crate::bomberman::create_objects;



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
    let mut file_contents = match read_file(&input_file) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Error al leer el archivo: {}", error);
            return Err(error);
        }
    };

    let objects = match create_objects(&mut file_contents, coordinate_x, coordinate_y) {
        Ok(resultado) => resultado,
        Err(error) => return Err(error),
    };
    let input = "B5C";

    // Convierte la cadena en un iterador de caracteres
    let mut chars = input.chars();

    while let Some(c) = chars.next() {
        if c == 'B' {
            // Si el carácter es 'B', verifica el siguiente carácter
            if let Some(next_char) = chars.next() {
                // Convierte el siguiente carácter en usize
                if let Some(digit) = next_char.to_digit(10) {
                    let value_as_usize = digit as usize;
                    println!("Next character after 'B' is '{}', Converted to usize: {}", next_char, value_as_usize);
                }
            }
        }
    }







    Ok(())
}