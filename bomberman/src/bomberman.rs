use std::error::Error;

#[derive(Debug)]
enum TypeBomba {
    Normal,
    Traspaso,
}
struct Bomba {
    position: (usize, usize),
    typee: TypeBomba,
}

pub fn create_objects(file_contents: &mut str, coordinate_x: usize, coordinate_y: usize) -> Result<(), Box<dyn Error>>{
    let mut position: (usize, usize) = (0, 0);
    let  index: usize = 0;
    let mut bombas: Vec<Bomba> = Vec::new();
    //let mut object: [Box<dyn Object>] = [Vacio::new(), Vacio::new()];
     //proceso cada caracter 
     for character in file_contents.chars() {
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
                };
                println!("Posición de la bomba normal: {:?}", bomba_normal.position);
                bombas.push(bomba_normal);
            }
            if character == 'S'{
                let bomba_traspaso = Bomba {
                    position: (position.0, position.1),
                    typee: TypeBomba::Traspaso,
                };
                bombas.push(bomba_traspaso);
            }
            
            //println!("{}", character)
        }
    }
    // Supongamos que tienes un vector de bombas llamado 'bombas' que contiene todas las bombas.
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
