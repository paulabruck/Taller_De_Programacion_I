use std::error::Error;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;
use crate::utils::errores::error_empty_file;

pub fn read_file(input_file: &str) -> Result<String, Box<dyn Error>> {
    // Ahora 'file_contents' contiene el contenido completo del archivo como una cadena
    let file_contents = read_to_string(input_file)?;
    if file_contents.is_empty() {
        return Err(Box::new(error_empty_file()));
    }
    println!("Contenido del archivo:\n{}", file_contents);
    Ok(file_contents)
}

pub fn parse_maze(file_contents: &str) -> Vec<Vec<String>> {
    let mut maze: Vec<Vec<String>> = Vec::new();

    for line in file_contents.lines() {
        let row: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        maze.push(row);
    }
    maze
}

pub fn save_maze_in_file(
    maze: &Vec<Vec<String>>,
    ruta: &str,
) -> Result<(), std::io::Error> {
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
