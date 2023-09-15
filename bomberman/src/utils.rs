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
