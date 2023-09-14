use crate::bomba::Bomba;
use crate::bomba::TypeBomba;
use crate::desvio::Detour;
use crate::desvio::TypeDetour;
use crate::enemigo::Enemigo;
use crate::game_data::GameData;
use std::error::Error;
use crate::utils::errores::error_objeto_invalido;

pub fn process_bomba(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    bombas: &mut Vec<Bomba>,
) {
    if let Some(next_char) = chars.next() {
        if let Some(digit) = next_char.to_digit(10) {
            let value_as_usize = digit as usize;
            if character == 'B' {
                let bomba = Bomba::new((position.0, position.1), TypeBomba::Normal, value_as_usize);
                bombas.push(bomba);
            } else {
                let bomba = Bomba::new(
                    (position.0, position.1),
                    TypeBomba::Traspaso,
                    value_as_usize,
                );
                bombas.push(bomba);
            }
        }
    }
}

pub fn process_enemy(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    enemies: &mut Vec<Enemigo>,
) {
    if let Some(next_char) = chars.next() {
        if let Some(digit) = next_char.to_digit(10) {
            let value_as_usize = digit as usize;
            let enemy = Enemigo::new((position.0, position.1), value_as_usize);
            enemies.push(enemy);
        }
    }
}

pub fn process_detour(
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
        let detour = Detour::new((position.0, position.1), direction);
        detours.push(detour);
    }
}

fn process_character(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    bombas: &mut Vec<Bomba>,
    enemies: &mut Vec<Enemigo>,
    detours: &mut Vec<Detour>,
)-> Result<(), Box<dyn Error>>{
    match character {
        'B' | 'S' => Ok(process_bomba(character, chars, position, bombas)),
        'F' => Ok(process_enemy(character, chars, position, enemies)),
        'D' =>  Ok(process_detour(chars, position, detours)),
        'W'|'R'=> Ok(()),
        _ => Err(Box::new(error_objeto_invalido())),
    }
}

fn create_game_data_internal(
    bombas: Vec<Bomba>,
    enemies: Vec<Enemigo>,
    detours: Vec<Detour>,
    maze: Vec<Vec<String>>,
) -> GameData {
    GameData {
        bombas,
        enemies,
        detours,
        laberinto: maze,
        pared_intercepta: false,
        roca_intercepta: false,
    }
}

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
            if let Err(error) = process_character(
                character,
                &mut chars,
                &mut position,
                &mut bombas,
                &mut enemies,
                &mut detours,
            ){
                return Err(Box::new(error_objeto_invalido()));
            }
            
        }
    }

    let game_data = create_game_data_internal(
        bombas.clone(),
        enemies.clone(),
        detours.clone(),
        maze.clone(),
    );

    if let Err(error) = game_data.validate_maze(coordinate_x, coordinate_y) {
        eprintln!("Error: {}", error);
        return Err(Box::new(error_objeto_invalido()));
    }
    Ok(game_data)
}

pub fn chequear_objetos(
    game_data: &mut GameData,
    objeto: &String,
    nueva_x: usize,
    y: usize,
    typee: TypeBomba,
    iteraciones_restantes: usize,
    bomba: &Bomba,
) {
    if objeto.starts_with("D") {
        GameData::handle_desvio(game_data, objeto, nueva_x, y, typee.clone(), iteraciones_restantes, &bomba)
    }
    if objeto.starts_with("F") {
        GameData:: handle_enemigo(
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
        GameData:: handle_roca(
            game_data,
        )
    }
    if objeto == "W" {
        GameData:: handle_pared(
            game_data,
        )
    }
    if objeto.starts_with("B") || objeto.starts_with("S") {
        GameData:: handle_bomba(
            game_data,
            objeto,
            nueva_x,
            y,
        )
    }
}
pub fn detonar_bomba(
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


