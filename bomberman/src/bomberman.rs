use std::error::Error;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeBomba {
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
impl GameData {
    pub fn find_bomba(&mut self, coordinate_x: usize, coordinate_y: usize) -> Option<&mut Bomba> {
        self.bombas.iter_mut().find(|b| b.position == (coordinate_x, coordinate_y))
    }
    pub fn remove_bomba(&mut self, coordinate_x: usize, coordinate_y: usize) {
        self.bombas.retain(|b| b.position != (coordinate_x, coordinate_y));
    }
    pub fn apply_bomba_effect(&mut self, coordinate_x: usize, coordinate_y: usize, alcance: usize, tipo_bomba: TypeBomba, bomba_copiada: &Bomba) {
        recorrer_hacia_abajo( self,coordinate_x, coordinate_y, alcance, tipo_bomba.clone(), &bomba_copiada);
        recorrer_hacia_arriba( self,coordinate_x, coordinate_y,alcance, tipo_bomba.clone(), &bomba_copiada);
        recorrer_hacia_derecha( self,coordinate_x,coordinate_y, alcance, tipo_bomba.clone(), &bomba_copiada);
        recorrer_hacia_izquierda( self, coordinate_x, coordinate_y, alcance, tipo_bomba.clone(), &bomba_copiada);
    }
    pub fn validate_maze(&self, coordinate_x: usize, coordinate_y: usize) -> Result<(), Box<dyn Error>> {
        let bomba_encontrada = self.bombas.iter().any(|b| b.position == (coordinate_x, coordinate_y) && b.reach > 0);
        let vidas_validas = self.enemies.iter().any(|enemy| enemy.lives <= 3 && enemy.lives > 0);

        if bomba_encontrada && vidas_validas {
            Ok(())
        } else {
            Err("No se encontró una bomba en las coordenadas especificadas o las vidas de los enemigos no son válidas.".into())
        }
    }
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
        let nueva_x = x.wrapping_add(1 * dx);
        
        //println!("dx ({})!", dx);
        //println!("alxance ({})!", alcance);
        let iteraciones_restantes = alcance - dx;
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

fn recorrer_hacia_arriba<'a>(game_data: &'a mut GameData, x: usize, y: usize, alcance: usize,typee: TypeBomba, bomba: &'a Bomba) -> &'a mut GameData {
    for dx in 1..=alcance {
        let nueva_x = x.wrapping_sub(1 * dx);
       //println!("dx ({})!", dx);
        //println!("alxance ({})!", alcance);
        let iteraciones_restantes = alcance - dx;
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
        let iteraciones_restantes = alcance - dx;
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
        let iteraciones_restantes = alcance - dx;
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

// En bomberman/src/bomberman.rs

// use crate::bomba::{Bomba, TypeBomba};
// use crate::enemigo::Enemigo;
// use crate::detour::{Detour, TypeDetour};
// use crate::game_data::GameData;
// use std::error::Error;

pub fn create_objects(
    file_contents: &mut str,
    coordinate_x: usize,
    coordinate_y: usize,
    maze: Vec<Vec<String>>,
) -> Result<GameData, Box<dyn Error>> {
    let mut position: (usize, usize) = (0, 0);
    let mut bombas: Vec<Bomba> = Vec::new();
    let mut enemies: Vec<Enemigo> = Vec::new();
    let mut detours: Vec<Detour> = Vec::new();
    let mut chars = file_contents.chars();

    while let Some(character) = chars.next() {
        if character == '\n' {
            position.1 = 0;
            position.0 += 1;
        }
        if character == ' ' {
            position.1 += 1;
        }
        if character == '\n' || character == '_' {
            continue;
        };
        if character != ' ' {
            process_character(
                character,
                &mut chars,
                &mut position,
                &mut bombas,
                &mut enemies,
                &mut detours,
            );
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

    if let Err(error) = game_data.validate_maze(coordinate_x, coordinate_y) {
        eprintln!("Error: {}", error);
    }

    Ok(game_data)
}

fn process_character(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    bombas: &mut Vec<Bomba>,
    enemies: &mut Vec<Enemigo>,
    detours: &mut Vec<Detour>,
) {
    match character {
        'B' | 'S' => process_bomba(character, chars, position, bombas),
        'F' => process_enemy(character, chars, position, enemies),
        'D' => process_detour(character, chars, position, detours),
        _ => {}
    }
}

fn process_bomba(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    bombas: &mut Vec<Bomba>,
) {
    if let Some(next_char) = chars.next() {
        if let Some(digit) = next_char.to_digit(10) {
            let value_as_usize = digit as usize;
            let bomba = Bomba {
                position: (position.0, position.1),
                typee: if character == 'B' {
                    TypeBomba::Normal
                } else {
                    TypeBomba::Traspaso
                },
                reach: value_as_usize,
            };
            bombas.push(bomba);
        }
    }
}

fn process_enemy(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    enemies: &mut Vec<Enemigo>,
) {
    if let Some(next_char) = chars.next() {
        if let Some(digit) = next_char.to_digit(10) {
            let value_as_usize = digit as usize;
            let enemy = Enemigo {
                position: (position.0, position.1),
                lives: value_as_usize,
                bombas_recibidas: None,
            };
            enemies.push(enemy);
        }
    }
}

fn process_detour(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    detours: &mut Vec<Detour>,
) {
    if let Some(next_char) = chars.next() {
        let direction = match next_char {
            'R' => TypeDetour::Right,
            'L' => TypeDetour::Left,
            'U' => TypeDetour::Up,
            'D' => TypeDetour::Down,
            _ => TypeDetour::Left, // Definir un valor predeterminado apropiado
        };
        let detour = Detour {
            position: (position.0, position.1),
            direction,
        };
        detours.push(detour);
    }
}

// Resto del código del módulo bomberman

pub fn show_maze(mut game_data: &mut GameData, coordinate_x: usize, coordinate_y: usize) -> Result<(), Box<dyn Error>>{
    let mut alcance = 0;
    let mut tipo_bomba = TypeBomba::Normal;
    let mut posicion_bomba= (0,0);
    if let Some(bomba) = game_data.find_bomba(coordinate_x, coordinate_y) { 
        alcance = bomba.reach;
        tipo_bomba = bomba.typee;
        posicion_bomba = bomba.position;
        let bomba_copiada = bomba.clone();

        game_data.laberinto[coordinate_x][coordinate_y] = "_".to_string();
        game_data.remove_bomba(coordinate_x, coordinate_y);
        game_data.apply_bomba_effect(coordinate_x, coordinate_y, alcance, tipo_bomba.clone(), &bomba_copiada);        
    }
    print_laberinto(&game_data.laberinto);
    Ok(())
}
fn print_laberinto(laberinto: &Vec<Vec<String>>) {
    for row in laberinto {
        for cell in row {
            print!("{}", cell);
            print!(" ");
        }
        println!(); // Salto de línea para separar las filas
    }
}