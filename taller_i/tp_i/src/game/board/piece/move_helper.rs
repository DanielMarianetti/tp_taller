use super::position;

///Helper para modularizar los operaciones al determinar las respuestas de las piezas
/// ante la consulta de si pueden comer

///Responde true si ambas piezas están en la misma fila
pub fn estan_en_misma_fila(
    una_posicion: &position::Position,
    otra_posicion: &position::Position,
) -> bool {
    una_posicion.obtener_fila() == otra_posicion.obtener_fila()
}

///Responde true si ambas piezas están en la misma columna
pub fn estan_en_misma_columna(
    una_posicion: &position::Position,
    otra_posicion: &position::Position,
) -> bool {
    una_posicion.obtener_columna() == otra_posicion.obtener_columna()
}

///Responde true si ambas piezas están en diagonal
pub fn estan_en_diagonal(
    una_posicion: &position::Position,
    otra_posicion: &position::Position,
) -> bool {
    (una_posicion.obtener_fila() as i32 - otra_posicion.obtener_fila() as i32).abs()
        == (una_posicion.obtener_columna() as i32 - otra_posicion.obtener_columna() as i32).abs()
}

///Responde true si la otra piezas está en la posición determinada por fila y columna
pub fn otra_posicion_es(otra_posicion: &position::Position, fila: u8, columna: u8) -> bool {
    otra_posicion.obtener_fila() == fila && otra_posicion.obtener_columna() == columna
}

///función específica para el caballo ya que el cálculo es muy particular y engorroso
pub fn caballo_puede_comer_otra_pieza(
    posicion_caballo: &position::Position,
    otra_posicion: &position::Position,
) -> bool {
    if (posicion_caballo.obtener_fila() as i32 - otra_posicion.obtener_fila() as i32).abs() == 2 {
        (posicion_caballo.obtener_columna() as i32 - otra_posicion.obtener_columna() as i32).abs()
            == 1
    } else if (posicion_caballo.obtener_columna() as i32 - otra_posicion.obtener_columna() as i32)
        .abs()
        == 2
    {
        (posicion_caballo.obtener_fila() as i32 - otra_posicion.obtener_fila() as i32).abs() == 1
    } else {
        false
    }
}
