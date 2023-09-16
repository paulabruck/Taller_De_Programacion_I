use crate::bomb::Bomb;
use crate::bomb::TypeBomb;
use crate::detour::Detour;
use crate::detour::TypeDetour;
use crate::enemy::Enemy;
use crate::game_data::GameData;
use crate::utils::errores::error_objeto_invalido;
use std::error::Error;
use crate::utils::constantes::*;


/// Procesa un carácter como una bomba y agrega una instancia de `Bomb` al vector `bombs`.
///
/// # Argumentos
///
/// - `character`: Carácter que representa el tipo de bomba ('B' para Normal, 'S' para Traspaso).
/// - `chars`: Referencia mutable a los caracteres restantes en la cadena.
/// - `position`: Referencia mutable a la posición actual en el laberinto.
/// - `bombs`: Referencia mutable al vector de bombas.
pub fn process_bomb(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    bombs: &mut Vec<Bomb>,
) {
    if let Some(next_char) = chars.next() {
        if let Some(digit) = next_char.to_digit(10) {
            let value_as_usize = digit as usize;
            if character == BOMBA_NORMAL {
                let bomb = Bomb::new((position.0, position.1), TypeBomb::Normal, value_as_usize);
                bombs.push(bomb);
            } else {
                let bomb = Bomb::new((position.0, position.1), TypeBomb::Traspaso, value_as_usize);
                bombs.push(bomb);
            }
        }
    }
}

/// Procesa un carácter como un enemigo y agrega una instancia de `Enemy` al vector `enemies`.
///
/// # Argumentos
///
/// - `character`: Carácter que representa el tipo de enemigo.
/// - `chars`: Referencia mutable a los caracteres restantes en la cadena.
/// - `position`: Referencia mutable a la posición actual en el laberinto.
/// - `enemies`: Referencia mutable al vector de enemigos.
pub fn process_enemy(
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

/// Procesa un carácter como un desvío y agrega una instancia de `Detour` al vector `detours`.
///
/// # Argumentos
///
/// - `chars`: Referencia mutable a los caracteres restantes en la cadena.
/// - `position`: Referencia mutable a la posición actual en el laberinto.
/// - `detours`: Referencia mutable al vector de desvíos.
pub fn process_detour(
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    detours: &mut Vec<Detour>,
) {
    if let Some(next_char) = chars.next() {
        let direction = match next_char {
            RIGHT => TypeDetour::Right,
            LEFT => TypeDetour::Left,
            UP => TypeDetour::Up,
            DOWN => TypeDetour::Down,
            _ => TypeDetour::Left, // Definir un valor predeterminado apropiado
        };
        let detour = Detour::new((position.0, position.1), direction);
        detours.push(detour);
    }
}

/// Procesa un carácter como un objeto en el laberinto y realiza las acciones correspondientes.
///
/// # Argumentos
///
/// - `character`: Carácter que representa el objeto.
/// - `chars`: Referencia mutable a los caracteres restantes en la cadena.
/// - `position`: Referencia mutable a la posición actual en el laberinto.
/// - `bombs`: Referencia mutable al vector de bombas.
/// - `enemies`: Referencia mutable al vector de enemigos.
/// - `detours`: Referencia mutable al vector de desvíos.
///
/// # Errores
///
/// Devuelve un error si el carácter representa un objeto inválido en el laberinto.
fn process_character(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    bombs: &mut Vec<Bomb>,
    enemies: &mut Vec<Enemy>,
    detours: &mut Vec<Detour>,
) -> Result<(), Box<dyn Error>> {
    match character {
        BOMBA_NORMAL | BOMBA_TRASPASO => {
            process_bomb(character, chars, position, bombs);
            Ok(())
        }
        ENEMY => {
            process_enemy(chars, position, enemies);
            Ok(())
        }
        DETOUR => {
            process_detour(chars, position, detours);
            Ok(())
        }
        WALL | ROCK => Ok(()),
        _ => Err(Box::new(error_objeto_invalido())),
    }
}

/// Crea una instancia de `GameData` internamente utilizando los vectores de objetos y el laberinto.
///
/// # Argumentos
///
/// - `bombs`: Vector de bombas.
/// - `enemies`: Vector de enemigos.
/// - `detours`: Vector de desvíos.
/// - `maze`: Laberinto representado como una matriz de cadenas.
///
/// # Retorna
///
/// Una instancia de `GameData` que contiene los objetos y el laberinto.
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
        maze,
        wall_interceps: false,
        rock_interceps: false,
    }
}

