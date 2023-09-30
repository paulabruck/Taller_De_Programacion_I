use crate::utils::errores::error_empty_file;
use std::error::Error;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

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
pub fn save_maze_in_file(
    maze: &Vec<Vec<String>>,
    folder: &str,
    input_path: &str,
) -> Result<(), std::io::Error> {
    let input_file_name = Path::new(input_path)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("maze.txt");

    // Crear la ruta completa de destino
    let mut destination_path = PathBuf::new();
    destination_path.push(folder);
    destination_path.push(input_file_name);

    let mut file = File::create(destination_path)?;

    for row in maze {
        let row_str = row.join(" ");
        file.write_all(row_str.as_bytes())?;
        file.write_all(b"\n")?;
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
pub fn write_error_in_file(
    mensaje_error: &str,
    folder: &str,
    input_path: &str,
) -> Result<(), std::io::Error> {
    let input_file_name = Path::new(input_path)
        .file_name()
        .and_then(|os_str| os_str.to_str())
        .unwrap_or("error.txt");

    let mut destination_path = PathBuf::new();
    destination_path.push(folder);
    destination_path.push(input_file_name);

    let mut file = File::create(destination_path)?;
    file.write_all(mensaje_error.as_bytes())?;
    Ok(())
}
