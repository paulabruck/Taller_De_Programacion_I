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
}
