use crate::bomba::process_bomba;
use crate::bomba::Bomba;
use crate::bomba::TypeBomba;
use crate::desvio::handle_desvio;
use crate::desvio::process_detour;
use crate::desvio::Detour;
use crate::enemigo::handle_enemigo;
use crate::enemigo::process_enemy;
use crate::enemigo::Enemigo;
use crate::game_data::create_game_data;
use crate::game_data::GameData;
use std::error::Error;

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
    let game_data = create_game_data(
        bombas.clone(),
        enemies.clone(),
        detours.clone(),
        maze.clone(),
        false, // Pared intercepta inicialmente en falso
        false, // Roca intercepta inicialmente en falso
    );

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
        'D' => process_detour(chars, position, detours),
        _ => {}
    }
}

fn chequear_objetos(
    game_data: &mut GameData,
    objeto: &String,
    nueva_x: usize,
    y: usize,
    typee: TypeBomba,
    iteraciones_restantes: usize,
    bomba: &Bomba,
) {
    if objeto.starts_with("D") {
        handle_desvio(
            game_data,
            objeto,
            nueva_x,
            y,
            typee.clone(),
            iteraciones_restantes,
            &bomba,
        )
    }
    if objeto.starts_with("F") {
        handle_enemigo(
            game_data,
            objeto,
            nueva_x,
            y,
            typee.clone(),
            iteraciones_restantes,
            &bomba,
        )
    }
    if objeto == "R" && typee == TypeBomba::Normal {
        game_data.roca_intercepta = true;
    }
    if objeto == "W" {
        game_data.pared_intercepta = true;
    }
    if objeto.starts_with("B") || objeto.starts_with("S") {
        show_maze(game_data, nueva_x, y);
    }
}

pub fn chequeos_recorridos<'a>(
    nueva_x: usize,
    game_data: &'a mut GameData,
    y: usize,
    typee: TypeBomba,
    bomba: &'a Bomba,
    iteraciones_restantes: usize,
) {
    let objeto = &game_data.laberinto[nueva_x][y]; // Obtener el objeto en la posición
    let mut game_data_clone = game_data.clone();
    chequear_objetos(
        &mut game_data_clone,
        objeto,
        nueva_x,
        y,
        typee,
        iteraciones_restantes,
        &bomba,
    );
    *game_data = game_data_clone;
}

pub fn recorrer_hacia_abajo<'a>(
    game_data: &'a mut GameData,
    x: usize,
    y: usize,
    alcance: usize,
    typee: TypeBomba,
    bomba: &'a Bomba,
) -> &'a mut GameData {
    for dx in 1..=alcance {
        let nueva_x = x.wrapping_add(1 * dx);
        let iteraciones_restantes = alcance - dx;
        if nueva_x < game_data.laberinto.len() && y < game_data.laberinto[nueva_x].len() {
            chequeos_recorridos(nueva_x, game_data, y, typee, bomba, iteraciones_restantes);
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

pub fn recorrer_hacia_arriba<'a>(
    game_data: &'a mut GameData,
    x: usize,
    y: usize,
    alcance: usize,
    typee: TypeBomba,
    bomba: &'a Bomba,
) -> &'a mut GameData {
    for dx in 1..=alcance {
        let nueva_x = x.wrapping_sub(1 * dx);
        let iteraciones_restantes = alcance - dx;
        if nueva_x < game_data.laberinto.len() && y < game_data.laberinto[nueva_x].len() {
            chequeos_recorridos(nueva_x, game_data, y, typee, bomba, iteraciones_restantes);
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

pub fn recorrer_hacia_derecha<'a>(
    game_data: &'a mut GameData,
    x: usize,
    y: usize,
    alcance: usize,
    typee: TypeBomba,
    bomba: &'a Bomba,
) -> &'a mut GameData {
    for dx in 1..=alcance {
        let nueva_y = y.wrapping_add(1 * dx);
        let iteraciones_restantes = alcance - dx;
        if x < game_data.laberinto.len() && nueva_y < game_data.laberinto[x].len() {
            chequeos_recorridos(x, game_data, nueva_y, typee, bomba, iteraciones_restantes);
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

pub fn recorrer_hacia_izquierda<'a>(
    game_data: &'a mut GameData,
    x: usize,
    y: usize,
    alcance: usize,
    typee: TypeBomba,
    bomba: &'a Bomba,
) -> &'a mut GameData {
    for dx in 1..=alcance {
        let nueva_y = y.wrapping_sub(1 * dx);
        let iteraciones_restantes = alcance - dx;
        if x < game_data.laberinto.len() && nueva_y < game_data.laberinto[x].len() {
            chequeos_recorridos(x, game_data, nueva_y, typee, bomba, iteraciones_restantes);
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

pub fn show_maze(
    game_data: &mut GameData,
    coordinate_x: usize,
    coordinate_y: usize,
) -> Result<(), Box<dyn Error>> {
    if let Some(bomba) = game_data.find_bomba(coordinate_x, coordinate_y) {
        let alcance = bomba.reach;
        let tipo_bomba = bomba.typee;
        let bomba_copiada = bomba.clone();

        game_data.laberinto[coordinate_x][coordinate_y] = "_".to_string();
        game_data.remove_bomba(coordinate_x, coordinate_y);
        game_data.apply_bomba_effect(
            coordinate_x,
            coordinate_y,
            alcance,
            tipo_bomba.clone(),
            &bomba_copiada,
        );
    }
    Ok(())
}

pub fn print_laberinto(laberinto: &Vec<Vec<String>>) {
    for row in laberinto {
        for cell in row {
            print!("{}", cell);
            print!(" ");
        }
        println!(); // Salto de línea para separar las filas
    }
}