/// Crea objetos a partir de la cadena de contenido del archivo.
///
/// # Argumentos
///
/// - `file_contents`: Referencia mutable a la cadena de contenido del archivo.
/// - `coordinate_x`: Coordenada X del jugador.
/// - `coordinate_y`: Coordenada Y del jugador.
/// - `maze`: Laberinto representado como una matriz de cadenas.
///
/// # Errores
///
/// Devuelve un error si se encuentra un objeto inválido en el laberinto.
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
        if character == SALTO_LINEA {
            position.1 = 0;
            position.0 += 1;
        }
        if character == ESPACIO {
            position.1 += 1;
        }
        if character == SALTO_LINEA || character == VACIO {
            continue;
        };
        if character != ESPACIO {
            if let Err(_error) = process_character(
                character,
                &mut chars,
                &mut position,
                &mut bombs,
                &mut enemies,
                &mut detours,
            ) {
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

/// Realiza las acciones correspondientes a un objeto en el laberinto. Chequea que clase de objeto see ncunetran en el alcance de la bomba 
///
/// # Argumentos
///
/// - `game_data`: Referencia mutable a los datos del juego.
/// - `object`: Referencia a la cadena que representa el objeto.
/// - `new_x`: Nueva coordenada X después de un desvío.
/// - `y`: Coordenada Y actual.
/// - `typee`: Tipo de bomba (Normal o Traspaso).
/// - `interations_pending`: Iteraciones pendientes para objetos de tipo Traspaso.
/// - `bomb`: Referencia a la bomba asociada al objeto.
pub fn check_objects(
    game_data: &mut GameData,
    object: &String,
    new_x: usize,
    y: usize,
    typee: TypeBomb,
    interations_pending: usize,
    bomb: &Bomb,
) {
    if object.starts_with(DETOUR) {
        GameData::handle_detour(
            game_data,
            object,
            new_x,
            y,
            typee,
            interations_pending,
            bomb,
        )
    }
    if object.starts_with(ENEMY) {
        GameData::handle_enemy(game_data, new_x, y, bomb)
    }
    if object == "R" && typee == TypeBomb::Normal {
        GameData::handle_rock(game_data)
    }
    if object == "W" {
        GameData::handle_wall(game_data)
    }
    if object.starts_with(BOMBA_NORMAL) || object.starts_with(BOMBA_TRASPASO) {
        GameData::handle_bomb(game_data, object, new_x, y)
    }
}

/// Detona una bomba en las coordenadas especificadas en el laberinto y aplica sus efectos.
///
/// # Argumentos
///
/// - `game_data`: Referencia mutable a los datos del juego.
/// - `coordinate_x`: Coordenada X de la bomba a detonar.
/// - `coordinate_y`: Coordenada Y de la bomba a detonar.
///
/// # Errores
///
/// Devuelve un error si no se encuentra una bomba en las coordenadas especificadas.
pub fn detonar_bomb(
    game_data: &mut GameData,
    coordinate_x: usize,
    coordinate_y: usize,
) -> Result<(), Box<dyn Error>> {
    if let Some(bomb) = game_data.find_bomb(coordinate_x, coordinate_y) {
        let reach = bomb.reach;
        let tipo_bomb = bomb.typee;
        let copy_bomb = bomb.clone();

        game_data.maze[coordinate_x][coordinate_y] = "_".to_string();
        game_data.remove_bomb(coordinate_x, coordinate_y);
        game_data.apply_bomb_effect(coordinate_x, coordinate_y, reach, tipo_bomb, &copy_bomb);
    }
    Ok(())
}
