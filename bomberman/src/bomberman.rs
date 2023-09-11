use std::error::Error;

#[derive(Debug)]
enum TypeBomba {
    Normal,
    Traspaso,
}
struct Bomba {
    position: (usize, usize),
    typee: TypeBomba,
    reach: Option<char>,
}
fn validate_maze(bombas:Vec<Bomba>, coordinate_x: usize, coordinate_y: usize)-> Result<(), Box<dyn Error>>{
    for bomba in &bombas {
        if bomba.position == (coordinate_x, coordinate_y) {
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
    Ok(())
}
pub fn create_objects(file_contents: &mut str, coordinate_x: usize, coordinate_y: usize) -> Result<(), Box<dyn Error>>{
    let mut position: (usize, usize) = (0, 0);
    //let  index: usize = 0;
    let mut bombas: Vec<Bomba> = Vec::new();
    //let mut object: [Box<dyn Object>] = [Vacio::new(), Vacio::new()];
     //proceso cada caracter 
     for (index, character) in file_contents.char_indices() {
        // Aquí puedes procesar cada carácter individual del laberinto
        // character es el carácter actual
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
                
                let bomba_normal = Bomba {
                    position: (position.0, position.1),
                    typee: TypeBomba::Normal,
                    reach: file_contents.chars().nth(index + 1),
                };
                println!("Posición de la bomba normal: {:?}", bomba_normal.position);
                bombas.push(bomba_normal);
            }
            if character == 'S'{
                let bomba_traspaso = Bomba {
                    position: (position.0, position.1),
                    typee: TypeBomba::Traspaso,
                    reach: file_contents.chars().nth(index + 1),

                };
                bombas.push(bomba_traspaso);
            }
            
            println!("{}", character)
        }
    }

    validate_maze(bombas, coordinate_x, coordinate_y);
    Ok(())
}
