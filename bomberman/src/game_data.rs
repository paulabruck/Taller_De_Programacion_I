use crate::bomb::{Bomb, TypeBomb};
use crate::detour::{Detour, TypeDetour};
use crate::enemy::Enemy;
use crate::utils::constantes::*;
use crate::utils::errores::{error_objetos_invalidos, error_objeto_invalido };
use std::error::Error;
use std::collections::HashMap;

/// Enumeración que representa las direcciones posibles en el juego.
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum TypeDirection {
    /// Dirección hacia la izquierda.
    Left,
    /// Dirección hacia la derecha.
    Right,
    /// Dirección hacia arriba.
    Up,
    /// Dirección hacia abajo.
    Down,
    /// Dirección nula o sin movimiento.
    None,
}

/// `GameData` es una estructura que almacena los datos del juego
///
/// Esta estructura es utilizada para gestionar el estado del juego y proporciona un
/// conjunto de campos públicos que permiten el acceso a los datos relevantes.
///
#[derive(Clone)]
pub struct GameData {
    /// Vector que almacena las bombas presentes en el juego.
    pub bombs: Vec<Bomb>,
    /// Vector que almacena los enemigos presentes en el juego.
    pub enemies: Vec<Enemy>,
    /// Vector que almacena los desvíos presentes en el juego.
    pub detours: Vec<Detour>,
    /// Matriz que representa el laberinto del juego.
    pub maze: Vec<Vec<String>>,
    /// HashMap que indica si una pared o roca está interceptando una ráfaga.
    pub interceps_map: HashMap<String, bool>,
    /// Enum que representa la dirección actual del movimiento.
    pub actual_direction: TypeDirection,
    /// HashMap que indica si una dirección está bloqueada.
    pub block_map: HashMap<String, bool>,
}


/// Crea una nueva instancia de `GameData` con los datos proporcionados.
///
/// # Argumentos
///
/// * `bombs`: Vector que almacena las bombas presentes en el juego.
/// * `enemies`: Vector que almacena los enemigos presentes en el juego.
/// * `detours`: Vector que almacena los desvíos presentes en el juego.
/// * `maze`: Matriz que representa el laberinto del juego.
/// * `actual_direction`: Enum que representa la dirección actual del jugador.
/// 
impl GameData {
    pub fn new(
        bombs: Vec<Bomb>,
        enemies: Vec<Enemy>,
        detours: Vec<Detour>,
        maze: Vec<Vec<String>>,
        actual_direction: TypeDirection,
    ) -> Self {
        let mut block_map = HashMap::new();
        block_map.insert("Down".to_string(), false);
        block_map.insert("Left".to_string(), false);
        block_map.insert("Right".to_string(), false);
        block_map.insert("Up".to_string(), false);
        let mut interceps_map = HashMap::new();
        interceps_map.insert("Wall".to_string(), false);
        interceps_map.insert("Rock".to_string(), false);
       
        GameData {
            bombs,
            enemies,
            detours,
            maze,
            actual_direction,
            block_map,
            interceps_map,
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

    /// Verifica que objeto es el que se encuentra en la posicion segun el recorrido que se esta realizando
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
            bomb,
        );
        *game_data = game_data_clone;
    }

    ///  aplica el efecto de una bomba en las cuatro direcciones (arriba, abajo, izquierda y derecha) desde la posición especificada. Tambien modifica la direccion actual para que coincida con la que se sta usando para recorrer.
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
        self.actual_direction = TypeDirection::Down;
        Self::move_down(
            self,
            coordinate_x,
            coordinate_y,
            reach,
            tipo_bomb,
            copy_bomb,
        );
        
