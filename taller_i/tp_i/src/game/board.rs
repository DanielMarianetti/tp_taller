use crate::error_handler;
mod piece;
mod position;

///Define las características del Tablero, que contiene piezas (que será resuelto
/// en tiempo de ejecución el tipo), un máximo de tamaño del tablero por si se desearía
/// cambiar el tamaño del mismo y un atributo extra que indica las piezas ya colocadas
/// para facilitar cálculos sin perder performance ni muchos recursos
pub struct Board {
    piezas: Vec<Box<dyn piece::PuedeComer>>,
    max_tablero: u8,
    piezas_setteadas: u8,
}

impl Board {
    ///Inicializa el tablero, las piezas vacias, setteando el máximo de tablero e inicializando las piezas ya colocadas
    pub fn inicializar_tablero() -> Self {
        Board {
            piezas: Vec::new(),
            max_tablero: 8,
            piezas_setteadas: 0,
        }
    }

    ///Devuelve el máximo del tablero para verificación en otras partes del programa
    pub fn obtener_maximo(&self) -> u8 {
        self.max_tablero
    }

    /// Posiciona una pieza en el tablero. Recibe el identificador y posición que ocupará.
    /// Verifica que no haya más piezas de las deseadas -> PiezaOutOfRange
    /// Que el identificador de pieza sea una letra (luego se verifica si es una
    /// letra válida) -> IdentificadorInvalido
    /// Que no se puedan colocar dos piezas del mismo color
    pub fn posicionar_pieza(
        &mut self,
        identificador_de_pieza: char,
        fila_actual: u8,
        columna_actual: u8,
    ) -> Result<(), error_handler::Error> {
        let pieza;
        let pieza_builder = piece::PieceBuilder::inicializar_pieza();
        let color_pieza;
        if self.piezas.len() == 2 {
            return Err(error_handler::Error::new()
                .con_tipo_error(error_handler::ErrorType::PiezaOutOfRange)
                .con_mensaje("El maximo de piezas posibles es de 2".to_string()));
        } else {
            if identificador_de_pieza.is_lowercase() {
                color_pieza = piece::Color::Blanco;
            } else if identificador_de_pieza.is_uppercase() {
                color_pieza = piece::Color::Negro;
            } else {
                return Err(error_handler::Error::new()
                    .con_tipo_error(error_handler::ErrorType::IdentificadorInvalido)
                    .con_mensaje(
                        "Uno de los identificadores ingresados no es válido".to_string(),
                    ));
            }
            if self.piezas_setteadas == 1 && self.piezas[0].obtener_color() == color_pieza {
                return Err(error_handler::Error::new()
                    .con_tipo_error(error_handler::ErrorType::DosPiezasDelMismoColor)
                    .con_mensaje("No puede haber dos piezas del mismo color".to_string()));
            } else {
                pieza = pieza_builder
                    .definir_color(color_pieza)
                    .definir_posicion(fila_actual, columna_actual)
                    .crear_pieza_de_identificador(identificador_de_pieza)?;
                self.piezas.push(pieza);
                self.piezas_setteadas += 1;
            }
        }
        Ok(())
    }

    /// Devuelve un booleano que representa que haya exactamente dos piezas colocadas
    pub fn dos_piezas_colocadas(&self) -> bool {
        self.piezas.len() == 2
    }

    /// Juega el turno y llama a determinar el resultado de la jugada.
    /// Por decisión de diseño, elegí que la responsabilidad de jugar el turno sea
    /// de esta entidad y no de Juego. Si bien tal vez Juego podría ser responsable
    /// de llevar a cabo esta acción, Tablero tiene la información necesaria para
    /// llevarlo a cabo (cosa que Juego no) y a su vez, esta decisión está escondida
    /// ante ojos de un usuario.
    pub fn jugar_turno(&self) -> char {
        let mut negro_come = false;
        let mut blanco_come = false;
        if self.piezas[0].puede_comer(self.piezas[1].obtener_posicion()) {
            if self.piezas[0].obtener_color() == piece::Color::Negro {
                negro_come = true;
            } else {
                blanco_come = true;
            }
        }
        if self.piezas[1].puede_comer(self.piezas[0].obtener_posicion()) {
            if self.piezas[1].obtener_color() == piece::Color::Negro {
                negro_come = true;
            } else {
                blanco_come = true;
            }
        }
        self.determinar_resultado_de_partida(blanco_come, negro_come)
    }

    /// Función interna para devolver el caracter pedido que representa el resultado de la jugada
    fn determinar_resultado_de_partida(&self, blanco_come: bool, negro_come: bool) -> char {
        match (blanco_come, negro_come) {
            (true, true) => 'E',
            (false, false) => 'P',
            (false, true) => 'N',
            (true, false) => 'B',
        }
    }
}
