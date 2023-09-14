use crate::utils::errores::error_objetos_invalidos;
use crate::bomberman::detonar_bomb;
use crate::bomberman::check_objects;
use crate::detour::Detour;
use crate::enemy::Enemy;
use std::error::Error;
use crate::detour::TypeDetour;
use crate::bomb::{Bomb, TypeBomb};

#[derive(Clone)]
pub struct GameData {
    pub bombs: Vec<Bomb>,
    pub enemies: Vec<Enemy>,
    pub detours: Vec<Detour>,
    pub maze: Vec<Vec<String>>,
    pub wall_interceps: bool,
    pub rock_interceps: bool,
}
impl GameData {
        pub fn new(
            bombs: Vec<Bomb>,
            enemies: Vec<Enemy>,
            detours: Vec<Detour>,
            maze: Vec<Vec<String>>,
            wall_interceps: bool,
            rock_interceps: bool,
        ) -> Self {
            GameData {
                bombs,
                enemies,
                detours,
                maze: maze,
                wall_interceps,
                rock_interceps,
            }
        }
        
        pub fn find_bomb(&mut self, coordinate_x: usize, coordinate_y: usize) -> Option<&mut Bomb> {
            self.bombs
            .iter_mut()
            .find(|b| b.position == (coordinate_x, coordinate_y))
        }
        pub fn remove_bomb(&mut self, coordinate_x: usize, coordinate_y: usize) {
            self.bombs
            .retain(|b| b.position != (coordinate_x, coordinate_y));
        }
        pub fn check_paths<'a>(
            new_x: usize,
            game_data: &'a mut GameData,
            y: usize,
            typee: TypeBomb,
            bomb: &'a Bomb,
            iterations_pending: usize,
        ) {
            let object = &game_data.maze[new_x][y]; // Obtener el object en la posición
            let mut game_data_clone = game_data.clone();
            check_objects(
                &mut game_data_clone,
                object,
                new_x,
                y,
                typee,
                iterations_pending,
                &bomb,
            );
            *game_data = game_data_clone;
        }
        pub fn move_down<'a>(
            game_data: &'a mut GameData,
            x: usize,
            y: usize,
            reach: usize,
            typee: TypeBomb,
            bomb: &'a Bomb,
        ) -> &'a mut GameData {
            for dx in 1..=reach {
                let new_x = x.wrapping_add(1 * dx);
                let iterations_pending = reach - dx;
                if new_x < game_data.maze.len() && y < game_data.maze[new_x].len() {
                    Self::check_paths(new_x, game_data, y, typee, bomb, iterations_pending);
                    if game_data.wall_interceps == true || game_data.rock_interceps == true {
                        break;
                    }
                } else {
                    // La nueva posición está fuera de los límites del maze, así que detenemos la búsqueda en esa dirección.
                    break;
                }
            }
            game_data.wall_interceps = false;
            game_data.rock_interceps = false;
            game_data
        }
    pub fn apply_bomb_effect(
        &mut self,
        coordinate_x: usize,
        coordinate_y: usize,
        reach: usize,
        tipo_bomb: TypeBomb,
        copy_bomb: &Bomb,
    ) {
        Self::move_down(
            self,
            coordinate_x,
            coordinate_y,
            reach,
            tipo_bomb.clone(),
            &copy_bomb,
        );
        Self::move_up(
            self,
            coordinate_x,
            coordinate_y,
            reach,
            tipo_bomb.clone(),
            &copy_bomb,
        );
        Self::move_right(
            self,
            coordinate_x,
            coordinate_y,
            reach,
            tipo_bomb.clone(),
            &copy_bomb,
        );
        Self::move_left(
            self,
            coordinate_x,
            coordinate_y,
            reach,
            tipo_bomb.clone(),
            &copy_bomb,
        );
    }
    pub fn validate_maze(
        &self,
        coordinate_x: usize,
        coordinate_y: usize,
    ) -> Result<(), Box<dyn Error>> {
        let found_bomb = self
            .bombs
            .iter()
            .any(|b| b.position == (coordinate_x, coordinate_y) && b.reach > 0);
            let vidas_validas = self
            .enemies
            .iter()
            .any(|enemy| enemy.lives <= 3 && enemy.lives > 0);
            
            if found_bomb && vidas_validas {
                Ok(())
            } else {
                return Err(Box::new(error_objetos_invalidos()));              
        }
    }

    pub fn handle_detour(
        game_data: &mut GameData,
        object: &String,
        new_x: usize,
        y: usize,
        typee: TypeBomb,
        iterations_pending: usize,
        bomb: &Bomb,
    ) {
        if object == "DU" {
            Self::move_up(
                game_data,
                new_x,
                y,
                iterations_pending,
                typee.clone(),
                &bomb,
            );
        }
        if object == "DD" {
            Self::move_down(
                game_data,
                new_x,
                y,
                iterations_pending,
                typee.clone(),
                &bomb,
            );
        }
        if object == "DR" {
            Self::move_right(
                game_data,
                new_x,
                y,
                iterations_pending,
                typee.clone(),
                &bomb,
            );
        }
        if object == "DL" {
            Self::move_left(
                game_data,
                new_x,
                y,
                iterations_pending,
                typee.clone(),
                &bomb,
            );
        }
    }

    pub fn handle_enemy(
        game_data: &mut GameData,
        object: &String,
        new_x: usize,
        y: usize,
        typee: TypeBomb,
        iterations_pending: usize,
        bomb: &Bomb,
    ) {
        if let Some(enemy) = game_data
            .enemies
            .iter_mut()
            .find(|enemy| enemy.position == (new_x, y))
        {
            if enemy.lives > 0 {
                if let Some(ref mut received_bombs) = &mut enemy.received_bombs {
                    if !received_bombs
                        .iter()
                        .any(|b| b.position == bomb.position)
                    {
                        received_bombs.push(bomb.clone());
                    } else {
                        enemy.lives += 1;
                    }
                } else {
                    let mut new_received_bombs = Vec::new();
                    new_received_bombs.push(bomb.clone());
                    enemy.received_bombs = Some(new_received_bombs);
                }
                enemy.lives -= 1;
                let lives_str = enemy.lives.to_string();
                let objeto_str = "F".to_string() + &lives_str;
                game_data.maze[new_x][y] = objeto_str;
            }
            if enemy.lives == 0 {
                game_data.maze[new_x][y] = "_".to_string();
                game_data.enemies.retain(|b| b.position != (new_x, y));
            }
        }
    }
    pub fn handle_rock(game_data: &mut GameData ){
        game_data.rock_interceps = true;

    }
    pub fn handle_wall(game_data: &mut GameData ){
        game_data.wall_interceps = true;
    }
    pub fn handle_bomb( game_data: &mut GameData,
        object: &String,
        new_x: usize,
        y: usize){
        detonar_bomb(game_data, new_x, y);
    }
    
    pub fn move_up<'a>(
        game_data: &'a mut GameData,
        x: usize,
        y: usize,
        reach: usize,
        typee: TypeBomb,
        bomb: &'a Bomb,
    ) -> &'a mut GameData {
        for dx in 1..=reach {
            let new_x = x.wrapping_sub(1 * dx);
            let iterations_pending = reach - dx;
            if new_x < game_data.maze.len() && y < game_data.maze[new_x].len() {
                Self::check_paths(new_x, game_data, y, typee, bomb, iterations_pending);
                if game_data.wall_interceps == true || game_data.rock_interceps == true {
                    break;
                }
            } else {
                // La nueva posición está fuera de los límites del maze, así que detenemos la búsqueda en esa dirección.
                break;
            }
        }
        game_data.wall_interceps = false;
        game_data.rock_interceps = false;
        game_data // Devuelve el game_data actualizado
    }

    pub fn move_right<'a>(
        game_data: &'a mut GameData,
        x: usize,
        y: usize,
        reach: usize,
        typee: TypeBomb,
        bomb: &'a Bomb,
    ) -> &'a mut GameData {
        for dx in 1..=reach {
            let nueva_y = y.wrapping_add(1 * dx);
            let iterations_pending = reach - dx;
            if x < game_data.maze.len() && nueva_y < game_data.maze[x].len() {
                Self::check_paths(x, game_data, nueva_y, typee, bomb, iterations_pending);
                if game_data.wall_interceps == true || game_data.rock_interceps == true {
                    break;
                }
            } else {
                // La nueva posición está fuera de los límites del maze, así que detenemos la búsqueda en esa dirección.
                break;
            }
        }
        game_data.wall_interceps = false;
        game_data.rock_interceps = false;
        game_data // Devuelve el game_data actualizado
    }

    pub fn move_left<'a>(
        game_data: &'a mut GameData,
        x: usize,
        y: usize,
        reach: usize,
        typee: TypeBomb,
        bomb: &'a Bomb,
    ) -> &'a mut GameData {
        for dx in 1..=reach {
            let nueva_y = y.wrapping_sub(1 * dx);
            let iterations_pending = reach - dx;
            if x < game_data.maze.len() && nueva_y < game_data.maze[x].len() {
                Self::check_paths(x, game_data, nueva_y, typee, bomb, iterations_pending);
                if game_data.wall_interceps == true || game_data.rock_interceps == true {
                    break;
                }
            } else {
                // La nueva posición está fuera de los límites del maze, así que detenemos la búsqueda en esa dirección.
                break;
            }
        }
        game_data.wall_interceps = false;
        game_data.rock_interceps = false;
        game_data // Devuelve el game_data actualizado
    }

    pub fn print_maze(maze: &Vec<Vec<String>>) {
        for row in maze {
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
        let bombs = vec![
            Bomb::new((1, 2), TypeBomb::Normal, 3),
            Bomb::new((3, 4), TypeBomb::Traspaso, 2),
        ];

        let enemies = vec![
            Enemy::new((5, 6), 4),
            Enemy::new((7, 8), 2),
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
            bombs.clone(),
            enemies.clone(),
            detours.clone(),
            maze.clone(),
            false,
            false,
        );
       

        // Verificar que los datos creados sean iguales a los datos de prueba
        assert_eq!(game_data.bombs, bombs);
        assert_eq!(game_data.enemies, enemies);
        assert_eq!(game_data.detours, detours);
        assert_eq!(game_data.maze, maze);
        assert_eq!(game_data.wall_interceps, false);
        assert_eq!(game_data.rock_interceps, false);
    }
    

    // Escribe tus pruebas unitarias aquí
    #[test]
    fn test_find_bomb() {
        // Crea un juego de datos de ejemplo
        let mut game_data = GameData {
            bombs: vec![
                Bomb {
                    position: (0, 0),
                    typee: TypeBomb::Normal,
                    reach: 1,
                },
                Bomb {
                    position: (1, 1),
                    typee: TypeBomb::Traspaso,
                    reach: 2,
                },
            ],
            enemies: vec![],
            detours: vec![],
            maze: vec![],
            wall_interceps: false,
            rock_interceps: false,
        };

        // Prueba la función `find_bomb`
        assert_eq!(game_data.find_bomb(0, 0).is_some(), true);
        assert_eq!(game_data.find_bomb(1, 1).is_some(), true);
        assert_eq!(game_data.find_bomb(2, 2).is_some(), false);
    }
    #[test]
    fn test_remove_bomb() {
        // Crea un juego de datos de ejemplo con bombs
        let mut game_data = GameData {
            bombs: vec![
                Bomb {
                    position: (0, 0),
                    typee: TypeBomb::Normal,
                    reach: 1,
                },
                Bomb {
                    position: (1, 1),
                    typee: TypeBomb::Traspaso,
                    reach: 2,
                },
                Bomb {
                    position: (2, 2),
                    typee: TypeBomb::Normal,
                    reach: 3,
                },
            ],
            enemies: vec![],
            detours: vec![],
            maze: vec![],
            wall_interceps: false,
            rock_interceps: false,
        };

        // Verifica que haya tres bombs inicialmente
        assert_eq!(game_data.bombs.len(), 3);

        // Elimina una bomb
        game_data.remove_bomb(1, 1);

        // Verifica que la bomb se haya eliminado
        assert_eq!(game_data.bombs.len(), 2);

        // Verifica que la posición (1, 1) ya no esté en la lista de bombs
        assert_eq!(game_data.bombs.iter().any(|b| b.position == (1, 1)), false);
    }
    #[test]
    fn test_validate_maze() {
        // Creamos un conjunto de datos de prueba
        let bombs = vec![Bomb {
            position: (1, 1),
            typee: TypeBomb::Normal,
            reach: 2,
        }];
        let enemies = vec![Enemy {
            position: (1, 1),
            lives: 3,
            received_bombs: None,
        }];
        let game_data = GameData::new(
            bombs.clone(),
            enemies.clone(),
            Vec::new(), // Otras detours vacías
            vec![vec!["_".to_string(); 5]; 5], // Laberinto de 5x5 lleno de "_"
            false,
            false,
        );

        // Validamos el maze
        let result = game_data.validate_maze(1, 1);

        // Debería ser válido porque hay una bomb y un enemy válidos en las coordenadas especificadas
        assert!(result.is_ok());
    }
    // #[test]
    // fn test_apply_bomb_effect() {
    //     // Crea un juego de datos de ejemplo con bombs y un maze vacío
    //     let mut game_data = GameData {
    //         bombs: vec![
    //             Bomb {
    //                 position: (1, 1),
    //                 typee: TypeBomb::Normal,
    //                 reach: 1,
    //             },
    //         ],
    //         enemies: vec![],
    //         detours: vec![],
    //         maze:  vec![
    //         vec!["_".to_string(), "_".to_string()],
    //         vec!["_".to_string(), "B1".to_string()],
    //         ], 
    //         wall_interceps: false,
    //         rock_interceps: false,
    //     };
    
    //     // Clonar la bomb para evitar el problema de referencias mutables e inmutables
    //     let copy_bomb = game_data.bombs[0].clone();
    //     GameData::print_maze( &game_data.maze);
    //     // Aplica el efecto de la bomb y verifica que la celda afectada tenga "_"
    //     game_data.apply_bomb_effect(1, 1, 1, TypeBomb::Normal, &copy_bomb);
    //     assert_eq!(game_data.maze[1][1], "_");
    // }

    // #[test]
    // fn test_handle_desvio() {
    //     // Crea un juego de datos de ejemplo con un maze vacío
    //     let mut game_data = GameData {
    //         bombs: vec![],
    //         enemies: vec![],
    //         detours: vec![],
    //         maze: vec![vec!["_".to_string(); 3]; 3],
    //         wall_interceps: false,
    //         rock_interceps: false,
    //     };

    //     // Agrega un desvío en una celda y verifica que afecte al recorrido
    //     game_data.detours.push(Detour {
    //         position: (0, 1),
    //         direction: TypeDetour::Up,
    //     });
    //     GameData::handle_detour(&mut game_data,&"DU".to_string(), 0, 1, TypeBomb::Normal, 2, &Bomb {
    //         position: (0, 1),
    //         typee: TypeBomb::Normal,
    //         reach: 2,
    //         });
    //     assert_eq!(game_data.maze[0][1], "_");

    //     // Cambia el tipo de desvío y verifica que afecte al recorrido en otra dirección
    //     game_data.detours[0].direction = TypeDetour::Down;
    //     GameData::handle_detour(&mut game_data,&"DD".to_string(), 1, 0, TypeBomb::Normal, 2, &Bomb {            position: (0, 1),
    //                      typee: TypeBomb::Normal,
    //                      reach: 2,
    //                      });
                
    //     assert_eq!(game_data.maze[0][1], "E");
    // }



}