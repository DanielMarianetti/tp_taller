mod bishop;
mod king;
mod knight;
mod move_helper;
mod pawn;
mod queen;
mod rook;

use super::position;
use crate::error_handler;

///NOTA PARA CORRECTOR: No encontré la forma de hacer que se pueda definir cada pieza en
/// un archivo distinto, lo cual sería más apropiado para el entendimiento y correctitud
/// de la solución. Aprecio si me puede comentar el modo para una potencial reentrega
///Enumerador que define los posibles colores de las piezas y un valor para poner por default
#[derive(Debug, PartialEq)]
pub enum Color {
    Negro,
    Blanco,
    Indefinido,
}

///Implementación del trait PartialEq<Color> para poder comparar dos colores y verificar la igualdad de tipos
impl PartialEq<Color> for &Color {
    fn eq(&self, other: &Color) -> bool {
        *self == other
    }
}

///Trait que implementan todas las piezas. Este trait hace posible el "polimorfismo"
/// del programa, haciendo que el sistema de tipos acepte en tiempo de compilación
/// cualquier estructura que lo implemente.
/// La función puede_comer, decide si una pieza puede comer a otra cuya posición es
/// posicion_otra_pieza.
/// obtener_color devuelve el color de la pieza que luego se usará
/// obtener_posicion devuelve la posición actual de una pieza
pub trait PuedeComer {
    fn puede_comer(&self, posicion_otra_pieza: &position::Position) -> bool;

    fn obtener_color(&self) -> &Color;

    fn obtener_posicion(&self) -> &position::Position;
}

///Interfaz para la creación de una pieza
pub struct PieceBuilder {
    color: Color,
    posicion: position::Position,
}

impl PieceBuilder {
    ///Inicializa el constructor, sirve para llenar los espacios que pide el compilador
    pub fn inicializar_pieza() -> Self {
        PieceBuilder {
            color: Color::Indefinido,
            posicion: position::Position::inicializar_posicion(),
        }
    }

    ///Crea una pieza según el identificador pasado por parámetro, carga las
    /// propiedades de la pieza según la definida dentro de esta misma estructura por
    /// las funciones siguientes
    pub fn crear_pieza_de_identificador(
        self,
        identificador: char,
    ) -> Result<Box<dyn PuedeComer>, error_handler::Error> {
        match identificador {
            'a' | 'A' => Ok(Box::new(bishop::Bishop::new(self.color, self.posicion))),
            'r' | 'R' => Ok(Box::new(king::King::new(self.color, self.posicion))),
            'd' | 'D' => Ok(Box::new(queen::Queen::new(self.color, self.posicion))),
            'c' | 'C' => Ok(Box::new(knight::Knight::new(self.color, self.posicion))),
            't' | 'T' => Ok(Box::new(rook::Rook::new(self.color, self.posicion))),
            'p' | 'P' => Ok(Box::new(pawn::Pawn::new(self.color, self.posicion))),
            _ => Err(error_handler::Error::new()
                .con_tipo_error(error_handler::ErrorType::IdentificadorInvalido)
                .con_mensaje("Uno de los identificadores ingresados no es válido".to_string())),
        }
    }

    ///Guarda temporalmente el color de la pieza que luego será creada
    pub fn definir_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    ///Guarda temporalmente la posición de la pieza que luego será creada
    pub fn definir_posicion(mut self, fila: u8, columna: u8) -> Self {
        self.posicion = position::Position::set_posicion(fila, columna);
        self
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn definir_color_negro_en_builder_debe_guardar_ese_color() {
        let pieza_negra_builder = PieceBuilder::inicializar_pieza();
        assert_eq!(
            pieza_negra_builder.definir_color(Color::Negro).color,
            Color::Negro
        );
    }

    #[test]
    fn definir_color_blanco_en_builder_debe_guardar_ese_color() {
        let pieza_blanca_builder = PieceBuilder::inicializar_pieza();
        assert_eq!(
            pieza_blanca_builder.definir_color(Color::Blanco).color,
            Color::Blanco
        );
    }

    #[test]
    fn definir_posicion_de_fila_en_builder_debe_guardar_esa_fila() {
        let pieza_builder = PieceBuilder::inicializar_pieza();
        assert_eq!(
            pieza_builder
                .definir_posicion(30, 0)
                .posicion
                .obtener_fila(),
            30
        );
    }

    #[test]
    fn definir_posicion_de_columna_en_builder_debe_guardar_esa_fila() {
        let pieza_builder = PieceBuilder::inicializar_pieza();
        assert_eq!(
            pieza_builder
                .definir_posicion(25, 10)
                .posicion
                .obtener_columna(),
            10
        );
    }
}
