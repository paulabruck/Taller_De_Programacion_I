use crate::bomba::Bomba;
use crate::bomba::TypeBomba;
use crate::game_data::GameData;

#[derive(Clone)]
pub struct Enemigo {
    pub position: (usize, usize),
    pub lives: usize,
    pub bombas_recibidas: Option<Vec<Bomba>>,
}

pub fn process_enemy(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    enemies: &mut Vec<Enemigo>,
) {
    if let Some(next_char) = chars.next() {
        if let Some(digit) = next_char.to_digit(10) {
            let value_as_usize = digit as usize;
            let enemy = Enemigo {
                position: (position.0, position.1),
                lives: value_as_usize,
                bombas_recibidas: None,
            };
            enemies.push(enemy);
        }
    }
}

pub fn handle_enemigo(
    game_data: &mut GameData,
    objeto: &String,
    nueva_x: usize,
    y: usize,
    typee: TypeBomba,
    iteraciones_restantes: usize,
    bomba: &Bomba,
) {
    if let Some(enemy) = game_data
        .enemies
        .iter_mut()
        .find(|enemy| enemy.position == (nueva_x, y))
    {
        if enemy.lives > 0 {
            if let Some(ref mut bombas_recibidas) = &mut enemy.bombas_recibidas {
                if !bombas_recibidas
                    .iter()
                    .any(|b| b.position == bomba.position)
                {
                    bombas_recibidas.push(bomba.clone());
                } else {
                    enemy.lives += 1;
                }
            } else {
                let mut new_bombas_recibidas = Vec::new();
                new_bombas_recibidas.push(bomba.clone());
                enemy.bombas_recibidas = Some(new_bombas_recibidas);
            }
            enemy.lives -= 1;
            let lives_str = enemy.lives.to_string();
            let objeto_str = "F".to_string() + &lives_str;
            game_data.laberinto[nueva_x][y] = objeto_str;
        }
        if enemy.lives == 0 {
            game_data.laberinto[nueva_x][y] = "_".to_string();
            game_data.enemies.retain(|b| b.position != (nueva_x, y));
        }
    }
}
