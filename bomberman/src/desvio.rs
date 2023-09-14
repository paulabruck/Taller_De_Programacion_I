use crate::bomberman::recorrer_hacia_abajo;
use crate::bomberman::recorrer_hacia_arriba;
use crate::bomberman::recorrer_hacia_derecha;
use crate::bomberman::recorrer_hacia_izquierda;
use crate::bomba::TypeBomba;
use crate::bomba::Bomba;
use crate::game_data::GameData;

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

pub fn process_detour(
    character: char,
    chars: &mut std::str::Chars,
    position: &mut (usize, usize),
    detours: &mut Vec<Detour>,
) {
    if let Some(next_char) = chars.next() {
        let direction = match next_char {
            'R' => TypeDetour::Right,
            'L' => TypeDetour::Left,
            'U' => TypeDetour::Up,
            'D' => TypeDetour::Down,
            _ => TypeDetour::Left, // Definir un valor predeterminado apropiado
        };
        let detour = Detour {
            position: (position.0, position.1),
            direction,
        };
        detours.push(detour);
    }
}

pub fn handle_desvio(game_data: &mut GameData, objeto: &String, nueva_x: usize, y: usize,typee: TypeBomba, iteraciones_restantes: usize, bomba: &Bomba){

    if objeto == "DU"{
        recorrer_hacia_arriba( game_data,nueva_x, y,iteraciones_restantes, typee.clone(), &bomba);
    }
    if objeto == "DD"{
        recorrer_hacia_abajo( game_data,nueva_x, y,iteraciones_restantes, typee.clone(), &bomba);
    }
    if objeto == "DR"{
        recorrer_hacia_derecha( game_data,nueva_x, y,iteraciones_restantes, typee.clone(), &bomba);
    }
    if objeto == "DL"{
        recorrer_hacia_izquierda( game_data,nueva_x, y,iteraciones_restantes, typee.clone(), &bomba);
    }

}