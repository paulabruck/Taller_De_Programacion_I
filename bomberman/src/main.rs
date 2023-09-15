use bomberman::bomberman::{create_objects, detonar_bomb};
use bomberman::file::{parse_maze, read_file, save_maze_in_file};
use bomberman::game_data::GameData;
use bomberman::utils::errores::{error_objetos_invalidos, error_path_invalido};
use std::env;
use std::error::Error;

/// Parsea los argumentos de línea de comandos.
///
/// Esta función se encarga de parsear los argumentos de línea de comandos proporcionados al programa. Se espera que los argumentos incluyan el nombre del archivo de entrada, el directorio de salida, las coordenadas X e Y.
///
/// # Argumentos
///
/// * `input_file`: El nombre del archivo de entrada.
/// * `output_directory`: El directorio de salida donde se guardará el laberinto modificado.
/// * `coordinate_x`: La coordenada X en la que se colocará la bomba.
/// * `coordinate_y`: La coordenada Y en la que se colocará la bomba.
///
/// # Errores
///
/// Esta función devuelve un `Result` que contiene una tupla con los valores parseados o un error si la cantidad de argumentos es incorrecta o si no se pueden parsear las coordenadas X e Y.

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
