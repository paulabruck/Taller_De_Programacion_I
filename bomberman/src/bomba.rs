#[derive(Debug, Clone, Copy, PartialEq)]
enum TypeBomba {
    Normal,
    Traspaso,
}
#[derive(Clone)]
pub struct Bomba {
    position: (usize, usize),
    typee: TypeBomba,
    reach: usize,
}