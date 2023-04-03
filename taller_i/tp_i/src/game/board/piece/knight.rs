use crate::game::board::piece::{move_helper, Color, PuedeComer};
use crate::game::board::position;

pub struct Knight {
    color: Color,
    posicion: position::Position,
}

impl Knight {
    pub fn new(color: Color, posicion: position::Position) -> Self {
        Knight { color, posicion }
    }
}

impl PuedeComer for Knight {
    fn puede_comer(&self, posicion_otra_pieza: &position::Position) -> bool {
        move_helper::caballo_puede_comer_otra_pieza(&self.posicion, posicion_otra_pieza)
    }

    fn obtener_color(&self) -> &Color {
        &self.color
    }

    fn obtener_posicion(&self) -> &position::Position {
        &self.posicion
    }
}
