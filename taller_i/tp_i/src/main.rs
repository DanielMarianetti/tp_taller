mod error_handler;
use error_handler::ErrorType;

use crate::managers::{args_manager, file_manager};
mod managers;

mod game;

/// En caso que se genere un error en cualquier punto del programa, al volver a esta función,
/// cortara abruptamente la ejecución del mismo informando al usuario el motivo.
/// Para finalizar imprime por console el resultado del juego (tal vez sería mas elegante
/// crear una mini función que muestre el resultado, pero es sólo una línea en esta función
/// y no creo que sea importante para el diseño de solución)
fn main() {
    let file_path = args_manager::get_file_path().unwrap_or_else(|err| {
        err.mostrar_error();
        std::process::exit(1);
    });

    let board_rows = file_manager::get_rows_from_file(file_path).unwrap_or_else(|err| {
        error_handler::Error::new()
            .con_tipo_error(ErrorType::FileReadError)
            .con_mensaje(err.to_string())
            .mostrar_error();
        std::process::exit(1);
    });

    let mut juego = game::Game::iniciar_juego();

    juego.posicionar_piezas(board_rows).unwrap_or_else(|err| {
        err.mostrar_error();
        std::process::exit(1);
    });

    let resultado_del_juego: char = juego.resultado_juego().unwrap_or_else(|err| {
        err.mostrar_error();
        std::process::exit(1);
    });

    print!("{}", resultado_del_juego);
}
