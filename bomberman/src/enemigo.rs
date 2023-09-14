use crate::bomba::Bomba;
use crate::bomba::TypeBomba;
use crate::game_data::GameData;

#[derive(Clone)]
pub struct Enemigo {
    pub position: (usize, usize),
    pub lives: usize,
    pub bombas_recibidas: Option<Vec<Bomba>>,
}

impl Enemigo {
    // Constructor para crear un nuevo enemigo
    pub fn new(position: (usize, usize), lives: usize) -> Self {
        Enemigo {
            position,
            lives,
            bombas_recibidas: None, // Inicialmente no tiene bombas recibidas
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

pub fn handle_enemigo(
    game_data: &mut GameData,
    objeto: &String,
    nueva_x: usize,
    y: usize,
    typee: TypeBomba,
    iteraciones_restantes: usize,
    bomba: &Bomba,
) {
    if let Some(enemy) = game_data
        .enemies
        .iter_mut()
        .find(|enemy| enemy.position == (nueva_x, y))
    {
        if enemy.lives > 0 {
            if let Some(ref mut bombas_recibidas) = &mut enemy.bombas_recibidas {
                if !bombas_recibidas
                    .iter()
                    .any(|b| b.position == bomba.position)
                {
                    bombas_recibidas.push(bomba.clone());
                } else {
                    enemy.lives += 1;
                }
            } else {
                let mut new_bombas_recibidas = Vec::new();
                new_bombas_recibidas.push(bomba.clone());
                enemy.bombas_recibidas = Some(new_bombas_recibidas);
            }
            enemy.lives -= 1;
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


// enemigo_tests.rs

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bomba::{Bomba, TypeBomba};
    use crate::game_data::GameData;

    #[test]
    fn test_new_enemy() {
        let enemy = Enemigo::new((1, 2), 3);
        assert_eq!(enemy.position, (1, 2));
        assert_eq!(enemy.lives, 3);
        assert!(enemy.bombas_recibidas.is_none());
    }

    // #[test]
    // fn test_process_enemy() {
    //     let mut chars = "F4".chars();
    //     let mut position = (0, 0);
    //     let mut enemies = Vec::new();

    //     process_enemy('F', &mut chars, &mut position, &mut enemies);

    //     assert_eq!(enemies.len(), 1);
    //     let enemy = &enemies[0];
    //     assert_eq!(enemy.position, (0, 0));
    //     assert_eq!(enemy.lives, 4);
    //     assert!(enemy.bombas_recibidas.is_none());
    // }

    // #[test]
    // fn test_handle_enemigo() {
    //     let mut game_data = GameData::new(); // Asegúrate de inicializar correctamente tu GameData aquí

    //     let objeto = "F3".to_string();
    //     let nueva_x = 1;
    //     let y = 2;
    //     let typee = TypeBomba::Normal;
    //     let iteraciones_restantes = 2;
    //     let bomba = Bomba::new((1, 2), TypeBomba::Normal, 1);

    //     handle_enemigo(
    //         &mut game_data,
    //         &objeto,
    //         nueva_x,
    //         y,
    //         typee,
    //         iteraciones_restantes,
    //         &bomba,
    //     );

        // Realiza las aserciones necesarias para verificar que handle_enemigo funcionó como se esperaba
   // }
}
