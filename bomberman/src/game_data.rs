use crate::bomb::{Bomb, TypeBomb};
use crate::bomberman::check_objects;
use crate::bomberman::detonar_bomb;
use crate::detour::Detour;
use crate::detour::TypeDetour;
use crate::enemy::Enemy;
use crate::utils::errores::error_objetos_invalidos;
use std::error::Error;

#[derive(Clone)]
pub struct GameData {
    pub bombs: Vec<Bomb>,
    pub enemies: Vec<Enemy>,
    pub detours: Vec<Detour>,
    pub maze: Vec<Vec<String>>,
    pub wall_interceps: bool,
    pub rock_interceps: bool,
}
/// Crea una nueva instancia de `GameData` con los datos especificados.
///
/// # Argumentos
///
/// - `bombs`: Vector de bombas en el juego.
/// - `enemies`: Vector de enemigos en el juego.
/// - `detours`: Vector de detours en el juego.
/// - `maze`: Laberinto del juego representado como una matriz de cadenas.
/// - `wall_interceps`: Indica si las intercepciones de muros están habilitadas.
/// - `rock_interceps`: Indica si las intercepciones de rocas están habilitadas.
///
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
    /// Encuentra una bomba en las coordenadas especificadas y devuelve una referencia mutable a ella.
    ///
    /// # Argumentos
    ///
    /// - `coordinate_x`: Coordenada X de la bomba.
    /// - `coordinate_y`: Coordenada Y de la bomba.
    ///
    pub fn find_bomb(&mut self, coordinate_x: usize, coordinate_y: usize) -> Option<&mut Bomb> {
        self.bombs
            .iter_mut()
            .find(|b| b.position == (coordinate_x, coordinate_y))
    }
    /// Elimina una bomba en las coordenadas especificadas.
    ///
    /// # Argumentos
    ///
    /// - `coordinate_x`: Coordenada X de la bomba a eliminar.
    /// - `coordinate_y`: Coordenada Y de la bomba a eliminar.
    ///
    pub fn remove_bomb(&mut self, coordinate_x: usize, coordinate_y: usize) {
        self.bombs
            .retain(|b| b.position != (coordinate_x, coordinate_y));
    }
    /// Esta función verifica los caminos posibles y aplica efectos en la dirección hacia abajo en el juego.
    ///
    /// # Argumentos
    ///
    /// * `new_x`: La nueva coordenada X a la que se moverá la función.
    /// * `game_data`: Una referencia mutable a los datos del juego.
    /// * `y`: La coordenada Y actual en la que se encuentra la función.
    /// * `typee`: El tipo de bomba que se está utilizando.
    /// * `bomb`: Una referencia a la bomba que se está utilizando.
    /// * `iterations_pending`: El número de iteraciones pendientes en la dirección hacia abajo.
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
    /// Esta función mueve al personaje hacia abajo en el laberinto y aplica efectos de bombas u obstáculos en su camino.
    ///
    /// # Argumentos
    ///
    /// * `game_data`: Una referencia mutable a los datos del juego.
    /// * `x`: La coordenada X actual del personaje.
    /// * `y`: La coordenada Y actual del personaje.
    /// * `reach`: La distancia máxima que el personaje puede moverse hacia abajo.
    /// * `typee`: El tipo de bomba que se está utilizando.
    /// * `bomb`: Una referencia a la bomba que se está utilizando.
    ///
    /// # Devolución
    ///
    /// Devuelve una referencia mutable a los datos del juego actualizados después de mover al personaje.
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
    /// Esta función aplica el efecto de una bomba en las cuatro direcciones (arriba, abajo, izquierda y derecha) desde la posición especificada.
    ///
    /// # Argumentos
    ///
    /// * `coordinate_x`: La coordenada X de la posición desde la cual se aplicará el efecto de la bomba.
    /// * `coordinate_y`: La coordenada Y de la posición desde la cual se aplicará el efecto de la bomba.
    /// * `reach`: La distancia máxima que el efecto de la bomba puede alcanzar en cada dirección.
    /// * `tipo_bomb`: El tipo de bomba que se está utilizando.
    /// * `copy_bomb`: Una referencia a la bomba que se está utilizando.
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
    /// Esta función valida el estado actual del laberinto en las coordenadas especificadas.
    ///
    /// # Argumentos
    ///
    /// * `coordinate_x`: La coordenada X a validar.
    /// * `coordinate_y`: La coordenada Y a validar.
    ///
    /// # Retorna
    ///
    /// * `Ok(())`: Si se encuentra una bomba válida en las coordenadas especificadas y si hay enemigos con vidas válidas cerca.
    /// * `Err`: Si no se cumplen las condiciones de validación, se devuelve un error con detalles sobre objetos inválidos.
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

    /// Esta función maneja una situación en la que se encuentra un objeto de desvío en el laberinto.
    ///
    /// # Argumentos
    ///
    /// * `game_data`: Una referencia mutable a los datos del juego.
    /// * `object`: Una cadena que representa el tipo de objeto de desvío ("DU", "DD", "DR", o "DL").
    /// * `new_x`: La nueva coordenada X después de seguir el objeto de desvío.
    /// * `y`: La coordenada Y actual.
    /// * `typee`: El tipo de bomba utilizada para seguir el objeto de desvío.
    /// * `iterations_pending`: El número de iteraciones pendientes para seguir el objeto de desvío.
    /// * `bomb`: Una referencia a la bomba actual.
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
    /// Esta función maneja una situación en la que se encuentra un enemigo en el laberinto.
    ///
    /// # Argumentos
    ///
    /// * `game_data`: Una referencia mutable a los datos del juego.
    /// * `object`: Una cadena que representa el tipo de enemigo.
    /// * `new_x`: La nueva coordenada X después de encontrarse con el enemigo.
    /// * `y`: La coordenada Y actual.
    /// * `typee`: El tipo de bomba utilizada para enfrentar al enemigo.
    /// * `iterations_pending`: El número de iteraciones pendientes después de enfrentarse al enemigo.
    /// * `bomb`: Una referencia a la bomba utilizada para enfrentarse al enemigo.
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
                    if !received_bombs.iter().any(|b| b.position == bomb.position) {
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

    /// Esta función establece la señal de que un objeto tipo "rock" ha interceptado el camino.
    ///
    /// # Argumentos
    ///
    /// * `game_data`: Una referencia mutable a los datos del juego.
    pub fn handle_rock(game_data: &mut GameData) {
        game_data.rock_interceps = true;
    }

    /// Esta función establece la señal de que un objeto tipo "wall" ha interceptado el camino.
    ///
    /// # Argumentos
    ///
    /// * `game_data`: Una referencia mutable a los datos del juego.
    pub fn handle_wall(game_data: &mut GameData) {
        game_data.wall_interceps = true;
    }

    /// Esta función maneja una situación en la que se encuentra una bomba en el laberinto.
    ///
    /// # Argumentos
    ///
    /// * `game_data`: Una referencia mutable a los datos del juego.
    /// * `object`: Una cadena que representa el tipo de objeto en la posición actual (e.g., "B" para bomba).
    /// * `new_x`: La nueva coordenada X después de encontrarse con la bomba.
    /// * `y`: La coordenada Y actual.
    pub fn handle_bomb(game_data: &mut GameData, object: &String, new_x: usize, y: usize) {
        // Llama a la función detonar_bomb para manejar la explosión de la bomba
        detonar_bomb(game_data, new_x, y);
    }
    /// Mueve al jugador hacia arriba en el laberinto hasta alcanzar una distancia máxima especificada o hasta encontrar un obstáculo.
    ///
    /// Esta función se encarga de mover al jugador hacia arriba en el laberinto, una casilla a la vez, hasta alcanzar una distancia máxima especificada o hasta encontrar un obstáculo como una pared o una roca. Para cada paso, verifica si la nueva posición está dentro de los límites del laberinto y realiza las comprobaciones necesarias llamando a `check_paths`. Si se encuentra con una pared o una roca, se detiene. Al final, restablece las señales de intercepción de pared y roca y devuelve una referencia mutable a los datos del juego actualizados.
    ///
    /// # Argumentos
    ///
    /// * `game_data`: Una referencia mutable a los datos del juego.
    /// * `x`: La coordenada X actual del jugador.
    /// * `y`: La coordenada Y actual del jugador.
    /// * `reach`: La distancia máxima que puede moverse hacia arriba.
    /// * `typee`: El tipo de bomba que se utiliza.
    /// * `bomb`: Una referencia a la bomba utilizada.
    ///
    /// # Devuelve
    ///
    /// Una referencia mutable a los datos del juego actualizados después del movimiento.
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
    /// Mueve al jugador hacia la derecha en el laberinto hasta alcanzar una distancia máxima especificada o hasta encontrar un obstáculo.
    ///
    /// Esta función se encarga de mover al jugador hacia la derecha en el laberinto, una casilla a la vez, hasta alcanzar una distancia máxima especificada o hasta encontrar un obstáculo como una pared o una roca. Para cada paso, verifica si la nueva posición está dentro de los límites del laberinto y realiza las comprobaciones necesarias llamando a `check_paths`. Si se encuentra con una pared o una roca, se detiene. Al final, restablece las señales de intercepción de pared y roca y devuelve una referencia mutable a los datos del juego actualizados.
    ///
    /// # Argumentos
    ///
    /// * `game_data`: Una referencia mutable a los datos del juego.
    /// * `x`: La coordenada X actual del jugador.
    /// * `y`: La coordenada Y actual del jugador.
    /// * `reach`: La distancia máxima que puede moverse hacia la derecha.
    /// * `typee`: El tipo de bomba que se utiliza.
    /// * `bomb`: Una referencia a la bomba utilizada.
    ///
    /// # Devuelve
    ///
    /// Una referencia mutable a los datos del juego actualizados después del movimiento.
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
    /// Mueve al jugador hacia la izquierda en el laberinto hasta alcanzar una distancia máxima especificada o hasta encontrar un obstáculo.
    ///
    /// Esta función se encarga de mover al jugador hacia la izquierda en el laberinto, una casilla a la vez, hasta alcanzar una distancia máxima especificada o hasta encontrar un obstáculo como una pared o una roca. Para cada paso, verifica si la nueva posición está dentro de los límites del laberinto y realiza las comprobaciones necesarias llamando a `check_paths`. Si se encuentra con una pared o una roca, se detiene. Al final, restablece las señales de intercepción de pared y roca y devuelve una referencia mutable a los datos del juego actualizados.
    ///
    /// # Argumentos
    ///
    /// * `game_data`: Una referencia mutable a los datos del juego.
    /// * `x`: La coordenada X actual del jugador.
    /// * `y`: La coordenada Y actual del jugador.
    /// * `reach`: La distancia máxima que puede moverse hacia la izquierda.
    /// * `typee`: El tipo de bomba que se utiliza.
    /// * `bomb`: Una referencia a la bomba utilizada.
    ///
    /// # Devuelve
    ///
    /// Una referencia mutable a los datos del juego actualizados después del movimiento.
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
    /// Imprime el laberinto en la consola.
    ///
    /// Esta función toma una referencia a una matriz bidimensional de cadenas que representa el laberinto y lo imprime en la consola. Cada elemento de la matriz se imprime sin espacio adicional entre ellos, y se agrega un espacio en blanco después de cada celda para separar las columnas. Además, se agrega un salto de línea al final de cada fila para separar las filas en la salida.
    ///
    /// # Argumentos
    ///
    /// * `maze`: Una referencia a la matriz bidimensional que representa el laberinto.
    ///
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_game_data() {
        let bombs = vec![
            Bomb::new((1, 2), TypeBomb::Normal, 3),
            Bomb::new((3, 4), TypeBomb::Traspaso, 2),
        ];

        let enemies = vec![Enemy::new((5, 6), 4), Enemy::new((7, 8), 2)];

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

    #[test]
    fn test_find_bomb() {
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
            Vec::new(),                        // Otras detours vacías
            vec![vec!["_".to_string(); 5]; 5], // Laberinto de 5x5 lleno de "_"
            false,
            false,
        );

        // Validamos el maze
        let result = game_data.validate_maze(1, 1);

        // Debería ser válido porque hay una bomb y un enemy válidos en las coordenadas especificadas
        assert!(result.is_ok());
    }
}
