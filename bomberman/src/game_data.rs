use crate::utils::errores::error_objetos_invalidos;
use crate::bomberman::detonar_bomba;
use crate::bomberman::chequear_objetos;
use crate::desvio::Detour;
use crate::enemigo::Enemigo;
use std::error::Error;
use crate::desvio::TypeDetour;
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
        pub fn new(
            bombas: Vec<Bomba>,
            enemies: Vec<Enemigo>,
            detours: Vec<Detour>,
            maze: Vec<Vec<String>>,
            pared_intercepta: bool,
            roca_intercepta: bool,
        ) -> Self {
            GameData {
                bombas,
                enemies,
                detours,
                laberinto: maze,
                pared_intercepta,
                roca_intercepta,
            }
        }
        
        pub fn find_bomba(&mut self, coordinate_x: usize, coordinate_y: usize) -> Option<&mut Bomba> {
            self.bombas
            .iter_mut()
            .find(|b| b.position == (coordinate_x, coordinate_y))
        }
        pub fn remove_bomba(&mut self, coordinate_x: usize, coordinate_y: usize) {
            self.bombas
            .retain(|b| b.position != (coordinate_x, coordinate_y));
        }
        pub fn chequeos_recorridos<'a>(
            nueva_x: usize,
            game_data: &'a mut GameData,
            y: usize,
            typee: TypeBomba,
            bomba: &'a Bomba,
            iteraciones_restantes: usize,
        ) {
            let objeto = &game_data.laberinto[nueva_x][y]; // Obtener el objeto en la posición
            let mut game_data_clone = game_data.clone();
            chequear_objetos(
                &mut game_data_clone,
                objeto,
                nueva_x,
                y,
                typee,
                iteraciones_restantes,
                &bomba,
            );
            *game_data = game_data_clone;
        }
        pub fn recorrer_hacia_abajo<'a>(
            game_data: &'a mut GameData,
            x: usize,
            y: usize,
            alcance: usize,
            typee: TypeBomba,
            bomba: &'a Bomba,
        ) -> &'a mut GameData {
            for dx in 1..=alcance {
                let nueva_x = x.wrapping_add(1 * dx);
                let iteraciones_restantes = alcance - dx;
                if nueva_x < game_data.laberinto.len() && y < game_data.laberinto[nueva_x].len() {
                    Self::chequeos_recorridos(nueva_x, game_data, y, typee, bomba, iteraciones_restantes);
                    if game_data.pared_intercepta == true || game_data.roca_intercepta == true {
                        break;
                    }
                } else {
                    // La nueva posición está fuera de los límites del laberinto, así que detenemos la búsqueda en esa dirección.
                    break;
                }
            }
            game_data.pared_intercepta = false;
            game_data.roca_intercepta = false;
            game_data
        }
    pub fn apply_bomba_effect(
        &mut self,
        coordinate_x: usize,
        coordinate_y: usize,
        alcance: usize,
        tipo_bomba: TypeBomba,
        bomba_copiada: &Bomba,
    ) {
        Self::recorrer_hacia_abajo(
            self,
            coordinate_x,
            coordinate_y,
            alcance,
            tipo_bomba.clone(),
            &bomba_copiada,
        );
        Self::recorrer_hacia_arriba(
            self,
            coordinate_x,
            coordinate_y,
            alcance,
            tipo_bomba.clone(),
            &bomba_copiada,
        );
        Self::recorrer_hacia_derecha(
            self,
            coordinate_x,
            coordinate_y,
            alcance,
            tipo_bomba.clone(),
            &bomba_copiada,
        );
        Self::recorrer_hacia_izquierda(
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
                return Err(Box::new(error_objetos_invalidos()));              
        }
    }

    pub fn handle_desvio(
        game_data: &mut GameData,
        objeto: &String,
        nueva_x: usize,
        y: usize,
        typee: TypeBomba,
        iteraciones_restantes: usize,
        bomba: &Bomba,
    ) {
        if objeto == "DU" {
            Self::recorrer_hacia_arriba(
                game_data,
                nueva_x,
                y,
                iteraciones_restantes,
                typee.clone(),
                &bomba,
            );
        }
        if objeto == "DD" {
            Self::recorrer_hacia_abajo(
                game_data,
                nueva_x,
                y,
                iteraciones_restantes,
                typee.clone(),
                &bomba,
            );
        }
        if objeto == "DR" {
            Self::recorrer_hacia_derecha(
                game_data,
                nueva_x,
                y,
                iteraciones_restantes,
                typee.clone(),
                &bomba,
            );
        }
        if objeto == "DL" {
            Self::recorrer_hacia_izquierda(
                game_data,
                nueva_x,
                y,
                iteraciones_restantes,
                typee.clone(),
                &bomba,
            );
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
    pub fn handle_roca(game_data: &mut GameData ){
        game_data.roca_intercepta = true;

    }
    pub fn handle_pared(game_data: &mut GameData ){
        game_data.pared_intercepta = true;
    }
    pub fn handle_bomba( game_data: &mut GameData,
        objeto: &String,
        nueva_x: usize,
        y: usize){
        detonar_bomba(game_data, nueva_x, y);
    }
    
    pub fn recorrer_hacia_arriba<'a>(
        game_data: &'a mut GameData,
        x: usize,
        y: usize,
        alcance: usize,
        typee: TypeBomba,
        bomba: &'a Bomba,
    ) -> &'a mut GameData {
        for dx in 1..=alcance {
            let nueva_x = x.wrapping_sub(1 * dx);
            let iteraciones_restantes = alcance - dx;
            if nueva_x < game_data.laberinto.len() && y < game_data.laberinto[nueva_x].len() {
                Self::chequeos_recorridos(nueva_x, game_data, y, typee, bomba, iteraciones_restantes);
                if game_data.pared_intercepta == true || game_data.roca_intercepta == true {
                    break;
                }
            } else {
                // La nueva posición está fuera de los límites del laberinto, así que detenemos la búsqueda en esa dirección.
                break;
            }
        }
        game_data.pared_intercepta = false;
        game_data.roca_intercepta = false;
        game_data // Devuelve el game_data actualizado
    }

    pub fn recorrer_hacia_derecha<'a>(
        game_data: &'a mut GameData,
        x: usize,
        y: usize,
        alcance: usize,
        typee: TypeBomba,
        bomba: &'a Bomba,
    ) -> &'a mut GameData {
        for dx in 1..=alcance {
            let nueva_y = y.wrapping_add(1 * dx);
            let iteraciones_restantes = alcance - dx;
            if x < game_data.laberinto.len() && nueva_y < game_data.laberinto[x].len() {
                Self::chequeos_recorridos(x, game_data, nueva_y, typee, bomba, iteraciones_restantes);
                if game_data.pared_intercepta == true || game_data.roca_intercepta == true {
                    break;
                }
            } else {
                // La nueva posición está fuera de los límites del laberinto, así que detenemos la búsqueda en esa dirección.
                break;
            }
        }
        game_data.pared_intercepta = false;
        game_data.roca_intercepta = false;
        game_data // Devuelve el game_data actualizado
    }

    pub fn recorrer_hacia_izquierda<'a>(
        game_data: &'a mut GameData,
        x: usize,
        y: usize,
        alcance: usize,
        typee: TypeBomba,
        bomba: &'a Bomba,
    ) -> &'a mut GameData {
        for dx in 1..=alcance {
            let nueva_y = y.wrapping_sub(1 * dx);
            let iteraciones_restantes = alcance - dx;
            if x < game_data.laberinto.len() && nueva_y < game_data.laberinto[x].len() {
                Self::chequeos_recorridos(x, game_data, nueva_y, typee, bomba, iteraciones_restantes);
                if game_data.pared_intercepta == true || game_data.roca_intercepta == true {
                    break;
                }
            } else {
                // La nueva posición está fuera de los límites del laberinto, así que detenemos la búsqueda en esa dirección.
                break;
            }
        }
        game_data.pared_intercepta = false;
        game_data.roca_intercepta = false;
        game_data // Devuelve el game_data actualizado
    }

    pub fn print_laberinto(laberinto: &Vec<Vec<String>>) {
        for row in laberinto {
            for cell in row {
                print!("{}", cell);
                print!(" ");
            }
            println!(); // Salto de línea para separar las filas
        }
    }
}


