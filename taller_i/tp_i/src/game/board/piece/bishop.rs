use crate::game::board::piece::{move_helper, Color, PuedeComer};
use crate::game::board::position;

pub struct Bishop {
    color: Color,
    posicion: position::Position,
}

impl Bishop {
    pub fn new(color: Color, posicion: position::Position) -> Self {
        Bishop { color, posicion }
    }
}

impl PuedeComer for Bishop {
    fn puede_comer(&self, posicion_otra_pieza: &position::Position) -> bool {
        move_helper::estan_en_diagonal(&self.posicion, posicion_otra_pieza)
    }

    fn obtener_color(&self) -> &Color {
        &self.color
    }

    fn obtener_posicion(&self) -> &position::Position {
        &self.posicion
    }
}
