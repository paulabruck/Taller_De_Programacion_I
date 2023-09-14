use crate::bomba::Bomba;
use crate::bomba::TypeBomba;
use crate::game_data::GameData;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeDetour {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Clone, PartialEq, Debug)]
pub struct Detour {
    pub position: (usize, usize),
    pub direction: TypeDetour,
}

impl Detour {
    // Constructor para crear una nueva instancia de Detour
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

    // Define tus pruebas aquí
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
