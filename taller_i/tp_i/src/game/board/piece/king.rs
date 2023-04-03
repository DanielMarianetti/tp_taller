use crate::game::board::piece::{Color, PuedeComer};
use crate::game::board::position;
pub struct King {
    color: Color,
    posicion: position::Position,
}

impl King {
    pub fn new(color: Color, posicion: position::Position) -> Self {
        King { color, posicion }
    }
}

impl PuedeComer for King {
    fn puede_comer(&self, posicion_otra_pieza: &position::Position) -> bool {
        if (self.posicion.obtener_fila() as i32 - posicion_otra_pieza.obtener_fila() as i32).abs()
            <= 1
        {
            if (self.posicion.obtener_columna() as i32
                - posicion_otra_pieza.obtener_columna() as i32)
                .abs()
                <= 1
            {
                return true;
            }
            return false;
        }
        false
    }

    fn obtener_color(&self) -> &Color {
        &self.color
    }

    fn obtener_posicion(&self) -> &position::Position {
        &self.posicion
    }
}
