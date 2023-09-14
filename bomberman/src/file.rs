use std::error::Error;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

pub fn read_file(input_file: &str) -> Result<String, Box<dyn Error>> {
    // Ahora 'file_contents' contiene el contenido completo del archivo como una cadena
    let file_contents = read_to_string(input_file)?;
    println!("Contenido del archivo:\n{}", file_contents);
    Ok(file_contents)
}

pub fn guardar_laberinto_en_archivo(
    laberinto: &Vec<Vec<String>>,
    ruta: &str,
) -> Result<(), std::io::Error> {
    let mut file = File::create(ruta)?;
    for row in laberinto {
        for cell in row {
            file.write_all(cell.as_bytes())?;
            file.write_all(b" ")?;
        }
        file.write_all(b"\n")?; // Salto de línea para separar las filas
    }
    Ok(())
}
