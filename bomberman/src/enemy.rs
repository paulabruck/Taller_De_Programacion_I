use crate::bomb::Bomb;
use crate::bomb::TypeBomb;
use crate::game_data::GameData;

#[derive(Clone, PartialEq, Debug)]
pub struct Enemy {
    pub position: (usize, usize),
    pub lives: usize,
    pub received_bombs: Option<Vec<Bomb>>,
}

impl Enemy {
    // Constructor para crear un nuevo enemigo
    pub fn new(position: (usize, usize), lives: usize) -> Self {
        Enemy {
            position,
            lives,
            received_bombs: None, // Inicialmente no tiene bombas recibidas
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bomb::{Bomb, TypeBomb};
    use crate::game_data::GameData;

    #[test]
    fn test_new_enemy() {
        let enemy = Enemy::new((1, 2), 3);
        assert_eq!(enemy.position, (1, 2));
        assert_eq!(enemy.lives, 3);
        assert!(enemy.received_bombs.is_none());
    }
}