        self.actual_direction = TypeDirection::Up;
        Self::move_up(
            self,
            coordinate_x,
            coordinate_y,
            reach,
            tipo_bomb,
            copy_bomb,
        );
        self.actual_direction = TypeDirection::Right;
        Self::move_right(
            self,
            coordinate_x,
            coordinate_y,
            reach,
            tipo_bomb,
            copy_bomb,
        );
        self.actual_direction = TypeDirection::Left;
        Self::move_left(
            self,
            coordinate_x,
            coordinate_y,
            reach,
            tipo_bomb,
            copy_bomb,
        );
    }

    ///  valida el estado actual del laberinto en las coordenadas especificadas.
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

        let has_invalid_enemy = self
            .enemies
            .iter()
            .any(|enemy| enemy.lives > 3 || enemy.lives == 0);

        if found_bomb && !has_invalid_enemy {
            Ok(())
        } else {
            Err(Box::new(error_objetos_invalidos()))
        }
    }

    ///  maneja una situación en la que se encuentra un objeto de desvío en el laberinto.
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
      
        update_block_map( game_data, "Down");
        update_block_map( game_data, "Up");
        update_block_map( game_data, "Left");
        update_block_map( game_data, "Right");
        if object == DETOUR_UP {
            game_data.actual_direction = TypeDirection::Up;
            Self::move_up(game_data, new_x, y, iterations_pending, typee, bomb);
        }
        if object == DETOUR_DOWN {
            game_data.actual_direction = TypeDirection::Down;
            Self::move_down(game_data, new_x, y, iterations_pending, typee, bomb);
        }
        if object == DETOUR_RIGHT {
            game_data.actual_direction = TypeDirection::Right;
            Self::move_right(game_data, new_x, y, iterations_pending, typee, bomb);
        }
        if object == DETOUR_LEFT {
            game_data.actual_direction = TypeDirection::Left;
            Self::move_left(game_data, new_x, y, iterations_pending, typee, bomb);
        }
    }

    ///  maneja una situación en la que se encuentra un enemigo en el laberinto.
    ///
    /// # Argumentos
    ///
    /// * `game_data`: Una referencia mutable a los datos del juego.
    /// * `new_x`: La nueva coordenada X después de encontrarse con el enemigo.
    /// * `y`: La coordenada Y actual.
    /// * `typee`: El tipo de bomba utilizada para enfrentar al enemigo.
    /// * `iterations_pending`: El número de iteraciones pendientes después de enfrentarse al enemigo.
    /// * `bomb`: Una referencia a la bomba utilizada para enfrentarse al enemigo.
    pub fn handle_enemy(game_data: &mut GameData, new_x: usize, y: usize, bomb: &Bomb) {
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
                    let new_received_bombs = vec![bomb.clone()];
                    enemy.received_bombs = Some(new_received_bombs);
                }
                enemy.lives -= 1;
                let lives_str = enemy.lives.to_string();
                let objeto_str = ENEMY_.to_string() + &lives_str;
                game_data.maze[new_x][y] = objeto_str;
            }
            if enemy.lives == 0 {
                game_data.maze[new_x][y] = VACIO_.to_string();
                game_data.enemies.retain(|b| b.position != (new_x, y));
            }
        }
    }

    ///  establece la señal de que un objeto tipo "rock" ha interceptado el camino.
    ///
    /// # Argumentos
    ///
    /// * `game_data`: Una referencia mutable a los datos del juego.
    pub fn handle_rock(game_data: &mut GameData) {
        let Some(value) = game_data.interceps_map.get_mut("Rock")else { todo!() };
        *value = true;
    }

    ///  establece la señal de que un objeto tipo "wall" ha interceptado el camino.
    ///
    /// # Argumentos
    ///
    /// * `game_data`: Una referencia mutable a los datos del juego.
    pub fn handle_wall(game_data: &mut GameData) {
        let Some(value) = game_data.interceps_map.get_mut("Wall")else { todo!() };
        *value = true;
       
    }

    ///  maneja una situación en la que se encuentra una bomba en el laberinto.
    ///
    /// # Argumentos
    ///
    /// * `game_data`: Una referencia mutable a los datos del juego.
    /// * `object`: Una cadena que representa el tipo de objeto en la posición actual (e.g., "B" para bomba).
    /// * `new_x`: La nueva coordenada X después de encontrarse con la bomba.
    /// * `y`: La coordenada Y actual.
    pub fn handle_bomb(game_data: &mut GameData, _object: &str, new_x: usize, y: usize) {
        let _ = detonar_bomb(game_data, new_x, y);
    }

    ///  mueve a la rafaga hacia abajo en el laberinto y aplica efectos de bombas y obstáculos en su camino.
    ///
    /// # Argumentos
    ///
    /// * `game_data`: Una referencia mutable a los datos del juego.
    /// * `x`: La coordenada X actual de la rafaga.
    /// * `y`: La coordenada Y actual de la rafaga.
    /// * `reach`: La distancia máxima que la rafaga puede moverse hacia abajo.
    /// * `typee`: El tipo de bomba que se está utilizando.
    /// * `bomb`: Una referencia a la bomba que se está utilizando.
    ///
    /// # Devolución
    ///
    /// Devuelve una referencia mutable a los datos del juego actualizados después de mover a la rafaga.
    pub fn move_down(
        game_data: &mut GameData,
        x: usize,
        y: usize,
        reach: usize,
        typee: TypeBomb,
        bomb: &Bomb,
    ) {
        for dx in 1..=reach {
            let new_x = x.wrapping_add(dx);
            let iterations_pending = reach - dx;
            if new_x < game_data.maze.len() && y < game_data.maze[new_x].len() {
                Self::check_paths(new_x, game_data, y, typee, bomb, iterations_pending);
                let Some(value) = game_data.block_map.get_mut("Down")else { todo!() };
                let mut wall = false;
                let mut rock = false;
                if let Some(w) = game_data.interceps_map.get_mut("Wall") {
                    wall = *w;
                }
                if let Some(r) = game_data.interceps_map.get_mut("Rock") {
                    rock = *r;
                }
                if wall || rock || *value {
                    *value = false;
                    break;
                }                
            } else {
                break;
            }
        }
        for (_, value) in game_data.interceps_map.iter_mut() {
            *value = false;
        }
    }


    /// Mueve al jugador hacia arriba en el laberinto hasta alcanzar una distancia máxima especificada o hasta encontrar un obstáculo.
    ///
    ///  se encarga de mover al jugador hacia arriba en el laberinto, una casilla a la vez, hasta alcanzar una distancia máxima especificada o hasta encontrar un obstáculo como una pared o una roca. Para cada paso, verifica si la nueva posición está dentro de los límites del laberinto y realiza las comprobaciones necesarias llamando a `check_paths`. Si se encuentra con una pared o una roca, se detiene. Al final, restablece las señales de intercepción de pared y roca y devuelve una referencia mutable a los datos del juego actualizados.
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
    pub fn move_up(
        game_data: &mut GameData,
        x: usize,
        y: usize,
        reach: usize,
        typee: TypeBomb,
        bomb: &Bomb,
    ) {
        for dx in 1..=reach {
            let new_x = x.wrapping_sub(dx);
            let iterations_pending = reach - dx;
            if new_x < game_data.maze.len() && y < game_data.maze[new_x].len() {
                Self::check_paths(new_x, game_data, y, typee, bomb, iterations_pending);
                let Some(value) = game_data.block_map.get_mut("Up")else { todo!() };
                let mut wall = false;
                let mut rock = false;
                if let Some(w) = game_data.interceps_map.get_mut("Wall") {
                    wall = *w;
                }
                if let Some(r) = game_data.interceps_map.get_mut("Rock") {
                    rock = *r;
                }
                if wall || rock || *value {
                    break;
                }                
            } else {
                break;
            }
        }
        if let Some(value) = game_data.block_map.get_mut("Up") {
            *value = false;
        }
        for (_, value) in game_data.interceps_map.iter_mut() {
            *value = false;
        }
    }

    /// Mueve al jugador hacia la derecha en el laberinto hasta alcanzar una distancia máxima especificada o hasta encontrar un obstáculo.
    ///
    ///  se encarga de mover al jugador hacia la derecha en el laberinto, una casilla a la vez, hasta alcanzar una distancia máxima especificada o hasta encontrar un obstáculo como una pared o una roca. Para cada paso, verifica si la nueva posición está dentro de los límites del laberinto y realiza las comprobaciones necesarias llamando a `check_paths`. Si se encuentra con una pared o una roca, se detiene. Al final, restablece las señales de intercepción de pared y roca y devuelve una referencia mutable a los datos del juego actualizados.
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
    pub fn move_right(
        game_data: &mut GameData,
        x: usize,
        y: usize,
        reach: usize,
        typee: TypeBomb,
        bomb: &Bomb,
    ) {
        for dy in 1..=reach {
            let new_y = y.wrapping_add(dy);
            let iterations_pending = reach - dy;
            if x < game_data.maze.len() && new_y < game_data.maze[x].len() {
                Self::check_paths(x, game_data, new_y, typee, bomb, iterations_pending);
                let Some(value) = game_data.block_map.get_mut("Right")else { todo!() };
                let mut wall = false;
                let mut rock = false;
                if let Some(w) = game_data.interceps_map.get_mut("Wall") {
                    wall = *w;
                }
                if let Some(r) = game_data.interceps_map.get_mut("Rock") {
                    rock = *r;
                }
                if wall || rock || *value {
                    break;
                }
                
            } else {
                break;
            }
        }
       if let Some(value) = game_data.block_map.get_mut("Right") {
            *value = false;
        }
        for (_, value) in game_data.interceps_map.iter_mut() {
            *value = false;
        }
    }

    /// Mueve al jugador hacia la izquierda en el laberinto hasta alcanzar una distancia máxima especificada o hasta encontrar un obstáculo.
    ///
    ///  se encarga de mover al jugador hacia la izquierda en el laberinto, una casilla a la vez, hasta alcanzar una distancia máxima especificada o hasta encontrar un obstáculo como una pared o una roca. Para cada paso, verifica si la nueva posición está dentro de los límites del laberinto y realiza las comprobaciones necesarias llamando a `check_paths`. Si se encuentra con una pared o una roca, se detiene. Al final, restablece las señales de intercepción de pared y roca y devuelve una referencia mutable a los datos del juego actualizados.
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
    pub fn move_left(
        game_data: &mut GameData,
        x: usize,
        y: usize,
        reach: usize,
        typee: TypeBomb,
        bomb: &Bomb,
    ) {
        for dy in 1..=reach {
            let new_y = y.wrapping_sub(dy);
            let iterations_pending = reach - dy;
            if x < game_data.maze.len() && new_y < game_data.maze[x].len() {
                Self::check_paths(x, game_data, new_y, typee, bomb, iterations_pending);
                let Some(value) = game_data.block_map.get_mut("Left")else { todo!() };
                let mut wall = false;
                let mut rock = false;
                if let Some(w) = game_data.interceps_map.get_mut("Wall") {
                    wall = *w;
                }
                if let Some(r) = game_data.interceps_map.get_mut("Rock") {
                    rock = *r;
                }
                if wall || rock || *value {
                    break;
                }
               
            } else {
                break;
            }
        }
       if let Some(value) = game_data.block_map.get_mut("Left") {
            *value = false;
        }
        for (_, value) in game_data.interceps_map.iter_mut() {
            *value = false;
        }
    }

    /// Imprime el laberinto en la consola.
    ///
    ///  toma una referencia a una matriz bidimensional de cadenas que representa el laberinto y lo imprime en la consola. Cada elemento de la matriz se imprime sin espacio adicional entre ellos, y se agrega un espacio en blanco después de cada celda para separar las columnas. Además, se agrega un salto de línea al final de cada fila para separar las filas en la salida.
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
            println!();
        }
    }
}


