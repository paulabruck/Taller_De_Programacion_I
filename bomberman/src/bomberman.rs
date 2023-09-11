use std::error::Error;

#[derive(Debug)]
enum TypeBomba {
    Normal,
    Traspaso,
}
struct Bomba {
    position: (usize, usize),
    typee: TypeBomba,
    reach: usize,
}
fn validate_maze(bombas:Vec<Bomba>, coordinate_x: usize, coordinate_y: usize)-> Result<(), Box<dyn Error>>{
    let mut bomba_encontrada = false;
    for bomba in &bombas {
        if bomba.position == (coordinate_x, coordinate_y) {
            bomba_encontrada = true;
            match bomba.typee {
                TypeBomba::Normal => {
                    println!("¡Encontraste una bomba normal en la coordenada ({}, {})!", coordinate_x, coordinate_y);
                }
                TypeBomba::Traspaso => {
                    println!("¡Encontraste una bomba de traspaso en la coordenada ({}, {})!", coordinate_x, coordinate_y);
                }
            }
            // Haz lo que necesites hacer cuando encuentres la bomba aquí.
        }
    }
    if bomba_encontrada {
        Ok(())
    } else {
        Err("No se encontró una bomba en las coordenadas especificadas.".into())
    }
}
pub fn create_objects(file_contents: &mut str, coordinate_x: usize, coordinate_y: usize) -> Result<(), Box<dyn Error>>{
    let mut position: (usize, usize) = (0, 0);
    let mut bombas: Vec<Bomba> = Vec::new();
    let mut chars = file_contents.chars();
    while let Some(character) = chars.next(){
        if character == '\n'{
            position.1 = 0;
            position.0 += 1;
        }
        if character == ' '{
            position.1 += 1;
        }
        if character == '\n' || character == '_' {

            continue;
        };
        if  character != ' ' {   
            if character == 'B'{
                if let Some(next_char) = chars.next() {
                    if let Some(digit) = next_char.to_digit(10) {
                        let value_as_usize = digit as usize;
                        println!("Next character after 'B' is '{}', Converted to usize: {}", next_char, value_as_usize);
                        let bomba_normal = Bomba {
                            position: (position.0, position.1),
                            typee: TypeBomba::Normal,
                            reach: value_as_usize,
                        };
                        println!("Posición de la bomba normal: {:?}", bomba_normal.position);
                        bombas.push(bomba_normal);
                    }
                }
            }
            if character == 'S'{
                if let Some(next_char) = chars.next() {
                    if let Some(digit) = next_char.to_digit(10) {
                        let value_as_usize = digit as usize;
                        println!("Next character after 'S' is '{}', Converted to usize: {}", next_char, value_as_usize);
                        let bomba_traspaso = Bomba {
                            position: (position.0, position.1),
                            typee: TypeBomba::Traspaso,
                            reach: value_as_usize,
        
                        };
                        println!("Posición de la bomba traspaso: {:?}", bomba_traspaso.position);
                        bombas.push(bomba_traspaso);
                    }
                }
            }
            println!("{}", character)
        }
    }
    match validate_maze(bombas, coordinate_x, coordinate_y) {
        Ok(_) => {
            // La validación fue exitosa, continúa con tu programa
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
    Ok(())
}
