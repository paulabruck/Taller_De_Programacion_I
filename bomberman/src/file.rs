use std::error::Error;
use std::fs::read_to_string;

pub fn read_file(input_file: &str) -> Result<String, Box<dyn Error>> {
    // Ahora 'file_contents' contiene el contenido completo del archivo como una cadena
    let file_contents = read_to_string(input_file)?;
    println!("Contenido del archivo:\n{}", file_contents);
    Ok(file_contents)
}
