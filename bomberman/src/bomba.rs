
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TypeBomba {
    Normal,
    Traspaso,
}
#[derive(Clone)]
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

pub fn process_bomba(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    bombas: &mut Vec<Bomba>,
) {
    if let Some(next_char) = chars.next() {
        if let Some(digit) = next_char.to_digit(10) {
            let value_as_usize = digit as usize;
            if character == 'B'{
                let bomba = Bomba::new((position.0, position.1), TypeBomba::Normal, value_as_usize);
                bombas.push(bomba);

            }else {
                let bomba = Bomba::new((position.0, position.1), TypeBomba::Traspaso, value_as_usize);
                bombas.push(bomba);
            }
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