// Anota tu módulo de pruebas
#[cfg(test)]
mod tests {
    // Importa las partes que deseas probar en el ámbito de pruebas
    use super::*;

    #[test]
    fn test_create_game_data() {
        // Definir datos de prueba
        let bombas = vec![
            Bomba::new((1, 2), TypeBomba::Normal, 3),
            Bomba::new((3, 4), TypeBomba::Traspaso, 2),
        ];

        let enemies = vec![
            Enemigo::new((5, 6), 4),
            Enemigo::new((7, 8), 2),
        ];

        let detours = vec![
            Detour::new((9, 10), TypeDetour::Right),
            Detour::new((11, 12), TypeDetour::Left),
        ];

        let maze = vec![
            vec!["X".to_string(), " ".to_string()],
            vec!["W".to_string(), "D".to_string()],
        ];
        let game_data = GameData::new(
            bombas.clone(),
            enemies.clone(),
            detours.clone(),
            maze.clone(),
            false,
            false,
        );
       

        // Verificar que los datos creados sean iguales a los datos de prueba
        assert_eq!(game_data.bombas, bombas);
        assert_eq!(game_data.enemies, enemies);
        assert_eq!(game_data.detours, detours);
        assert_eq!(game_data.laberinto, maze);
        assert_eq!(game_data.pared_intercepta, false);
        assert_eq!(game_data.roca_intercepta, false);
    }
    

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
    #[test]
    fn test_validate_maze() {
        // Creamos un conjunto de datos de prueba
        let bombas = vec![Bomba {
            position: (1, 1),
            typee: TypeBomba::Normal,
            reach: 2,
        }];
        let enemies = vec![Enemigo {
            position: (1, 1),
            lives: 3,
            bombas_recibidas: None,
        }];
        let game_data = GameData::new(
            bombas.clone(),
            enemies.clone(),
            Vec::new(), // Otras detours vacías
            vec![vec!["_".to_string(); 5]; 5], // Laberinto de 5x5 lleno de "_"
            false,
            false,
        );

        // Validamos el laberinto
        let result = game_data.validate_maze(1, 1);

        // Debería ser válido porque hay una bomba y un enemigo válidos en las coordenadas especificadas
        assert!(result.is_ok());
    }
    // #[test]
    // fn test_apply_bomba_effect() {
    //     // Crea un juego de datos de ejemplo con bombas y un laberinto vacío
    //     let mut game_data = GameData {
    //         bombas: vec![
    //             Bomba {
    //                 position: (1, 1),
    //                 typee: TypeBomba::Normal,
    //                 reach: 1,
    //             },
    //         ],
    //         enemies: vec![],
    //         detours: vec![],
    //         laberinto:  vec![
    //         vec!["_".to_string(), "_".to_string()],
    //         vec!["_".to_string(), "B1".to_string()],
    //         ], 
    //         pared_intercepta: false,
    //         roca_intercepta: false,
    //     };
    
