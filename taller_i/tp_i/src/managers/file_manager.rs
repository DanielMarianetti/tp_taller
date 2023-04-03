use std::fs::File;
use std::io::{self, BufRead};

/// Helper para esconder el proceso de leido del archivo ya que es una función genérica
/// para leido de archivos por lineas. Devuelve las filas si la apertura del archivo es correcta,
/// caso contrario, devuelve un error propio de la libreria std::fs::File;
pub fn get_rows_from_file(filename: String) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
