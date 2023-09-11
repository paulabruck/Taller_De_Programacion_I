// trait Objeto {
//     // Métodos comunes que deben ser implementados por los tipos que implementen el trait.

//     // Método para obtener la posición del objeto.
//     fn obtener_posicion(&self) -> (usize, usize);

//     // Método para obtener el carácter que representa al objeto.
//     fn obtener_caracter(&self) -> char;

//     // Método para verificar si es una bomba.
//     fn es_bomba(&self) -> bool;

//     // Método para verificar si es un enemigo.
//     fn es_enemigo(&self) -> bool;

//     // Método para verificar si es un desvío.
//     fn es_desvio(&self) -> bool;
// }

// // Implementación para un tipo de objeto concreto, por ejemplo, Enemigo.
// struct Enemigo {
//     x: usize,
//     y: usize,
// }

// impl Objeto for Enemigo {
//     fn obtener_posicion(&self) -> (usize, usize) {
//         (self.x, self.y)
//     }

//     fn obtener_caracter(&self) -> char {
//         'E'
//     }

//     fn es_bomba(&self) -> bool {
//         false
//     }

//     fn es_enemigo(&self) -> bool {
//         true
//     }

//     fn es_desvio(&self) -> bool {
//         false
//     }
// }

// // Implementa el trait Objeto para otros tipos de objetos de manera similar.
// pub trait Object {
//     fn position(&self) -> &(i8, i8);
// }
// ///Crea una pieza a partir de su posición y el carácter asociado
// pub fn create_object(position: (i8, i8), object: char) -> Result<Box<dyn Object>, Error> {
    
//     match object {
//         'B' => Ok(Box::new(Bomba::new(position, typee))),
//         _ => Err(error_pieza_invalida()),
//     }
// }