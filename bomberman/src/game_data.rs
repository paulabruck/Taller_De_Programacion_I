use crate::bomba::Bomba;
use crate::bomba::TypeBomba;
use crate::bomberman::recorrer_hacia_abajo;
use crate::bomberman::recorrer_hacia_arriba;
use crate::bomberman::recorrer_hacia_derecha;
use crate::bomberman::recorrer_hacia_izquierda;
use crate::desvio::Detour;
use crate::enemigo::Enemigo;
use std::error::Error;

#[derive(Clone)]
pub struct GameData {
    pub bombas: Vec<Bomba>,
    pub enemies: Vec<Enemigo>,
    pub detours: Vec<Detour>,
    pub laberinto: Vec<Vec<String>>,
    pub pared_intercepta: bool,
    pub roca_intercepta: bool,
}
impl GameData {
    pub fn find_bomba(&mut self, coordinate_x: usize, coordinate_y: usize) -> Option<&mut Bomba> {
        self.bombas
            .iter_mut()
            .find(|b| b.position == (coordinate_x, coordinate_y))
    }
    pub fn remove_bomba(&mut self, coordinate_x: usize, coordinate_y: usize) {
        self.bombas
            .retain(|b| b.position != (coordinate_x, coordinate_y));
    }
    pub fn apply_bomba_effect(
        &mut self,
        coordinate_x: usize,
        coordinate_y: usize,
        alcance: usize,
        tipo_bomba: TypeBomba,
        bomba_copiada: &Bomba,
    ) {
        recorrer_hacia_abajo(
            self,
            coordinate_x,
            coordinate_y,
            alcance,
            tipo_bomba.clone(),
            &bomba_copiada,
        );
        recorrer_hacia_arriba(
            self,
            coordinate_x,
            coordinate_y,
            alcance,
            tipo_bomba.clone(),
            &bomba_copiada,
        );
        recorrer_hacia_derecha(
            self,
            coordinate_x,
            coordinate_y,
            alcance,
            tipo_bomba.clone(),
            &bomba_copiada,
        );
        recorrer_hacia_izquierda(
            self,
            coordinate_x,
            coordinate_y,
            alcance,
            tipo_bomba.clone(),
            &bomba_copiada,
        );
    }
    pub fn validate_maze(
        &self,
        coordinate_x: usize,
        coordinate_y: usize,
    ) -> Result<(), Box<dyn Error>> {
        let bomba_encontrada = self
            .bombas
            .iter()
            .any(|b| b.position == (coordinate_x, coordinate_y) && b.reach > 0);
        let vidas_validas = self
            .enemies
            .iter()
            .any(|enemy| enemy.lives <= 3 && enemy.lives > 0);

        if bomba_encontrada && vidas_validas {
            Ok(())
        } else {
            Err("No se encontró una bomba en las coordenadas especificadas o las vidas de los enemigos no son válidas.".into())
        }
    }
}
pub fn create_game_data(
    bombas: Vec<Bomba>,
    enemies: Vec<Enemigo>,
    detours: Vec<Detour>,
    maze: Vec<Vec<String>>,
    pared_intercepta: bool,
    roca_intercepta: bool,
) -> GameData {
    GameData {
        bombas,
        enemies,
        detours,
        laberinto: maze,
        pared_intercepta,
        roca_intercepta,
    }
}
