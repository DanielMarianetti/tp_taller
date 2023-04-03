use crate::game::board::piece::{move_helper, Color, PuedeComer};
use crate::game::board::position;

pub struct Rook {
    color: Color,
    posicion: position::Position,
}

impl Rook {
    pub fn new(color: Color, posicion: position::Position) -> Self {
        Rook { color, posicion }
    }
}

impl PuedeComer for Rook {
    fn puede_comer(&self, posicion_otra_pieza: &position::Position) -> bool {
        move_helper::estan_en_misma_fila(&self.posicion, posicion_otra_pieza)
            || move_helper::estan_en_misma_columna(&self.posicion, posicion_otra_pieza)
    }

    fn obtener_color(&self) -> &Color {
        &self.color
    }

    fn obtener_posicion(&self) -> &position::Position {
        &self.posicion
    }
}
