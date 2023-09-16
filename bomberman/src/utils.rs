pub mod constantes {
    pub const ERROR_CREAR_OBJETOS: &str = "ERROR: [problema al crear los objetos del laberinto]";
    pub const ERROR_DETONANDO_BOMBA: &str = "ERROR: [problema al detonar la bomba]";
    pub const ERROR_GUARDANDO_RESULTADO: &str =
        "ERROR: [problema al guardar el laberinto actualizado]";
    pub const ERROR_LEER_ARCHIVO: &str = "ERROR: [problema al leer el archivo]";
    pub const BOMBA_NORMAL: char = 'B';
    pub const BOMBA_TRASPASO: char = 'S';
    pub const ENEMY: char = 'F';
    pub const ENEMY_: &str = "F";
    pub const DETOUR: char = 'D';
    pub const WALL: char = 'W';
    pub const WALL_: &str = "W";
    pub const ROCK: char = 'R';
    pub const ROCK_: &str = "R";
    pub const SALTO_LINEA: char = '\n';
    pub const VACIO: char = '_';
    pub const VACIO_: &str = "_";
    pub const ESPACIO: char = ' ';
    pub const RIGHT: char = 'R';
    pub const LEFT: char = 'L';
    pub const UP: char = 'U';
    pub const DOWN: char = 'D';
    pub const DETOUR_UP: &str = "DU";
    pub const DETOUR_DOWN: &str = "DD";
    pub const DETOUR_RIGHT: &str = "DR";
    pub const DETOUR_LEFT: &str = "DL";
}
pub mod errores {
    use std::io::Error;
    use std::io::ErrorKind::{InvalidData, InvalidInput};

    /// Crea un error para objetos inválidos.
    ///
    /// Esta función retorna un error que indica que los objetos en el laberinto son inválidos.
    pub fn error_objetos_invalidos() -> Error {
        Error::new(InvalidData, "[ERROR] Los objetos son inválidos ")
    }

    /// Crea un error para un objeto inválido.
    ///
    /// Esta función retorna un error que indica que el laberinto contiene un carácter inválido.
    pub fn error_objeto_invalido() -> Error {
        Error::new(
            InvalidInput,
            "[ERROR] El laberinto contiene un carácter inválido",
        )
    }

    /// Crea un error para un path inválido.
    ///
    /// Esta función retorna un error que indica que no se ingresó el path del laberinto.
    pub fn error_path_invalido() -> Error {
        Error::new(InvalidInput, "[ERROR] No se ingresó el path del laberinto.")
    }

    /// Crea un error para un archivo vacío.
    ///
    /// Esta función retorna un error que indica que el archivo está vacío.
    pub fn error_empty_file() -> Error {
        Error::new(InvalidInput, "[ERROR] el archivo está vacío.")
    }
}
