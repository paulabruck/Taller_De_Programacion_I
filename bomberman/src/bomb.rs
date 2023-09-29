/// Enumeración que representa el tipo de bomba.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeBomb {
    /// Bomba normal.
    Normal,
    /// Bomba de traspaso.
    Traspaso,
}

/// Estructura que representa una bomba en el juego.
#[derive(Clone, PartialEq, Debug)]
pub struct Bomb {
    /// Posición de la bomba en el laberinto.
    pub position: (usize, usize),
    /// Tipo de bomba (Normal o Traspaso).
    pub typee: TypeBomb,
    /// Alcance o rango de explosión de la bomba.
    pub reach: usize,
}

impl Bomb {
    /// Constructor para crear una nueva instancia de `Bomb`.
    ///
    /// # Argumentos
    ///
    /// * `position`: La posición de la bomba en el laberinto.
    /// * `typee`: El tipo de bomba (Normal o Traspaso).
    /// * `reach`: El alcance o rango de explosión de la bomba.

    pub fn new(position: (usize, usize), typee: TypeBomb, reach: usize) -> Self {
        Bomb {
            position,
            typee,
            reach,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bomba_normal() {
        let bomba = Bomb::new((1, 2), TypeBomb::Normal, 3);
        assert_eq!(bomba.position, (1, 2));
        assert_eq!(bomba.typee, TypeBomb::Normal);
        assert_eq!(bomba.reach, 3);
    }

    #[test]
    fn test_new_bomba_traspaso() {
        let bomba = Bomb::new((3, 4), TypeBomb::Traspaso, 5);
        assert_eq!(bomba.position, (3, 4));
        assert_eq!(bomba.typee, TypeBomb::Traspaso);
        assert_eq!(bomba.reach, 5);
    }
}