pub fn update_block_map(game_data: &mut GameData, direction: &str) {
    if game_data.actual_direction == TypeDirection::Down
        || game_data.actual_direction == TypeDirection::Up
        || game_data.actual_direction == TypeDirection::Left
        || game_data.actual_direction == TypeDirection::Right
    {
        if let Some(value) = game_data.block_map.get_mut(direction) {
            *value = true;
        }
    }
}

/// Procesa un carácter como una bomba y agrega una instancia de `Bomb` al vector `bombs`.
///
/// # Argumentos
///
/// - `character`: Carácter que representa el tipo de bomba ('B' para Normal, 'S' para Traspaso).
/// - `chars`: Referencia mutable a los caracteres restantes en la cadena.
/// - `position`: Referencia mutable a la posición actual en el laberinto.
/// - `bombs`: Referencia mutable al vector de bombas.
pub fn process_bomb(
    maze: &Vec<Vec<String>>, // Cambia el parámetro a una referencia a la matriz maze
    character: &str,
    position: &mut (usize, usize),
    bombs: &mut Vec<Bomb>,
) -> Result<(), Box<dyn Error>> {
    if position.0 < maze.len() && position.1 < maze[position.0].len() {
        if let Some(character2) = maze[position.0][position.1].chars().next_back() {
            if let Some(digit) = character2.to_digit(10) {
                let value_as_usize = digit as usize;
                let bomb = if character.starts_with(BOMBA_NORMAL) {
                    Bomb::new((position.0, position.1), TypeBomb::Normal, value_as_usize)
                } else {
                    Bomb::new((position.0, position.1), TypeBomb::Traspaso, value_as_usize)
                };
                bombs.push(bomb);
            }
        }
    }
    Ok(())
}

