
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

pub fn process_bomba(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    bombas: &mut Vec<Bomba>,
) {
    if let Some(next_char) = chars.next() {
        if let Some(digit) = next_char.to_digit(10) {
            let value_as_usize = digit as usize;
            let bomba = Bomba {
                position: (position.0, position.1),
                typee: if character == 'B' {
                    TypeBomba::Normal
                } else {
                    TypeBomba::Traspaso
                },
                reach: value_as_usize,
            };
            bombas.push(bomba);
        }
    }
}