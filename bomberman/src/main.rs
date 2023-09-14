use bomberman::bomberman::{create_objects, print_laberinto, show_maze};
use bomberman::file::{guardar_laberinto_en_archivo, read_file};
use std::env;
use std::error::Error;

fn parse_arguments() -> Result<(String, String, usize, usize), Box<dyn Error>> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 5 {
        return Err("Cantidad incorrecta de argumentos".into());
    }

    let input_file = arguments[1].clone();
    let output_directory = arguments[2].clone();
    let coordinate_x = arguments[4].parse()?;
    let coordinate_y = arguments[3].parse()?;

    Ok((input_file, output_directory, coordinate_x, coordinate_y))
}

fn parse_maze(file_contents: &str) -> Vec<Vec<String>> {
    let mut maze: Vec<Vec<String>> = Vec::new();

    for line in file_contents.lines() {
        let row: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        maze.push(row);
    }
    maze
}

fn main() -> Result<(), Box<dyn Error>> {
    let (input_file, output_directory, coordinate_x, coordinate_y) = parse_arguments()?;
    let mut file_contents = match read_file(&input_file) {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Error al leer el archivo: {}", error);
            return Err(error);
        }
    };

    let maze = parse_maze(&file_contents);
    let mut game_data = match create_objects(&mut file_contents, coordinate_x, coordinate_y, maze) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Error: {}", error);
            return Err(error);
        }
    };
    let _final_maze = match show_maze(&mut game_data, coordinate_x, coordinate_y) {
        Ok(resultado) => resultado,
        Err(error) => return Err(error),
    };

    match guardar_laberinto_en_archivo(&game_data.laberinto, &output_directory) {
        Ok(_) => println!("El laberinto se ha guardado exitosamente."),
        Err(err) => eprintln!("Error al guardar el laberinto: {}", err),
    }

    print_laberinto(&game_data.laberinto);
    Ok(())
}