/// Procesa un carácter como un enemigo y agrega una instancia de `Enemy` al vector `enemies`.
///
/// # Argumentos
///
/// - `character`: Carácter que representa el tipo de enemigo.
/// - `chars`: Referencia mutable a los caracteres restantes en la cadena.
/// - `position`: Referencia mutable a la posición actual en el laberinto.
/// - `enemies`: Referencia mutable al vector de enemigos.
pub fn process_enemy(
    maze: &[Vec<String>], // Cambia el parámetro a la matriz maze
    position: &mut (usize, usize),
    enemies: &mut Vec<Enemy>,
) {
    if let Some(character2) = maze[position.0][position.1].chars().next_back() {
        if let Some(digit) = character2.to_digit(10) {
            let value_as_usize = digit as usize;
            let enemy = Enemy::new((position.0, position.1), value_as_usize);
            enemies.push(enemy);
        }
    }
}

/// Procesa un carácter como un desvío y agrega una instancia de `Detour` al vector `detours`.
///
/// # Argumentos
///
/// - `chars`: Referencia mutable a los caracteres restantes en la cadena.
/// - `position`: Referencia mutable a la posición actual en el laberinto.
/// - `detours`: Referencia mutable al vector de desvíos.
pub fn process_detour(
    maze: &[Vec<String>], // Cambia el parámetro a la matriz maze
    position: &mut (usize, usize),
    detours: &mut Vec<Detour>,
) {
    if let Some(next_char) = maze.get(position.0).and_then(|row| row.get(position.1)) {
        let direction = match next_char.as_str() {
            RIGHT => TypeDetour::Right,
            LEFT => TypeDetour::Left,
            UP => TypeDetour::Up,
            DOWN => TypeDetour::Down,
            _ => TypeDetour::Left, // Definir un valor predeterminado apropiado
        };
        let detour = Detour::new((position.0, position.1), direction);
        detours.push(detour);
    }
}

