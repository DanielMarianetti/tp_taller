use crate::game::board::piece::{move_helper, Color, PuedeComer};
use crate::game::board::position;

pub struct Pawn {
    color: Color,
    posicion: position::Position,
}

impl Pawn {
    pub fn new(color: Color, posicion: position::Position) -> Self {
        Pawn { color, posicion }
    }
}

impl PuedeComer for Pawn {
    fn puede_comer(&self, posicion_otra_pieza: &position::Position) -> bool {
        if self.color == Color::Blanco {
            if move_helper::otra_posicion_es(
                posicion_otra_pieza,
                self.posicion.obtener_fila() - 1,
                self.posicion.obtener_columna() - 1,
            ) {
                true
            } else {
                move_helper::otra_posicion_es(
                    posicion_otra_pieza,
                    self.posicion.obtener_fila() - 1,
                    self.posicion.obtener_columna() + 1,
                )
            }
        } else if move_helper::otra_posicion_es(
            posicion_otra_pieza,
            self.posicion.obtener_fila() + 1,
            self.posicion.obtener_columna() + 1,
        ) {
            true
        } else {
            move_helper::otra_posicion_es(
                posicion_otra_pieza,
                self.posicion.obtener_fila() + 1,
                self.posicion.obtener_columna() - 1,
            )
        }
    }

    fn obtener_color(&self) -> &Color {
        &self.color
    }

    fn obtener_posicion(&self) -> &position::Position {
        &self.posicion
    }
}
