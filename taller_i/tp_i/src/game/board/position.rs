///Estructura de datos que contiene la columna y fila en que está posicionada una pieza.
pub struct Position {
    fila: u8,
    col: u8,
}

impl Position {
    ///Inicializa una posición con valores cualquiera que no serán tomados en cuenta
    pub fn inicializar_posicion() -> Self {
        Position { fila: 0, col: 0 }
    }

    ///Devuelve una posición que será guardada por una pieza
    pub fn set_posicion(fila: u8, columna: u8) -> Self {
        Position { fila, col: columna }
    }

    ///Devuelve la fila de la posición
    pub fn obtener_fila(&self) -> u8 {
        self.fila
    }

    ///Devuelve la columna de la posición
    pub fn obtener_columna(&self) -> u8 {
        self.col
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn inicializar_posicion_debe_tener_ceros() {
        let posicion = Position::inicializar_posicion();
        assert_eq!(posicion.obtener_fila(), 0);
        assert_eq!(posicion.obtener_columna(), 0);
    }

    #[test]
    fn obtener_fila_de_una_posicion_setteada_devuelve_el_valor_correcto() {
        let posicion = Position::set_posicion(10, 6);
        assert_eq!(posicion.obtener_fila(), 10);
    }

    #[test]
    fn obtener_columna_de_una_posicion_setteada_devuelve_el_valor_correcto() {
        let posicion = Position::set_posicion(10, 6);
        assert_eq!(posicion.obtener_columna(), 6);
    }
}