/// Procesa un carácter como un objeto en el laberinto y realiza las acciones correspondientes.
///
/// # Argumentos
///
/// - `character`: Carácter que representa el objeto.
/// - `chars`: Referencia mutable a los caracteres restantes en la cadena.
/// - `position`: Referencia mutable a la posición actual en el laberinto.
/// - `bombs`: Referencia mutable al vector de bombas.
/// - `enemies`: Referencia mutable al vector de enemigos.
/// - `detours`: Referencia mutable al vector de desvíos.
///
/// # Errores
///
/// Devuelve un error si el carácter representa un objeto inválido en el laberinto.
pub fn process_character(
    character: &str,
    maze: Vec<Vec<String>>,
    position: &mut (usize, usize),
    bombs: &mut Vec<Bomb>,
    enemies: &mut Vec<Enemy>,
    detours: &mut Vec<Detour>,
) -> Result<(), Box<dyn Error>> {
    if character.starts_with(BOMBA_NORMAL) || character.starts_with(BOMBA_TRASPASO) {
        let _ = process_bomb(&maze, character, position, bombs);
        Ok(())
    } else if character.starts_with(ENEMY) {
        process_enemy(&maze, position, enemies);
        Ok(())
    } else if character.starts_with(DETOUR) {
        process_detour(&maze, position, detours);
        Ok(())
    } else if character.starts_with(WALL)
        || character.starts_with(ROCK)
        || character.starts_with(VACIO_)
    {
        Ok(())
    } else {
        Err(Box::new(error_objeto_invalido()))
    }
}

