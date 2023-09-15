use crate::bomb::Bomb;
use crate::bomb::TypeBomb;
use crate::game_data::GameData;

/// Estructura que representa a un enemigo en el juego.
#[derive(Clone, PartialEq, Debug)]
pub struct Enemy {
    /// Posición actual del enemigo en el laberinto.
    pub position: (usize, usize),
    /// Cantidad de vidas del enemigo.
    pub lives: usize,
    /// Bombas recibidas por el enemigo (opcional).
    pub received_bombs: Option<Vec<Bomb>>,
}

impl Enemy {
    /// Constructor para crear un nuevo enemigo.
    ///
    /// # Argumentos
    ///
    /// - `position`: Posición inicial del enemigo en el laberinto.
    /// - `lives`: Cantidad de vidas iniciales del enemigo.
    ///

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
