mod object;
    /*

fn create_objects(maze: &mut str) -> Result<Vec<Box<dyn Objeto>>, Box<dyn std::error::Error>> {
    let maze_chars: Vec<char> = maze.chars().collect();
    let mut objects: Vec<Box<dyn Objeto>> = Vec::new();

    let mut row = 0;
    let mut column = 0;

    for &caracter in &maze_chars {
        if caracter == '\n' {
            row += 1;
            column = 0;
            continue;
        }
        let position = (row, column);
        let object = create_object(caracter, position, OtrasPropiedades::default());
        objects.push(object);
        column += 1;
    }
    Ok(objects)
}
*/
fn create_objects(maze: &mut str) -> Result<Vec<Box<dyn Objeto>>, Box<dyn std::error::Error>> {
    let mut objetos: Vec<Box<dyn Objeto>> = Vec::new();

    // Procesar el laberinto y crear objetos aquí
    for (fila, linea) in maze.lines().enumerate() {
        for (columna, caracter) in linea.chars().enumerate() {
            let posicion = (fila, columna);
            let objeto = crear_objeto(caracter, posicion);
            objetos.push(objeto);
        }
    }

    // Devolver el vector de objetos
    Ok(objetos)
}
