pub mod errores {
    use std::io::Error;
    use std::io::ErrorKind::{InvalidData, InvalidInput};

    pub fn error_objetos_invalidos() -> Error {
        Error::new(
            InvalidData,
            "[ERROR] Los objetos son invalidos ",
        )
    }
    pub fn error_objeto_invalido() -> Error {
        Error::new(
            InvalidInput,
            "[ERROR] El laberinto contiene un carácter inválido",
        )
    }
    pub fn error_path_invalido() -> Error {
        Error::new(InvalidInput, "[ERROR] No se ingresó el path del laberinto.")
    }
    pub fn error_empty_file() -> Error {
        Error::new(InvalidInput, "[ERROR] el archivo esta vacio.")
    }
}