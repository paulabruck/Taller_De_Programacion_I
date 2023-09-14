#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeBomba {
    Normal,
    Traspaso,
}
#[derive(Clone, PartialEq, Debug)]
pub struct Bomba {
    pub position: (usize, usize),
    pub typee: TypeBomba,
    pub reach: usize,
}

impl Bomba {
    // Constructor para crear una nueva instancia de Bomba
    pub fn new(position: (usize, usize), typee: TypeBomba, reach: usize) -> Self {
        Bomba {
            position,
            typee,
            reach,
        }
    }
}

// Pruebas unitarias
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_bomba_normal() {
        let bomba = Bomba::new((1, 2), TypeBomba::Normal, 3);
        assert_eq!(bomba.position, (1, 2));
        assert_eq!(bomba.typee, TypeBomba::Normal);
        assert_eq!(bomba.reach, 3);
    }

    #[test]
    fn test_new_bomba_traspaso() {
        let bomba = Bomba::new((3, 4), TypeBomba::Traspaso, 5);
        assert_eq!(bomba.position, (3, 4));
        assert_eq!(bomba.typee, TypeBomba::Traspaso);
        assert_eq!(bomba.reach, 5);
    }
}