    //     // Clonar la bomba para evitar el problema de referencias mutables e inmutables
    //     let bomba_copiada = game_data.bombas[0].clone();
    //     GameData::print_laberinto( &game_data.laberinto);
    //     // Aplica el efecto de la bomba y verifica que la celda afectada tenga "_"
    //     game_data.apply_bomba_effect(1, 1, 1, TypeBomba::Normal, &bomba_copiada);
    //     assert_eq!(game_data.laberinto[1][1], "_");
    // }

    // #[test]
    // fn test_handle_desvio() {
    //     // Crea un juego de datos de ejemplo con un laberinto vacío
    //     let mut game_data = GameData {
    //         bombas: vec![],
    //         enemies: vec![],
    //         detours: vec![],
    //         laberinto: vec![vec!["_".to_string(); 3]; 3],
    //         pared_intercepta: false,
    //         roca_intercepta: false,
    //     };

    //     // Agrega un desvío en una celda y verifica que afecte al recorrido
    //     game_data.detours.push(Detour {
    //         position: (0, 1),
    //         direction: TypeDetour::Up,
    //     });
    //     GameData::handle_desvio(&mut game_data,&"DU".to_string(), 0, 1, TypeBomba::Normal, 2, &Bomba {
    //         position: (0, 1),
    //         typee: TypeBomba::Normal,
    //         reach: 2,
    //         });
    //     assert_eq!(game_data.laberinto[0][1], "_");

    //     // Cambia el tipo de desvío y verifica que afecte al recorrido en otra dirección
    //     game_data.detours[0].direction = TypeDetour::Down;
    //     GameData::handle_desvio(&mut game_data,&"DD".to_string(), 1, 0, TypeBomba::Normal, 2, &Bomba {            position: (0, 1),
    //                      typee: TypeBomba::Normal,
    //                      reach: 2,
    //                      });
                
    //     assert_eq!(game_data.laberinto[0][1], "E");
    // }



}