use bomberman::bomberman::{create_objects, detonar_bomb};
use bomberman::file::{save_maze_in_file, read_file, parse_maze};
use bomberman::game_data::GameData;
use bomberman::utils::errores::{error_path_invalido, error_objetos_invalidos };
use std::env;
use std::error::Error;

fn parse_arguments() -> Result<(String, String, usize, usize), Box<dyn Error>> {
    let arguments: Vec<String> = env::args().collect();
    if arguments.len() != 5 {
        return Err(Box::new(error_path_invalido()));
    }

    let input_file = arguments[1].clone();
    let output_directory = arguments[2].clone();
    let coordinate_x = arguments[4].parse()?;
    let coordinate_y = arguments[3].parse()?;

    Ok((input_file, output_directory, coordinate_x, coordinate_y))
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
            return Err(Box::new(error_objetos_invalidos()));
        }
    };

    let _final_maze = match detonar_bomb(&mut game_data, coordinate_x, coordinate_y) {
        Ok(resultado) => resultado,
        Err(error) => return Err(error),
    };

    match save_maze_in_file(&game_data.maze, &output_directory) {
        Ok(_) => println!("El maze se ha guardado exitosamente."),
        Err(err) => eprintln!("Error al guardar el maze: {}", err),
    }

    GameData::print_maze(&game_data.maze);
    Ok(())
}
