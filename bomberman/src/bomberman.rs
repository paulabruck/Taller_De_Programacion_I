use std::error::Error;

#[derive(Debug, Clone)]
enum TypeBomba {
    Normal,
    Traspaso,
}
#[derive(Clone)]
pub struct Bomba {
    position: (usize, usize),
    typee: TypeBomba,
    reach: usize,
}
#[derive(Clone)]
pub struct Enemigo{
    position: (usize, usize),
    lives: usize,
}

#[derive(Debug, Clone)]
enum TypeDetour {
    Left,
    Right,
    Up,
    Down,
}
#[derive(Clone)]
pub struct Detour{
    position: (usize, usize),
    direction: TypeDetour,
}

pub struct GameData {
    pub bombas: Vec<Bomba>,
    pub enemies: Vec<Enemigo>,
    pub detours: Vec<Detour>,
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
pub fn create_objects(file_contents: &mut str, coordinate_x: usize, coordinate_y: usize) -> Result<GameData, Box<dyn Error>>{
    let mut position: (usize, usize) = (0, 0);
    let mut bombas: Vec<Bomba> = Vec::new();
    let mut enemies: Vec<Enemigo> = Vec::new();
    let mut detours: Vec<Detour> = Vec::new();
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
            if character == 'B' || character == 'S'{
                if let Some(next_char) = chars.next() {
                    if let Some(digit) = next_char.to_digit(10) {
                        let value_as_usize = digit as usize;
                        if character == 'B'{
                            println!("Next character after 'B' is '{}', Converted to usize: {}", next_char, value_as_usize);
                            let bomba_normal = Bomba {
                                position: (position.0, position.1),
                                typee: TypeBomba::Normal,
                                reach: value_as_usize,
                            };
                            println!("Posición de la bomba normal: {:?}", bomba_normal.position);
                            bombas.push(bomba_normal);

                        }else{
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
            }
            if character == 'F'{
                if let Some(next_char) = chars.next() {
                    if let Some(digit) = next_char.to_digit(10) {
                        let value_as_usize = digit as usize;
                        let enemy = Enemigo {
                            position: (position.0, position.1),
                            lives: value_as_usize,
                        };
                        println!("Posición del enemigo: {:?}", enemy.position);
                        enemies.push(enemy);
                    }
                }    
            }
            if character == 'D' {
                if let Some(next_char) = chars.next() {
                    if next_char == 'R'{
                        let detour = Detour {
                            position: (position.0, position.1),
                            direction: TypeDetour::Right,
                        };  
                        println!("Posición del desvio: {:?}", detour.position);
                        detours.push(detour);
                    }
                    if next_char == 'L'{
                        let detour = Detour {
                            position: (position.0, position.1),
                            direction: TypeDetour::Left,
                        };  
                        println!("Posición del desvio: {:?}", detour.position);
                        detours.push(detour);
                    }
                    if next_char == 'U'{
                        let detour = Detour {
                            position: (position.0, position.1),
                            direction: TypeDetour::Up,
                        };  
                        println!("Posición del desvio: {:?}", detour.position);
                        println!("Next character after 'D' is '{}'", next_char);
                        detours.push(detour);
                    }
                    if next_char == 'D'{
                        let detour = Detour {
                            position: (position.0, position.1),
                            direction: TypeDetour::Down,
                        };  
                        println!("Posición del desvio: {:?}", detour.position);
                        detours.push(detour);
                    }
                    
                }
            }    
            //println!("{}", character)
        }
    }
    match validate_maze(bombas.clone(), coordinate_x, coordinate_y) {
        Ok(_) => {
            // La validación fue exitosa, continúa el programa
        }
        Err(error) => {
            eprintln!("Error: {}", error);
        }
    }
    let game_data = GameData {
        bombas: bombas.clone(),
        enemies: enemies.clone(),
        detours: detours.clone(),
    };
    Ok(game_data)
}
//pub fn show_maze(file_contents: &mut str) -> Result<(), Box<dyn Error>>{