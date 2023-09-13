// Importa las funciones y estructuras que deseas probar

use bomberman::file::{read_file, guardar_laberinto_en_archivo};
//use crate::bomberman::{create_objects, show_maze};
use bomberman::bomberman::{create_objects};

#[test]
fn test_read_file() {
    // Prueba la función read_file
    let result = read_file("maze.txt"); // Reemplaza "test_input.txt" con tu archivo de prueba
    assert!(result.is_ok()); // Verifica si la lectura del archivo tuvo éxito
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

// #[test]
// fn test_create_objects() {
//     // Crea un laberinto de prueba
//     let mut file_contents = read_file("maze.txt"); // Reemplaza con tu contenido de prueba
//     let mut maze: Vec<Vec<String>> = Vec::new();
    
//     for line in file_contents.lines() {
//         let row: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
//         maze.push(row);
//     }
//     // Prueba la función create_objects
//     let mut result = create_objects(&mut file_contents, 0, 0, maze); // Pasa tus argumentos de prueba
//     assert_eq!(result)
// }

