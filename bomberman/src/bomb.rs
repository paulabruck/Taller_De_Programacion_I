#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeBomb {
    Normal,
    Traspaso,
}
#[derive(Clone, PartialEq, Debug)]
pub struct Bomb {
    pub position: (usize, usize),
    pub typee: TypeBomb,
    pub reach: usize,
}

impl Bomb {
    // Constructor para crear una nueva instancia de Bomb
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
