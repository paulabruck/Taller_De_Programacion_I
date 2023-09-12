use std::error::Error;
use std::fmt::Display;

#[derive(Debug, Clone)]
enum TypeBomba {
    Normal,
    Traspaso,
}
#[derive(Clone)]
pub struct Bomba {
    position: (usize, usize),
    typee: TypeBomba,
    reach: usize,
}
#[derive(Clone)]
pub struct Enemigo{
    position: (usize, usize),
    lives: usize,
}

#[derive(Debug, Clone)]
enum TypeDetour {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Clone)]
pub struct Detour{
    position: (usize, usize),
    direction: TypeDetour,
}

pub struct GameData {
    pub bombas: Vec<Bomba>,
    pub enemies: Vec<Enemigo>,
    pub detours: Vec<Detour>,
    pub laberinto: Vec<Vec<String>>,
}

fn validate_maze(bombas:Vec<Bomba>, coordinate_x: usize, coordinate_y: usize)-> Result<(), Box<dyn Error>>{
    let mut bomba_encontrada = false;
    for bomba in &bombas {
        if bomba.position == (coordinate_x, coordinate_y) {
            bomba_encontrada = true;
            match bomba.typee {
                TypeBomba::Normal => {
                    println!("¡Encontraste una bomba normal en la coordenada ({}, {})!", coordinate_x, coordinate_y);
                }
                TypeBomba::Traspaso => {
                    println!("¡Encontraste una bomba de traspaso en la coordenada ({}, {})!", coordinate_x, coordinate_y);
                }
            }
        }
    }
    if bomba_encontrada {
        Ok(())
    } else {
        Err("No se encontró una bomba en las coordenadas especificadas.".into())
    }
}
pub fn create_objects(file_contents: &mut str, coordinate_x: usize, coordinate_y: usize, maze: Vec<Vec<String>>) -> Result<GameData, Box<dyn Error>>{
    let mut position: (usize, usize) = (0, 0);
    let mut bombas: Vec<Bomba> = Vec::new();
    let mut enemies: Vec<Enemigo> = Vec::new();
    let mut detours: Vec<Detour> = Vec::new();
    let mut chars = file_contents.chars();

    while let Some(character) = chars.next(){
        if character == '\n'{
            position.1 = 0;
            position.0 += 1;
        }
        if character == ' '{
            position.1 += 1;
        }
        if character == '\n' || character == '_' {
            continue;
        };
        if  character != ' ' {   
            if character == 'B' || character == 'S'{
                if let Some(next_char) = chars.next() {
                    if let Some(digit) = next_char.to_digit(10) {
                        let value_as_usize = digit as usize;
                        if character == 'B'{
                            println!("Next character after 'B' is '{}', Converted to usize: {}", next_char, value_as_usize);
                            let bomba_normal = Bomba {
                                position: (position.0, position.1),
                                typee: TypeBomba::Normal,
                                reach: value_as_usize,
                            };
                            println!("Posición de la bomba normal: {:?}", bomba_normal.position);
                            bombas.push(bomba_normal);

                        }else{
                            println!("Next character after 'S' is '{}', Converted to usize: {}", next_char, value_as_usize);
                            let bomba_traspaso = Bomba {
                            position: (position.0, position.1),
                            typee: TypeBomba::Traspaso,
                            reach: value_as_usize,
                            };
                            println!("Posición de la bomba traspaso: {:?}", bomba_traspaso.position);
                            bombas.push(bomba_traspaso);

                        }
                    }
                }
            }
            if character == 'F'{
                if let Some(next_char) = chars.next() {
                    if let Some(digit) = next_char.to_digit(10) {
                        let value_as_usize = digit as usize;
                        let enemy = Enemigo {
                            position: (position.0, position.1),
                            lives: value_as_usize,
                        };
                        println!("Posición del enemigo: {:?}", enemy.position);
                        enemies.push(enemy);
                    }
                }    
            }
            if character == 'D' {
                if let Some(next_char) = chars.next() {
                    if next_char == 'R'{
                        let detour = Detour {
                            position: (position.0, position.1),
                            direction: TypeDetour::Right,
                        };  
                        println!("Posición del desvio: {:?}", detour.position);
                        detours.push(detour);
                    }
                    if next_char == 'L'{
                        let detour = Detour {
                            position: (position.0, position.1),
                            direction: TypeDetour::Left,
                        };  
                        println!("Posición del desvio: {:?}", detour.position);
                        detours.push(detour);
                    }
                    if next_char == 'U'{
                        let detour = Detour {
                            position: (position.0, position.1),
                            direction: TypeDetour::Up,
                        };  
                        println!("Posición del desvio: {:?}", detour.position);
                        println!("Next character after 'D' is '{}'", next_char);
                        detours.push(detour);
                    }
                    if next_char == 'D'{
                        let detour = Detour {
                            position: (position.0, position.1),
                            direction: TypeDetour::Down,
                        };  
                        println!("Posición del desvio: {:?}", detour.position);
                        detours.push(detour);
                    }
                    
                }
            }    
            //println!("{}", character)
        }
    }
    match validate_maze(bombas.clone(), coordinate_x, coordinate_y) {
        Ok(_) => {
            // La validación fue exitosa, continúa el programa
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
    let game_data = GameData {
        bombas: bombas.clone(),
        enemies: enemies.clone(),
        detours: detours.clone(),
        laberinto: maze.clone(),

    };
    Ok(game_data)
}
pub fn show_maze(mut game_data: GameData, coordinate_x: usize, coordinate_y: usize) -> Result<(), Box<dyn Error>>{
    //busco en el vector de bombas la bomba en la coordenada x e y 
    //guardo en una varibale el alcance 
    //modifico en el laberinto la B por _ 
    //modifico el vector de bombas 
    let mut alcance = 0;
    let mut posicion_bomba= (0,0);
    if let Some(bomba) = game_data.bombas.iter_mut().find(|b| b.position == (coordinate_x, coordinate_y)) {
        // Guarda el alcance de la bomba
        alcance = bomba.reach;
        posicion_bomba = bomba.position;
        //println!("Alcance de la bomba x: {}", coordinate_x);
        game_data.laberinto[coordinate_x][coordinate_y] = "_".to_string();
        game_data.bombas.retain(|b| b.position != (coordinate_x, coordinate_y));
        for row in &game_data.laberinto {
            for cell in row {
                print!("{}", cell);
                print!(" ");
            }
            println!(); // Salto de línea para separar las filas
        }
    }


    //chequear lo q afecta 
    // Supongamos que tienes una matriz llamada `laberinto`, una posición inicial `(x, y)`
    // y el alcance de la bomba `alcance`.
let (x, y) = (coordinate_x, coordinate_y);
// Recorrer hacia abajo
for dx in 1..=alcance  {
    let mut nueva_x = x.wrapping_add(1 * dx);
    //let nueva_x = x + dx;
    // Verificar si la nueva posición está dentro de los límites del laberinto
    if nueva_x < game_data.laberinto.len() && y < game_data.laberinto[nueva_x].len() {
        let objeto = &game_data.laberinto[nueva_x][y]; // Obtener el objeto en la posición
        if objeto.starts_with("D"){
            println!("¡Encontraste un desvio en la posición ({}, {})!", nueva_x, y);
        }
        if objeto == "E"{
            println!("¡Encontraste un enemigo en la posición ({}, {})!", nueva_x, y);
        }
        if objeto == "R"{
            println!("¡Encontraste una roca en la posición ({}, {})!", nueva_x, y);
        }
        if objeto == "W"{
            println!("¡Encontraste una pared en la posición ({}, {})!", nueva_x, y);
        }
        if objeto == "B" || objeto == "S"{
            println!("¡Encontraste una bomba en la posición ({}, {})!", nueva_x, y);
        }
    } else {
        // La nueva posición está fuera de los límites del laberinto, así que detenemos la búsqueda en esa dirección.
        break;
    }
}
let (x, y) = (coordinate_x, coordinate_y);
// Recorrer hacia la arriba
for dx in 1..=alcance  {
    println!("dx {}", dx);
    //println!("x {}", x);
    let mut nueva_x = x.wrapping_sub(1 * dx);
    println!("x {}", nueva_x);
    
    // Verificar si la nueva posición está dentro de los límites del laberinto
    if nueva_x < game_data.laberinto.len() && y < game_data.laberinto[nueva_x].len() {
        let objeto = &game_data.laberinto[nueva_x][y]; // Obtener el objeto en la posición
        if objeto.starts_with("D"){
            println!("¡Encontraste un desvio en la posición ({}, {})!", nueva_x, y);
        }
        if objeto == "E"{
            println!("¡Encontraste un enemigo en la posición ({}, {})!", nueva_x, y);
        }
        if objeto == "R"{
            println!("¡Encontraste una roca en la posición ({}, {})!", nueva_x, y);
        }
        if objeto == "W"{
            println!("¡Encontraste una pared en la posición ({}, {})!", nueva_x, y);
        }
        if objeto == "B" || objeto == "S"{
            println!("¡Encontraste una bomba en la posición ({}, {})!", nueva_x, y);
        }
    } else {
        // La nueva posición está fuera de los límites del laberinto, así que detenemos la búsqueda en esa dirección.
        break;
    }
}
let (x, y) = (coordinate_x, coordinate_y);
// Recorrer hacia la derecha
for dx in 1..=alcance  {
    println!("dx {}", dx);
    println!("y {}", y);
    let  nueva_y = y.wrapping_add(1 * dx);
    println!("x {}", nueva_y);
    
    // Verificar si la nueva posición está dentro de los límites del laberinto
    if nueva_y < game_data.laberinto.len() && y < game_data.laberinto[nueva_y].len() {
        let objeto = &game_data.laberinto[x][nueva_y]; // Obtener el objeto en la posición
        if objeto.starts_with("D"){
            println!("¡Encontraste un desvio en la posición ({}, {})!", x, nueva_y);
        }
        if objeto == "E"{
            println!("¡Encontraste un enemigo en la posición ({}, {})!", x, nueva_y);
        }
        if objeto == "R"{
            println!("¡Encontraste una roca en la posición ({}, {})!", x, nueva_y);
        }
        if objeto == "W"{
            println!("¡Encontraste una pared en la posición ({}, {})!", x, nueva_y);
        }
        if objeto == "B" || objeto == "S"{
            println!("¡Encontraste una bomba en la posición ({}, {})!", x, nueva_y);
        }
    } else {
        // La nueva posición está fuera de los límites del laberinto, así que detenemos la búsqueda en esa dirección.
        break;
    }
}
let (x, y) = (coordinate_x, coordinate_y);
// Recorrer hacia la izquierda
for dx in 1..=alcance  {
    println!("dx {}", dx);
    //println!("x {}", x);
    let  nueva_y = y.wrapping_sub(1 * dx);
    println!("x {}", nueva_y);
    
    // Verificar si la nueva posición está dentro de los límites del laberinto
    if nueva_y < game_data.laberinto.len() && y < game_data.laberinto[nueva_y].len() {
        let objeto = &game_data.laberinto[x][nueva_y]; // Obtener el objeto en la posición
        if objeto.starts_with("D"){
            println!("¡Encontraste un desvio en la posición ({}, {})!", x, nueva_y);
        }
        if objeto == "E"{
            println!("¡Encontraste un enemigo en la posición ({}, {})!", x, nueva_y);
        }
        if objeto == "R"{
            println!("¡Encontraste una roca en la posición ({}, {})!", x, nueva_y);
        }
        if objeto == "W"{
            println!("¡Encontraste una pared en la posición ({}, {})!", x, nueva_y);
        }
        if objeto.starts_with("B") || objeto.starts_with("S"){
            println!("¡Encontraste una bomba en la posición ({}, {})!", x, nueva_y);
        }
    } else {
        // La nueva posición está fuera de los límites del laberinto, así que detenemos la búsqueda en esa dirección.
        break;
    }
}

    
    

Ok(())
}