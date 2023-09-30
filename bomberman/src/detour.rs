/// Enumeración que representa los posibles tipos de desvíos.
#[derive(Debug, Clone, PartialEq)]
pub enum TypeDetour {
    Left,
    Right,
    Up,
    Down,
}

/// Estructura que representa un desvío en el juego.
#[derive(Clone, PartialEq, Debug)]
pub struct Detour {
    /// Posición del desvío en el laberinto.
    pub position: (usize, usize),
    /// Dirección del desvío.
    pub direction: TypeDetour,
}

impl Detour {
    /// Constructor para crear una nueva instancia de `Detour`.
    ///
    /// # Argumentos
    ///
    /// - `position`: Posición del desvío en el laberinto.
    /// - `direction`: Dirección del desvío.
    ///

    pub fn new(position: (usize, usize), direction: TypeDetour) -> Self {
        Detour {
            position,
            direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_right_detour_creation() {
        let detour = Detour::new((1, 2), TypeDetour::Right);
        assert_eq!(detour.position, (1, 2));
        assert_eq!(detour.direction, TypeDetour::Right);
    }
    #[test]
    fn test_left_detour_creation() {
        let detour = Detour::new((1, 2), TypeDetour::Left);
        assert_eq!(detour.position, (1, 2));
        assert_eq!(detour.direction, TypeDetour::Left);
    }
    #[test]
    fn test_up_detour_creation() {
        let detour = Detour::new((1, 2), TypeDetour::Up);
        assert_eq!(detour.position, (1, 2));
        assert_eq!(detour.direction, TypeDetour::Up);
    }
    #[test]
    fn test_down_detour_creation() {
        let detour = Detour::new((1, 2), TypeDetour::Down);
        assert_eq!(detour.position, (1, 2));
        assert_eq!(detour.direction, TypeDetour::Down);
    }
}