/// Crea una instancia de `GameData` internamente utilizando los vectores de objetos y el laberinto.
///
/// # Argumentos
///
/// - `bombs`: Vector de bombas.
/// - `enemies`: Vector de enemigos.
/// - `detours`: Vector de desvíos.
/// - `maze`: Laberinto representado como una matriz de cadenas.
///
/// # Retorna
///
/// Una instancia de `GameData` que contiene los objetos y el laberinto.
fn create_game_data_internal(
    bombs: Vec<Bomb>,
    enemies: Vec<Enemy>,
    detours: Vec<Detour>,
    maze: Vec<Vec<String>>,
) -> GameData {
    let mut block_map = HashMap::new();
    block_map.insert("Down".to_string(), false);
    block_map.insert("Left".to_string(), false);
    block_map.insert("Right".to_string(), false);
    block_map.insert("Up".to_string(), false);
    let mut interceps_map = HashMap::new();
    interceps_map.insert("Wall".to_string(), false);
    interceps_map.insert("Rock".to_string(), false);
    GameData {
        bombs,
        enemies,
        detours,
        maze,
        actual_direction: TypeDirection::None,
        block_map, 
        interceps_map,
    }
}

/// Crea objetos a partir de la cadena de contenido del archivo.
///
/// # Argumentos
///
/// - `file_contents`: Referencia mutable a la cadena de contenido del archivo.
/// - `coordinate_x`: Coordenada X del objeto.
/// - `coordinate_y`: Coordenada Y del objeto.
/// - `maze`: Laberinto representado como una matriz de cadenas.
///
/// # Errores
///
/// Devuelve un error si se encuentra un objeto inválido en el laberinto.
pub fn create_objects(
    coordinate_x: usize,
    coordinate_y: usize,
    maze: Vec<Vec<String>>,
) -> Result<GameData, Box<dyn Error>> {
    let mut position: (usize, usize) = (0, 0);
    let mut bombs: Vec<Bomb> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();
    let mut detours: Vec<Detour> = Vec::new();

    // Recorre la matriz maze en lugar de los caracteres del archivo
    for (row_index, row) in maze.iter().enumerate() {
        for (col_index, character) in row.iter().enumerate() {
            if character == SALTO_LINEA || character == VACIO_ {
                continue;
            }
            position.0 = row_index;
            position.1 = col_index;
            if let Err(_error) = process_character(
                character,
                maze.clone(),
                &mut position,
                &mut bombs,
                &mut enemies,
                &mut detours,
            ) {
                return Err(Box::new(error_objeto_invalido()));
            }
        }
    }

    let game_data = create_game_data_internal(
        bombs.clone(),
        enemies.clone(),
        detours.clone(),
        maze.clone(),
    );

    if let Err(error) = game_data.validate_maze(coordinate_x, coordinate_y) {
        eprintln!("Error: {}", error);
        return Err(Box::new(error_objeto_invalido()));
    }

    Ok(game_data)
}

