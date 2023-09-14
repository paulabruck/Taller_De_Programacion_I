use crate::bomberman::recorrer_hacia_abajo;
use crate::bomberman::recorrer_hacia_arriba;
use crate::bomberman::recorrer_hacia_derecha;
use crate::bomberman::recorrer_hacia_izquierda;
use crate::desvio::Detour;
use crate::enemigo::Enemigo;
use std::error::Error;
// Importa las partes que deseas probar
use crate::bomba::{Bomba, TypeBomba};

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

// Anota tu módulo de pruebas
#[cfg(test)]
mod tests {
    // Importa las partes que deseas probar en el ámbito de pruebas
    use super::*;

    // Escribe tus pruebas unitarias aquí
    #[test]
    fn test_find_bomba() {
        // Crea un juego de datos de ejemplo
        let mut game_data = GameData {
            bombas: vec![
                Bomba {
                    position: (0, 0),
                    typee: TypeBomba::Normal,
                    reach: 1,
                },
                Bomba {
                    position: (1, 1),
                    typee: TypeBomba::Traspaso,
                    reach: 2,
                },
            ],
            enemies: vec![],
            detours: vec![],
            laberinto: vec![],
            pared_intercepta: false,
            roca_intercepta: false,
        };

        // Prueba la función `find_bomba`
        assert_eq!(game_data.find_bomba(0, 0).is_some(), true);
        assert_eq!(game_data.find_bomba(1, 1).is_some(), true);
        assert_eq!(game_data.find_bomba(2, 2).is_some(), false);
    }
    #[test]
    fn test_remove_bomba() {
        // Crea un juego de datos de ejemplo con bombas
        let mut game_data = GameData {
            bombas: vec![
                Bomba {
                    position: (0, 0),
                    typee: TypeBomba::Normal,
                    reach: 1,
                },
                Bomba {
                    position: (1, 1),
                    typee: TypeBomba::Traspaso,
                    reach: 2,
                },
                Bomba {
                    position: (2, 2),
                    typee: TypeBomba::Normal,
                    reach: 3,
                },
            ],
            enemies: vec![],
            detours: vec![],
            laberinto: vec![],
            pared_intercepta: false,
            roca_intercepta: false,
        };

        // Verifica que haya tres bombas inicialmente
        assert_eq!(game_data.bombas.len(), 3);

        // Elimina una bomba
        game_data.remove_bomba(1, 1);

        // Verifica que la bomba se haya eliminado
        assert_eq!(game_data.bombas.len(), 2);

        // Verifica que la posición (1, 1) ya no esté en la lista de bombas
        assert_eq!(game_data.bombas.iter().any(|b| b.position == (1, 1)), false);
    }

    // Agrega más pruebas según sea necesario
}
