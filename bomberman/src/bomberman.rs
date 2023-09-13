use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::Write;

// ...

pub fn guardar_laberinto_en_archivo(laberinto: &Vec<Vec<String>>, ruta: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(ruta)?;
    
    for row in laberinto {
        for cell in row {
            file.write_all(cell.as_bytes())?;
            file.write_all(b" ")?;
        }
        file.write_all(b"\n")?; // Salto de línea para separar las filas
    }

    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub bombas_recibidas: Option<Vec<Bomba>>,
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
#[derive(Clone)]
pub struct GameData {
    pub bombas: Vec<Bomba>,
    pub enemies: Vec<Enemigo>,
    pub detours: Vec<Detour>,
    pub laberinto: Vec<Vec<String>>,
    pub pared_intercepta: bool,
    pub roca_intercepta: bool,
}
fn chequear_objetos(game_data: &mut GameData, objeto: &String, nueva_x: usize, y: usize,typee: TypeBomba, iteraciones_restantes: usize, bomba: &Bomba) {
    if objeto.starts_with("D") {
        println!("¡Encontraste un desvío en la posición ({}, {})!", nueva_x, y);
        //println!("ALCANCE RESTANTE  {}!",  iteraciones_restantes);

        if objeto == "DU"{
            recorrer_hacia_arriba( game_data,nueva_x, y,iteraciones_restantes, typee.clone(), &bomba);
        }
        if objeto == "DD"{
            recorrer_hacia_abajo( game_data,nueva_x, y,iteraciones_restantes, typee.clone(), &bomba);
        }
        if objeto == "DR"{
            recorrer_hacia_derecha( game_data,nueva_x, y,iteraciones_restantes, typee.clone(), &bomba);
        }
        if objeto == "DL"{
            recorrer_hacia_izquierda( game_data,nueva_x, y,iteraciones_restantes, typee.clone(), &bomba);
        }
    }
    if objeto.starts_with("F") {
        println!("¡Encontraste un enemigo en la posición ({}, {})!", nueva_x, y);
        if let Some(enemy) = game_data.enemies.iter_mut().find(| enemy| enemy.position == (nueva_x, y)) {
            if enemy.lives > 0 {
                if let Some(ref mut bombas_recibidas) = &mut enemy.bombas_recibidas {
                    if !bombas_recibidas.iter().any(|b| b.position == bomba.position) {
                        bombas_recibidas.push(bomba.clone());
                    } else {
                        println!("La bomba ya existe en bombas_recibidas");
                        enemy.lives += 1;
                    }
                } else {
                    // Si `enemy.bombas_recibidas` es `None`, puedes crear un nuevo `Vec<Bomba>` y asignarlo.
                    let mut new_bombas_recibidas = Vec::new();
                    new_bombas_recibidas.push(bomba.clone());
                    enemy.bombas_recibidas = Some(new_bombas_recibidas);
                }
                enemy.lives -= 1;
                println!("Vidas del enemigo: {}", enemy.lives);
                let lives_str = enemy.lives.to_string();
                let objeto_str = "F".to_string() + &lives_str;
                game_data.laberinto[nueva_x][y] = objeto_str;
                
                
            }
            if enemy.lives == 0 {
                game_data.laberinto[nueva_x][y] = "_".to_string();
                game_data.enemies.retain(|b| b.position != (nueva_x, y));
            }
        }
    }
    if objeto == "R" {
        println!("¡Encontraste una roca en la posición ({}, {})!", nueva_x, y);
        if typee == TypeBomba::Normal {
            game_data.roca_intercepta = true;
        }
    }
    if objeto == "W" {
        println!("¡Encontraste una pared en la posición ({}, {})!", nueva_x, y);
        game_data.pared_intercepta = true;
        
    }
    if objeto.starts_with("B") || objeto.starts_with("S") {
        println!("¡Encontraste una bomba en la posición ({}, {})!", nueva_x, y);
        show_maze(  game_data, nueva_x, y);
    }
}
fn recorrer_hacia_abajo<'a>(game_data: &'a mut GameData, x: usize, y: usize, alcance: usize,typee: TypeBomba, bomba: &'a Bomba) -> &'a mut GameData{
    for dx in 1..=alcance {
        let mut nueva_x = x.wrapping_add(1 * dx);
        
        //println!("dx ({})!", dx);
        //println!("alxance ({})!", alcance);
        let mut iteraciones_restantes = alcance - dx;
        //println!("¡iteraciones ({})!", iteraciones_restantes);
        // Verificar si la nueva posición está dentro de los límites del laberinto
        if nueva_x < game_data.laberinto.len() && y < game_data.laberinto[nueva_x].len() {
            let objeto = &game_data.laberinto[nueva_x][y]; // Obtener el objeto en la posición
            let mut game_data_clone = game_data.clone();
            chequear_objetos(&mut game_data_clone, objeto, nueva_x, y, typee, iteraciones_restantes, &bomba);
            *game_data = game_data_clone; 
            if game_data.pared_intercepta == true || game_data.roca_intercepta == true {
                break;
            }
        } else {
            // La nueva posición está fuera de los límites del laberinto, así que detenemos la búsqueda en esa dirección.
            break;
        }
    }
    game_data.pared_intercepta = false;
    game_data.roca_intercepta = false;
    game_data
}

// Haz lo mismo para las otras funciones de recorrido (hacia arriba, derecha e izquierda).

fn recorrer_hacia_arriba<'a>(game_data: &'a mut GameData, x: usize, y: usize, alcance: usize,typee: TypeBomba, bomba: &'a Bomba) -> &'a mut GameData {
    for dx in 1..=alcance {
        let mut nueva_x = x.wrapping_sub(1 * dx);
       //println!("dx ({})!", dx);
        //println!("alxance ({})!", alcance);
        let mut iteraciones_restantes = alcance - dx;
        //println!("¡iteraciones ({})!", iteraciones_restantes);
        // Verificar si la nueva posición está dentro de los límites del laberinto
        if nueva_x < game_data.laberinto.len() && y < game_data.laberinto[nueva_x].len() {
            let objeto = &game_data.laberinto[nueva_x][y]; // Obtener el objeto en la posición
            let mut game_data_clone = game_data.clone();
            chequear_objetos(&mut game_data_clone, objeto, nueva_x, y, typee, iteraciones_restantes, &bomba);
            *game_data = game_data_clone; 
            if game_data.pared_intercepta == true || game_data.roca_intercepta == true {
                break;
            }
        } else {
            // La nueva posición está fuera de los límites del laberinto, así que detenemos la búsqueda en esa dirección.
            break;
        }
    }
    game_data.pared_intercepta = false;
    game_data.roca_intercepta = false;
    game_data // Devuelve el game_data actualizado
}

fn recorrer_hacia_derecha<'a>(game_data: &'a mut GameData, x: usize, y: usize, alcance: usize, typee: TypeBomba, bomba: &'a Bomba) -> &'a mut GameData {
    for dx in 1..=alcance {
        let nueva_y = y.wrapping_add(1 * dx);
        //println!("dx ({})!", dx);
        //println!("alxance ({})!", alcance);
        let mut iteraciones_restantes = alcance - dx;
        //println!("¡iteraciones ({})!", iteraciones_restantes);
        // Verificar si la nueva posición está dentro de los límites del laberinto
        if x < game_data.laberinto.len() && nueva_y < game_data.laberinto[x].len() {
            let objeto = &game_data.laberinto[x][nueva_y]; // Obtener el objeto en la posición
            let mut game_data_clone = game_data.clone();
            chequear_objetos(&mut game_data_clone, objeto, x, nueva_y, typee, iteraciones_restantes, &bomba);
            *game_data = game_data_clone; 
            if game_data.pared_intercepta == true || game_data.roca_intercepta == true {
                break;
            }
        } else {
            // La nueva posición está fuera de los límites del laberinto, así que detenemos la búsqueda en esa dirección.
            break;
        }
    }
    game_data.pared_intercepta = false;
    game_data.roca_intercepta = false;
    game_data // Devuelve el game_data actualizado
}

fn recorrer_hacia_izquierda<'a>(game_data: &'a mut GameData, x: usize, y: usize, alcance: usize, typee: TypeBomba, bomba: &'a Bomba) -> &'a mut GameData {
    for dx in 1..=alcance {
        let nueva_y = y.wrapping_sub(1 * dx);
        //println!("dx ({})!", dx);
        //println!("alxance ({})!", alcance);
        let mut iteraciones_restantes = alcance - dx;
        //println!("¡iteraciones ({})!", iteraciones_restantes);
        // Verificar si la nueva posición está dentro de los límites del laberinto
        if x < game_data.laberinto.len() && nueva_y < game_data.laberinto[x].len() {
            let objeto = &game_data.laberinto[x][nueva_y]; // Obtener el objeto en la posición
            let mut game_data_clone = game_data.clone();
            chequear_objetos(&mut game_data_clone, objeto, x, nueva_y, typee, iteraciones_restantes, &bomba);
            *game_data = game_data_clone; 
            if game_data.pared_intercepta == true || game_data.roca_intercepta == true{
                break;
            }
        } else {
            // La nueva posición está fuera de los límites del laberinto, así que detenemos la búsqueda en esa dirección.
            break;
        }
    }
    game_data.pared_intercepta = false;
    game_data.roca_intercepta = false;
    game_data // Devuelve el game_data actualizado
}
fn validate_maze(game_data:GameData, coordinate_x: usize, coordinate_y: usize)-> Result<(), Box<dyn Error>>{
    let vector_bombas = game_data.bombas;
    let vector_enemigos = game_data.enemies;
    let mut bomba_encontrada = false;
    let mut alcance_valido = false;
    let mut vidas_validas = false;


    for bomba in &vector_bombas {
        if bomba.position == (coordinate_x, coordinate_y) {
            bomba_encontrada = true;
            if bomba.reach > 0 {
                alcance_valido = true;
            }
        }
    }
    for enemy in &vector_enemigos{
        if enemy.lives <=  3 || enemy.lives > 0{
            vidas_validas = true;
        }
    }
    if bomba_encontrada && alcance_valido && vidas_validas{
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
                           // println!("Next character after 'B' is '{}', Converted to usize: {}", next_char, value_as_usize);
                            let bomba_normal = Bomba {
                                position: (position.0, position.1),
                                typee: TypeBomba::Normal,
                                reach: value_as_usize,
                            };
                           // println!("Posición de la bomba normal: {:?}", bomba_normal.position);
                            bombas.push(bomba_normal);

                        }else{
                           // println!("Next character after 'S' is '{}', Converted to usize: {}", next_char, value_as_usize);
                            let bomba_traspaso = Bomba {
                            position: (position.0, position.1),
                            typee: TypeBomba::Traspaso,
                            reach: value_as_usize,
                            };
                           // println!("Posición de la bomba traspaso: {:?}", bomba_traspaso.position);
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
                            bombas_recibidas: None,
                        };
                        //println!("Posición del enemigo: {:?}", enemy.position);
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
                      //  println!("Posición del desvio: {:?}", detour.position);
                        detours.push(detour);
                    }
                    if next_char == 'L'{
                        let detour = Detour {
                            position: (position.0, position.1),
                            direction: TypeDetour::Left,
                        };  
                       // println!("Posición del desvio: {:?}", detour.position);
                        detours.push(detour);
                    }
                    if next_char == 'U'{
                        let detour = Detour {
                            position: (position.0, position.1),
                            direction: TypeDetour::Up,
                        };  
                        //println!("Posición del desvio: {:?}", detour.position);
                       // println!("Next character after 'D' is '{}'", next_char);
                        detours.push(detour);
                    }
                    if next_char == 'D'{
                        let detour = Detour {
                            position: (position.0, position.1),
                            direction: TypeDetour::Down,
                        };  
                       // println!("Posición del desvio: {:?}", detour.position);
                        detours.push(detour);
                    }
                    
                }
            }    
        }
    }
    let game_data = GameData {
        bombas: bombas.clone(),
        enemies: enemies.clone(),
        detours: detours.clone(),
        laberinto: maze.clone(),
        pared_intercepta: false,
        roca_intercepta: false,


    };
    match validate_maze(game_data.clone(), coordinate_x, coordinate_y) {
        Ok(_) => {
            // La validación fue exitosa, continúa el programa
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
    Ok(game_data)
}
pub fn show_maze(mut game_data: &mut GameData, coordinate_x: usize, coordinate_y: usize) -> Result<(), Box<dyn Error>>{
    //busco en el vector de bombas la bomba en la coordenada x e y 
    //guardo en una varibale el alcance 
    //modifico en el laberinto la B por _ 
    //modifico el vector de bombas 
    let mut alcance = 0;
    let mut tipo_bomba = TypeBomba::Normal;
    let mut posicion_bomba= (0,0);
    if let Some(bomba) = game_data.bombas.iter_mut().find(|b| b.position == (coordinate_x, coordinate_y)) {
        // Guarda el alcance de la bomba
        alcance = bomba.reach;
        tipo_bomba = bomba.typee;
        posicion_bomba = bomba.position;
        let bomba_copiada = bomba.clone();
        //println!("Alcance de la bomba x: {}", coordinate_x);
        game_data.laberinto[coordinate_x][coordinate_y] = "_".to_string();
        game_data.bombas.retain(|b| b.position != (coordinate_x, coordinate_y));
        recorrer_hacia_abajo(&mut game_data,coordinate_x, coordinate_y, alcance, tipo_bomba.clone(), &bomba_copiada);
        recorrer_hacia_arriba(&mut game_data,coordinate_x, coordinate_y,alcance, tipo_bomba.clone(), &bomba_copiada);
        recorrer_hacia_derecha(&mut game_data,coordinate_x,coordinate_y, alcance, tipo_bomba.clone(), &bomba_copiada);
        recorrer_hacia_izquierda(&mut game_data, coordinate_x, coordinate_y, alcance, tipo_bomba.clone(), &bomba_copiada);
        println!(); 
    }
    //chequear lo q afecta 
    // Supongamos que tienes una matriz llamada `laberinto`, una posición inicial `(x, y)`
    // y el alcance de la bomba `alcance`.
    for row in &game_data.laberinto {
        for cell in row {
            print!("{}", cell);
            print!(" ");
        }
        println!(); // Salto de línea para separar las filas
    }
    

    Ok(())
}