/// Realiza las acciones correspondientes a un objeto en el laberinto. Chequea que clase de objeto se encuentran en el alcance de la bomba
///
/// # Argumentos
///
/// - `game_data`: Referencia mutable a los datos del juego.
/// - `object`: Referencia a la cadena que representa el objeto.
/// - `new_x`: Nueva coordenada X después de un desvío.
/// - `y`: Coordenada Y actual.
/// - `typee`: Tipo de bomba (Normal o Traspaso).
/// - `interations_pending`: Iteraciones pendientes para objetos de tipo Traspaso.
/// - `bomb`: Referencia a la bomba asociada al objeto.
pub fn check_objects(
    game_data: &mut GameData,
    object: &String,
    new_x: usize,
    y: usize,
    typee: TypeBomb,
    interations_pending: usize,
    bomb: &Bomb,
) {
    if object.starts_with(DETOUR) {
        GameData::handle_detour(
            game_data,
            object,
            new_x,
            y,
            typee,
            interations_pending,
            bomb,
        )
    }
    if object.starts_with(ENEMY) {
        GameData::handle_enemy(game_data, new_x, y, bomb)
    }
    if object == ROCK_ && typee == TypeBomb::Normal {
        GameData::handle_rock(game_data)
    }
    if object == WALL_ {
        GameData::handle_wall(game_data)
    }
    if object.starts_with(BOMBA_NORMAL) || object.starts_with(BOMBA_TRASPASO) {
        GameData::handle_bomb(game_data, object, new_x, y)
    }
}

/// Detona una bomba en las coordenadas especificadas en el laberinto y aplica sus efectos.
///
/// # Argumentos
///
/// - `game_data`: Referencia mutable a los datos del juego.
/// - `coordinate_x`: Coordenada X de la bomba a detonar.
/// - `coordinate_y`: Coordenada Y de la bomba a detonar.
///
/// # Errores
///
/// Devuelve un error si no se encuentra una bomba en las coordenadas especificadas.
pub fn detonar_bomb(
    game_data: &mut GameData,
    coordinate_x: usize,
    coordinate_y: usize,
) -> Result<(), Box<dyn Error>> {
    if let Some(bomb) = game_data.find_bomb(coordinate_x, coordinate_y) {
        let reach = bomb.reach;
        let tipo_bomb = bomb.typee;
        let copy_bomb = bomb.clone();

        game_data.maze[coordinate_x][coordinate_y] = "_".to_string();
        game_data.remove_bomb(coordinate_x, coordinate_y);
        game_data.apply_bomb_effect(coordinate_x, coordinate_y, reach, tipo_bomb, &copy_bomb);
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::detour::TypeDetour;
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
            vec!["R".to_string(), "_".to_string()],
            vec!["W".to_string(), "DU".to_string()],
        ];
        let mut game_data = GameData::new(
            bombs.clone(),
            enemies.clone(),
            detours.clone(),
            maze.clone(),
            TypeDirection::None,
        );

        assert_eq!(game_data.bombs, bombs);
        assert_eq!(game_data.enemies, enemies);
        assert_eq!(game_data.detours, detours);
        assert_eq!(game_data.maze, maze);
        
        let mut wall = false;
        let mut rock = false;
        if let Some(w) = game_data.interceps_map.get_mut("Wall") {
            wall = *w;
        }        
        if let Some(r) = game_data.interceps_map.get_mut("Rock") {
            rock = *r;
        }
        assert_eq!(wall, false);
        assert_eq!(rock, false);
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
            block_map: [("Down".to_string(), false), ("Up".to_string(), false)].iter().cloned().collect(),
            interceps_map: [("Rock".to_string(), false), ("Wall".to_string(), false)].iter().cloned().collect(),
            actual_direction: TypeDirection::None,
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
            block_map: [("Down".to_string(), false), ("Up".to_string(), false)].iter().cloned().collect(),
            interceps_map: [("Rock".to_string(), false), ("Wall".to_string(), false)].iter().cloned().collect(),
            actual_direction: TypeDirection::None,
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
            TypeDirection::None,
        );

        // Validamos el maze
        let result = game_data.validate_maze(1, 1);

        // Debería ser válido porque hay una bomb y un enemy válidos en las coordenadas especificadas
        assert!(result.is_ok());
    }
}
