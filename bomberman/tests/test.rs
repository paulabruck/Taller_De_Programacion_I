use bomberman::file::{guardar_laberinto_en_archivo, read_file, parse_maze};
use bomberman::bomberman::{create_objects, detonar_bomba};

#[test]
fn test_read_file() {
    let mut file_contents = match read_file("./src/maze.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
}

#[test]
fn test_guardar_laberinto_en_archivo() {
    // Crea un laberinto de prueba
    let laberinto: Vec<Vec<String>> = vec![
        vec!["_".to_string(), "_".to_string()],
        vec!["_".to_string(), "_".to_string()],
    ];

    // Prueba la función guardar_laberinto_en_archivo
    let result = guardar_laberinto_en_archivo(&laberinto, "maze4.txt"); // Reemplaza "test_output.txt" con tu archivo de prueba
    assert!(result.is_ok()); // Verifica si guardar el laberinto en un archivo tuvo éxito
}

#[test]
fn test_create_objects() {
    // Crea un laberinto de prueba
    let mut file_contents = match read_file("./src/maze.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);
    // Prueba la función create_objects
    let mut result = match create_objects(&mut file_contents, 0, 0, maze){
        Ok(data) => data,
        Err(error) => return assert!(false, "Los objetos no se pudieron crear correctamente"),
    };
    
}

#[test]
fn test_create_objects_invalid_empty_maze() {
    // Crea un laberinto de prueba
    let mut file_contents = match read_file("./src/maze_invalido.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(true, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);
    // Prueba la función create_objects
    let mut result = match create_objects(&mut file_contents, 0, 0, maze){
        Ok(data) => data,
        Err(error) => return assert!(false, "Los objetos no se pudieron crear correctamente"),
    };
    
}
#[test]
fn test_detonar_bomba_invalid_location() {
    let mut file_contents = match read_file("./src/maze2.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);

    let mut game_data = match create_objects(&mut file_contents, 0, 0, maze) {
        Ok(data) => data,
        Err(error) => return assert!(true, "Los objetos no se pudieron crear correctamente"),
    };

    let result = detonar_bomba(&mut game_data, 0, 0);

    // Verifica que la detonación de la bomba falle debido a la ubicación incorrecta
    assert!(result.is_err());
}

#[test]
fn test_create_objects_invalid_maze() {
    // Crea un laberinto de prueba
    let mut file_contents = match read_file("./src/maze_invalido1.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(true, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);
    // Prueba la función create_objects
    let mut result = match create_objects(&mut file_contents, 0, 0, maze){
        Ok(data) => data,
        Err(error) => return assert!(true, "Los objetos no se pudieron crear correctamente"),
    };
    
}

#[test]
fn test_detonate_bomb() {
    // Crea un laberinto de prueba con una bomba colocada
    let mut file_contents = match read_file("./src/maze.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);

    let mut game_data = match create_objects(&mut file_contents, 0, 0, maze) {
        Ok(data) => data,
        Err(error) => return assert!(false, "Los objetos no se pudieron crear correctamente"),
    };

    // Detona la bomba en una posición específica
    let result = detonar_bomba(&mut game_data, 0, 0);
    
    // Verifica que la detonación haya tenido éxito
    assert!(result.is_ok());

    // Verifica que el estado del juego y el laberinto reflejen los cambios esperados después de la detonación
    assert_eq!(game_data.enemies.len(), 0);
    assert_eq!(game_data.bombas.len(), 0);
}

#[test]
fn test_interaction_with_enemies() {
    // Crea un laberinto de prueba con enemigos cercanos a una bomba
    let mut file_contents = match read_file("./src/maze2.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);

    let mut game_data = match create_objects(&mut file_contents, 4, 0, maze) {
        Ok(data) => data,
        Err(error) => return assert!(false, "Los objetos no se pudieron crear correctamente"),
    };

    // Detona la bomba en una posición cercana a los enemigos
    let result = detonar_bomba(&mut game_data, 4, 0);
    
    // Verifica que la detonación haya tenido éxito
    assert!(result.is_ok());

    // Verifica que los enemigos hayan recibido daño y sus vidas se hayan actualizado correctamente
    assert_eq!(game_data.enemies.len(), 1);
    assert_eq!(game_data.enemies[0].lives, 1);
    assert_eq!(game_data.enemies[0].position, (4,2));

}

#[test]
fn test_detonar_bomba_valid_location() {
    let mut file_contents = match read_file("./src/maze.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);

    let mut game_data = match create_objects(&mut file_contents, 0, 0, maze) {
        Ok(data) => data,
        Err(error) => return assert!(false, "Los objetos no se pudieron crear correctamente"),
    };

    let result = detonar_bomba(&mut game_data, 0, 0);

    // Verifica que la detonación de la bomba tuvo éxito
    assert!(result.is_ok());
}


// Prueba detonar una bomba en una ubicación con enemigo
#[test]
fn test_detonar_bomba_enemy_location() {
    let mut file_contents = match read_file("./src/maze.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);

    let mut game_data = match create_objects(&mut file_contents, 0, 0, maze) {
        Ok(data) => data,
        Err(error) => return assert!(false, "Los objetos no se pudieron crear correctamente"),
    };

    let result = detonar_bomba(&mut game_data, 0, 0);

    // Verifica que la detonación de la bomba tuvo éxito y eliminó al enemigo
    assert!(result.is_ok());
    assert_eq!(game_data.enemies.len(), 0);
}

// Prueba detonar una bomba en una ubicación con objetos destructibles
#[test]
fn test_detonar_bomba_objects_location() {
    let mut file_contents = match read_file("./src/maze2.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);

    let mut game_data = match create_objects(&mut file_contents, 4, 0, maze) {
        Ok(data) => data,
        Err(error) => return assert!(false, "Los objetos no se pudieron crear correctamente"),
    };

    let result = detonar_bomba(&mut game_data, 4, 0);

    // Verifica que la detonación de la bomba tuvo éxito y destruyó los objetos
    assert!(result.is_ok());
    assert!(game_data.laberinto[4][0] == "_");
}

// Prueba detonar una bomba en una ubicación con desvío
#[test]
fn test_detonar_bomba_detour_location() {
    let mut file_contents = match read_file("./src/maze2.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);

    let mut game_data = match create_objects(&mut file_contents, 4, 0, maze) {
        Ok(data) => data,
        Err(error) => return assert!(false, "Los objetos no se pudieron crear correctamente"),
    };

}


