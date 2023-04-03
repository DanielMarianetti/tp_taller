use std::env;

use crate::error_handler;

///Esconde el procedimiento de leida de argumentos por linea de comandos, devuelve el valor del mismo.
/// Verifica que haya un y solo un argumento, caso contrario devuelve un error personalizado
pub fn get_file_path() -> Result<String, error_handler::Error> {
    let args: Vec<String> = env::args().collect();
    match args.len() {
    2 => Ok(args[1].clone()),
    n if n < 2 => Err(error_handler::Error::new()
        .con_tipo_error(error_handler::ErrorType::ArgsError)
        .con_mensaje("Not enough arguments, ONE is needed, your file name (make sure it is no the root directory)".to_string())),
    _ => Err(error_handler::Error::new()
        .con_tipo_error(error_handler::ErrorType::ArgsError)
        .con_mensaje("Too many arguments, ONE is needed, your file name (make sure it is no the root directory)".to_string())),
    }
}
