use crate::utils::errores::error_empty_file;
use std::error::Error;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

/// Lee el contenido de un archivo y lo devuelve como una cadena.
///
/// # Argumentos
///
/// - `input_file`: Ruta del archivo que se va a leer.
///
/// # Errores
///
/// Este método puede devolver un error si ocurren problemas al leer el archivo o si el archivo está vacío.
///

pub fn read_file(input_file: &str) -> Result<String, Box<dyn Error>> {
    // Ahora 'file_contents' contiene el contenido completo del archivo como una cadena
    let file_contents = read_to_string(input_file)?;
    if file_contents.is_empty() {
        return Err(Box::new(error_empty_file()));
    }
    println!("Contenido del archivo:\n{}", file_contents);
    Ok(file_contents)
}

/// Analiza el contenido del archivo para crear una representación del laberinto en forma de matriz.
///
/// # Argumentos
///
/// - `file_contents`: Contenido del archivo como una cadena.
///

pub fn parse_maze(file_contents: &str) -> Vec<Vec<String>> {
    let mut maze: Vec<Vec<String>> = Vec::new();

    for line in file_contents.lines() {
        let row: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        maze.push(row);
    }
    maze
}

/// Guarda una representación del laberinto en un archivo de texto.
///
/// # Argumentos
///
/// - `maze`: Matriz bidimensional que representa el laberinto.
/// - `ruta`: Ruta del archivo en el que se guardará el laberinto.
///
/// # Errores
///
/// Este método puede devolver un error si ocurren problemas al escribir en el archivo.
///

pub fn save_maze_in_file(maze: &Vec<Vec<String>>, ruta: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(ruta)?;
    for row in maze {
        for cell in row {
            file.write_all(cell.as_bytes())?;
            file.write_all(b" ")?;
        }
        file.write_all(b"\n")?; // Salto de línea para separar las filas
    }
    Ok(())
}
/// Escribe un mensaje de error en un archivo ubicado en la ruta especificada.
///
/// # Argumentos
///
/// - `mensaje_error`: El mensaje de error que se escribirá en el archivo.
/// - `ruta_archivo`: La ruta del archivo en el que se escribirá el mensaje de error.
///
/// # Errores
///
/// Este método puede devolver un error si ocurren problemas al escribir en el archivo.
///
pub fn write_error_in_file(mensaje_error: &str, ruta_archivo: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(ruta_archivo)?;

    // Escribe el mensaje de error en el archivo
    file.write_all(mensaje_error.as_bytes())?;

    Ok(())
}
