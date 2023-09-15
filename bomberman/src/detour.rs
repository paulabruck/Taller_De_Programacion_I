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
    use crate::bomberman::*;

    #[test]
    fn test_detour_creation() {
        let detour = Detour::new((1, 2), TypeDetour::Right);
        assert_eq!(detour.position, (1, 2));
        assert_eq!(detour.direction, TypeDetour::Right);
    }

    #[test]
    fn test_process_detour() {
        let mut chars = "DR".chars();
        let mut position = (0, 0);
        let mut detours = Vec::new();

        process_detour(&mut chars, &mut position, &mut detours);

        assert_eq!(detours.len(), 1);
        assert_eq!(detours[0].position, (0, 0));
    }
}
