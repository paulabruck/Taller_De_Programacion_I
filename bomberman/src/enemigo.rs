use crate::bomba::Bomba;
use crate::bomba::TypeBomba;
use crate::game_data::GameData;

#[derive(Clone, PartialEq, Debug)]
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
