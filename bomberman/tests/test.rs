use bomberman::bomberman::{create_objects, detonar_bomb};
use bomberman::file::{parse_maze, read_file, save_maze_in_file};

/// Prueba la función `read_file` para asegurarse de que puede leer un archivo correctamente.
#[test]
fn test_read_file() {
    let _file_contents = match read_file("./src/maze.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
}

/// Prueba la función `save_maze_in_file` para asegurarse de que puede guardar el maze en un archivo.
#[test]
fn test_save_maze_in_file() {
    // Crea un maze de prueba
    let maze: Vec<Vec<String>> = vec![
        vec!["_".to_string(), "_".to_string()],
        vec!["_".to_string(), "_".to_string()],
    ];

    // Prueba la función save_maze_in_file
    let result = save_maze_in_file(&maze, "./src/", "./src/maze6.txt");
    assert!(result.is_ok()); // Verifica si guardar el maze en un archivo tuvo éxito
}

/// Prueba la función `create_objects` para asegurarse de que puede crear objetos a partir de un archivo.
#[test]
fn test_create_objects() {
    // Crea un maze de prueba
    let mut file_contents = match read_file("./src/maze.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);
    // Prueba la función create_objects
    let _result = match create_objects( 0, 0, maze) {
        Ok(data) => data,
        Err(_error) => return assert!(false, "Los objetos no se pudieron crear correctamente"),
    };
}

/// Prueba la función `create_objects` cuando el maze es inválido y está vacío.
#[test]
fn test_create_objects_invalid_empty_maze() {
    // Crea un maze de prueba
    let mut file_contents = match read_file("./src/maze_invalido.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(true, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);
    // Prueba la función create_objects
    let _result = match create_objects( 0, 0, maze) {
        Ok(data) => data,
        Err(_error) => return assert!(true, "Los objetos no se pudieron crear correctamente"),
    };
}

/// Prueba la función `create_objects` cuando el maze es inválido y contiene objetos inválidos.
#[test]
fn test_create_objects_invalid_maze() {
    // Crea un maze de prueba
    let mut file_contents = match read_file("./src/maze_invalido1.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(true, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);
    // Prueba la función create_objects
    let _result = match create_objects( 0, 0, maze) {
        Ok(data) => data,
        Err(_error) => return assert!(true, "Los objetos no se pudieron crear correctamente"),
    };
}

/// Prueba la función `detonar_bomb` en una ubicación inválida.
#[test]
fn test_detonar_bomba_invalid_location() {
    let mut file_contents = match read_file("./src/maze2.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);

    let mut game_data = match create_objects( 0, 0, maze) {
        Ok(data) => data,
        Err(_error) => return assert!(true, "Los objetos no se pudieron crear correctamente"),
    };

    let result = detonar_bomb(&mut game_data, 0, 0);

    // Verifica que la detonación de la bomba falle debido a la ubicación incorrecta
    assert!(result.is_err());
}

/// Prueba la función `detonar_bomb` en una ubicación válida.
#[test]
fn test_detonate_bomb() {
    // Crea un maze de prueba con una bomba colocada
    let mut file_contents = match read_file("./src/maze.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);

    let mut game_data = match create_objects( 0, 0, maze) {
        Ok(data) => data,
        Err(_error) => return assert!(false, "Los objetos no se pudieron crear correctamente"),
    };

    // Detona la bomba en una posición específica
    let result = detonar_bomb(&mut game_data, 0, 0);

    // Verifica que la detonación haya tenido éxito
    assert!(result.is_ok());

    // Verifica que el estado del juego y el maze reflejen los cambios esperados después de la detonación
    assert_eq!(game_data.enemies.len(), 0);
    assert_eq!(game_data.bombs.len(), 0);
}

/// Prueba la interacción con enemigos cuando se detona una bomba cerca de ellos.
#[test]
fn test_interaction_with_enemies() {
    // Crea un maze de prueba con enemigos cercanos a una bomba
    let mut file_contents = match read_file("./src/maze2.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);

    let mut game_data = match create_objects( 4, 0, maze) {
        Ok(data) => data,
        Err(_error) => return assert!(false, "Los objetos no se pudieron crear correctamente"),
    };

    // Detona la bomba en una posición cercana a los enemigos
    let result = detonar_bomb(&mut game_data, 4, 0);

    // Verifica que la detonación haya tenido éxito
    assert!(result.is_ok());

    // Verifica que los enemigos hayan recibido daño y sus vidas se hayan actualizado correctamente
    assert_eq!(game_data.enemies.len(), 1);
    assert_eq!(game_data.enemies[0].lives, 1);
    assert_eq!(game_data.enemies[0].position, (4, 2));
    //verificamos que una misma bomba no pueda lastimar mas de 1 una vez a un enemigo
}

/// Prueba la función `detonar_bomb` en una ubicación con enemigo.
#[test]
fn test_detonar_bomba_enemy_location() {
    let mut file_contents = match read_file("./src/maze.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);

    let mut game_data = match create_objects( 0, 0, maze) {
        Ok(data) => data,
        Err(_error) => return assert!(false, "Los objetos no se pudieron crear correctamente"),
    };

    let result = detonar_bomb(&mut game_data, 0, 0);

    // Verifica que la detonación de la bomba tuvo éxito y eliminó al enemigo
    assert!(result.is_ok());
    assert_eq!(game_data.enemies.len(), 0);
}

/// Prueba la función `detonar_bomb` en una ubicación con objetos destructibles.
#[test]
fn test_detonar_bomba_objects_location() {
    let mut file_contents = match read_file("./src/maze2.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);

    let mut game_data = match create_objects( 4, 0, maze) {
        Ok(data) => data,
        Err(_error) => return assert!(false, "Los objetos no se pudieron crear correctamente"),
    };

    let result = detonar_bomb(&mut game_data, 4, 0);

    // Verifica que la detonación de la bomba tuvo éxito y destruyó los objetos
    assert!(result.is_ok());
    assert!(game_data.maze[4][0] == "_"); // lugar de la bomba inicial a detonar
    assert!(game_data.maze[2][0] == "_"); //detono otra bomba de traspaso
    assert!(game_data.maze[2][4] == "_"); // bomba de traspaso atraveso las rocas
}

/// Prueba la función `detonar_bomb` en una ubicación con objetos destructibles.
#[test]
fn test_detonar_bomba_objects_location2() {
    let mut file_contents = match read_file("./src/maze3.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);

    let mut game_data = match create_objects( 4, 0, maze) {
        Ok(data) => data,
        Err(_error) => return assert!(true, "Los objetos no se pudieron crear correctamente"),
    };

    let result = detonar_bomb(&mut game_data, 4, 0);

    // Verifica que la detonación de la bomba tuvo éxito y destruyó los objetos
    assert!(result.is_ok());
    assert!(game_data.maze[4][0] == "_"); // lugar de la bomba inicial a detonar
    assert!(game_data.maze[6][0] == "F1"); // bomba normal no atraviesa roca
    assert!(game_data.maze[4][2] == "F2"); // bomba normal no atravisesa pared
    assert!(game_data.maze[0][0] == "F4"); // bomba traspaso no atraviesa pared
}

/// Prueba la función `detonar_bomb` en una ubicación con desvío.
#[test]
fn test_detonar_bomba_detour_location() {
    let mut file_contents = match read_file("./src/maze0.txt") {
        Ok(resultado) => resultado,
        Err(_error) => return assert!(false, "El archivo no se pudo leer correctamente"),
    };
    let maze = parse_maze(&file_contents);

    let mut game_data = match create_objects( 4, 0, maze) {
        Ok(data) => data,
        Err(_error) => return assert!(false, "Los objetos no se pudieron crear correctamente"),
    };
    let result = detonar_bomb(&mut game_data, 4, 0);

    // Verifica que la detonación de la bomba tuvo éxito y destruyó los objetos
    assert!(result.is_ok());
    assert!(game_data.maze[4][0] == "_"); // lugar de la bomba inicial a detonar
    assert!(game_data.maze[2][4] == "_"); // desvio para arriba le quita la ultima vida al enemigo
    assert!(game_data.maze[4][4] == "DU"); // desvio para arriba
    assert!(game_data.maze[2][0] == "_"); // bomba detonada de traspaso
    assert!(game_data.maze[1][0] == "DR"); // desvio para la derecha
    assert!(game_data.maze[1][2] == "_"); // desvio para la derecha me mata al enemigo
    assert!(game_data.maze[1][3] == "DD"); // desvio para abajo
    assert!(game_data.maze[2][3] == "F2"); // desvio para abajo me resta una vida de el enemigo
    assert!(game_data.maze[3][3] == "DL"); // desvio para la izquierda
    assert!(game_data.maze[3][2] == "_"); // desvio para la izquierda me mata un enemigo
}
