use crate::bomb::Bomb;
use crate::bomb::TypeBomb;
use crate::detour::Detour;
use crate::detour::TypeDetour;
use crate::enemy::Enemy;
use crate::game_data::GameData;
use std::error::Error;
use crate::utils::errores::error_objeto_invalido;

pub fn process_bomb(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    bombs: &mut Vec<Bomb>,
) {
    if let Some(next_char) = chars.next() {
        if let Some(digit) = next_char.to_digit(10) {
            let value_as_usize = digit as usize;
            if character == 'B' {
                let bomb = Bomb::new((position.0, position.1), TypeBomb::Normal, value_as_usize);
                bombs.push(bomb);
            } else {
                let bomb = Bomb::new(
                    (position.0, position.1),
                    TypeBomb::Traspaso,
                    value_as_usize,
                );
                bombs.push(bomb);
            }
        }
    }
}

pub fn process_enemy(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    enemies: &mut Vec<Enemy>,
) {
    if let Some(next_char) = chars.next() {
        if let Some(digit) = next_char.to_digit(10) {
            let value_as_usize = digit as usize;
            let enemy = Enemy::new((position.0, position.1), value_as_usize);
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
    bombs: &mut Vec<Bomb>,
    enemies: &mut Vec<Enemy>,
    detours: &mut Vec<Detour>,
)-> Result<(), Box<dyn Error>>{
    match character {
        'B' | 'S' => Ok(process_bomb(character, chars, position, bombs)),
        'F' => Ok(process_enemy(character, chars, position, enemies)),
        'D' =>  Ok(process_detour(chars, position, detours)),
        'W'|'R'=> Ok(()),
        _ => Err(Box::new(error_objeto_invalido())),
    }
}

fn create_game_data_internal(
    bombs: Vec<Bomb>,
    enemies: Vec<Enemy>,
    detours: Vec<Detour>,
    maze: Vec<Vec<String>>,
) -> GameData {
    GameData {
        bombs,
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
    let mut bombs: Vec<Bomb> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
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
                &mut bombs,
                &mut enemies,
                &mut detours,
            ){
                return Err(Box::new(error_objeto_invalido()));
            }
            
        }
    }

    let game_data = create_game_data_internal(
        bombs.clone(),
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
    typee: TypeBomb,
    iteraciones_restantes: usize,
    bomb: &Bomb,
) {
    if objeto.starts_with("D") {
        GameData::handle_desvio(game_data, objeto, nueva_x, y, typee.clone(), iteraciones_restantes, &bomb)
    }
    if objeto.starts_with("F") {
        GameData:: handle_enemigo(
            game_data,
            objeto,
            nueva_x,
            y,
            typee.clone(),
            iteraciones_restantes,
            &bomb,
        )
    }
    if objeto == "R" && typee == TypeBomb::Normal {
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
        GameData:: handle_bomb(
            game_data,
            objeto,
            nueva_x,
            y,
        )
    }
}
pub fn detonar_bomb(
    game_data: &mut GameData,
    coordinate_x: usize,
    coordinate_y: usize,
) -> Result<(), Box<dyn Error>> {
    if let Some(bomb) = game_data.find_bomb(coordinate_x, coordinate_y) {
        let alcance = bomb.reach;
        let tipo_bomb = bomb.typee;
        let bomb_copiada = bomb.clone();

        game_data.laberinto[coordinate_x][coordinate_y] = "_".to_string();
        game_data.remove_bomb(coordinate_x, coordinate_y);
        game_data.apply_bomb_effect(
            coordinate_x,
            coordinate_y,
            alcance,
            tipo_bomb.clone(),
            &bomb_copiada,
        );
    }
    Ok(())
}


