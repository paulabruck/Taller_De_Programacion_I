/*
pub trait Objeto {
    fn position(&self) -> (usize, usize);
    fn caracter(&self) -> char;
    fn is_bomb(&self) -> bool;
    fn is_enemy(&self) -> bool;
    fn is_desvio(&self) -> bool;
  
}

pub fn create_object(caracter: char, position: (usize, usize), otras_propiedades: OtrasPropiedades) -> Box<dyn Objeto> {
    match caracter {
        'E' => Box::new(Enemigo::new(position, otras_propiedades)),
        'B' => Box::new(BombaNormal::new(position, otras_propiedades)),
        'S' => Box::new(BombaTraspaso::new(position, otras_propiedades)),
        'R' => Box::new(Roca::new(position, otras_propiedades)),
        'W' => Box::new(Pared::new(position, otras_propiedades)),
        'D' => Box::new(Desvio::new(position, otras_propiedades)),
        _ => Box::new(ObjetoDefault::new(position, otras_propiedades)),
    }
}
*/

// Definición del trait Objeto
trait Objeto {
    fn caracter(&self) -> char;
    fn posicion(&self) -> (usize, usize);
    fn es_bomba(&self) -> bool;
    fn es_enemigo(&self) -> bool;
    fn es_desvio(&self) -> bool;
    // Otras funciones específicas para cada tipo de objeto
}

// Implementación de Objeto para un objeto genérico
struct ObjetoGenerico {
    carac: char,
    pos: (usize, usize),
    es_b: bool,
    es_e: bool,
    es_d: bool,
}

impl Objeto for ObjetoGenerico {
    fn caracter(&self) -> char {
        self.carac
    }

    fn posicion(&self) -> (usize, usize) {
        self.pos
    }

    fn es_bomba(&self) -> bool {
        self.es_b
    }

    fn es_enemigo(&self) -> bool {
        self.es_e
    }

    fn es_desvio(&self) -> bool {
        self.es_d
    }
    // Implementa otras funciones específicas para este objeto si es necesario
}

// Implementación de crear_objeto
fn crear_objeto(caracter: char, posicion: (usize, usize)) -> Box<dyn Objeto> {
    let objeto: Box<dyn Objeto> = match caracter {
        'B' => Box::new(ObjetoGenerico {
            carac: caracter,
            pos: posicion,
            es_b: true,
            es_e: false,
            es_d: false,
        }),
        'E' => Box::new(ObjetoGenerico {
            carac: caracter,
            pos: posicion,
            es_b: false,
            es_e: true,
            es_d: false,
        }),
        'D' => Box::new(ObjetoGenerico {
            carac: caracter,
            pos: posicion,
            es_b: false,
            es_e: false,
            es_d: true,
        }),
        // Agrega más casos para otros tipos de objetos si es necesario
        _ => Box::new(ObjetoGenerico {
            carac: caracter,
            pos: posicion,
            es_b: false,
            es_e: false,
            es_d: false,
        }),
    };

    objeto
}


