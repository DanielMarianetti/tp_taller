use crate::error_handler;
use std::{
    fs::File,
    io::{BufReader, Lines},
};
mod board;

/// Struct Game contiene un tablero, interpreta las instrucciones de la interfaz de usuario y pide/manda info tablero
pub struct Game {
    tablero: board::Board,
}

///Implementación de las funciones de Game
impl Game {
    /// Inicializa el juego con valores dummy que por flujo de programa sé que no van a ser tomados en cuenta
    pub fn iniciar_juego() -> Self {
        Game {
            tablero: board::Board::inicializar_tablero(),
        }
    }

    /// Al recibir las lineas del tablero leidas del archivo de texto ingresado por el usuario
    /// Si el tablero brindado tiene la configuración adecuada (8x8), invoca a un método interno
    /// intermedio para colocar las piezas en el tablero pasando las filas una por una.
    /// Esta decisión de implementación se da porque considero que el Tablero no tiene por que conocer
    /// el input del usuario mientras que considero que al Juego le compete esta responsabilidad.
    /// En caso que las filas del tablero no sean exactamente 8 (determinado por el máximo del tablero),
    /// devuelve un Error personalizado
    pub fn posicionar_piezas(
        &mut self,
        board_lines: Lines<BufReader<File>>,
    ) -> Result<(), error_handler::Error> {
        let mut fila_actual = 0;
        for fila in board_lines {
            fila_actual += 1;

            let fila_data = if let Ok(fila) = fila {
                fila
            } else {
                return Err(error_handler::Error::new()
                    .con_tipo_error(error_handler::ErrorType::ErrorAlUnwrapDeFila)
                    .con_mensaje("Error al desempaquetar la fila".to_string()));
            };

            Self::colocar_piezas(self, fila_data, fila_actual)?;

            if fila_actual > self.tablero.obtener_maximo() {
                return Err(error_handler::Error::new()
                    .con_tipo_error(error_handler::ErrorType::TableroFueraDeRango)
                    .con_mensaje("El tablero debe tener dimensiones de 8x8".to_string()));
            }
        }
        if fila_actual < self.tablero.obtener_maximo() {
            return Err(error_handler::Error::new()
                .con_tipo_error(error_handler::ErrorType::TableroFueraDeRango)
                .con_mensaje("El tablero debe tener dimensiones de 8x8".to_string()));
        }
        Ok(())
    }

    /// Verifica la cantidad columnas en cada linea, en caso que sean mas o menos de 8, devuelve un Error
    /// Si encuentra un posible caracter que representa una pieza, pide al Tablero que la
    /// posicione pasando la informacion del tipo de pieza, la fila y la columna donde fue
    /// encontrada
    fn colocar_piezas(
        &mut self,
        fila_data: String,
        fila_actual: u8,
    ) -> Result<(), error_handler::Error> {
        let mut llego_al_límite = false;
        for (column_number, identificador_de_pieza) in fila_data.char_indices() {
            if column_number as u8 > ((self.tablero.obtener_maximo() * 2) - 2) {
                return Err(error_handler::Error::new()
                    .con_tipo_error(error_handler::ErrorType::TableroFueraDeRango)
                    .con_mensaje("El tablero debe tener dimensiones de 8x8".to_string()));
            }
            if identificador_de_pieza != ' ' && identificador_de_pieza != '_' {
                self.tablero.posicionar_pieza(
                    identificador_de_pieza,
                    fila_actual,
                    ((column_number / 2) + 1) as u8,
                )?;
            }
            if column_number as u8 == ((self.tablero.obtener_maximo() * 2) - 2) {
                llego_al_límite = true;
            }
        }
        if !llego_al_límite {
            return Err(error_handler::Error::new()
                .con_tipo_error(error_handler::ErrorType::TableroFueraDeRango)
                .con_mensaje("El tablero debe tener dimensiones de 8x8".to_string()));
        }
        Ok(())
    }

    /// Verifica que haya 2 piezas exactamente para empezar la partida; si las hay,
    /// sabiendo que en este punto las piezas SON VÁLIDAS, se juega el turno. Si no las hay,
    /// devuelve un Error personalizado.
    pub fn resultado_juego(&self) -> Result<char, error_handler::Error> {
        if self.tablero.dos_piezas_colocadas() {
            Ok(self.tablero.jugar_turno())
        } else {
            Err(error_handler::Error::new()
                .con_tipo_error(error_handler::ErrorType::PiezasInsuficientes)
                .con_mensaje("Debe haber dos piezas para comenzar el juego".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn una_fila_con_mas_de_ocho_columnas_devuelve_error() {
        let mut game = Game::iniciar_juego();
        let fila_data = "_ _ _ _ _ _ _ _ ".to_string(); // this has 12 characters, more than 14 (8*2-2)
        let result = game.colocar_piezas(fila_data, 0);
        assert!(result.is_err());
    }

    #[test]
    fn una_fila_con_menos_de_ocho_columnas_devuelve_error() {
        let mut game = Game::iniciar_juego();
        let fila_data = "_ _ _ _ _ _ _ ".to_string(); // this has 12 characters, more than 14 (8*2-2)
        let result = game.colocar_piezas(fila_data, 0);
        assert!(result.is_err());
    }
}
