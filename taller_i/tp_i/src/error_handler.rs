///Define los tipos de errores que se pueden encontrar en el programa
/// NOTA: NotHappened es simplemente para inicializar el error
pub enum ErrorType {
    NotHappened,
    FileReadError,
    ArgsError,
    TableroFueraDeRango,
    PiezaOutOfRange,
    PiezasInsuficientes,
    IdentificadorInvalido,
    ErrorAlUnwrapDeFila,
    DosPiezasDelMismoColor,
}

/// Por la forma de implementación, los atributos happened y error_type no tienen mayor
/// influencia en lo que ocurre en el programa, pero podría llegar a implementar alguna
/// forma de mostrar el error agregando/implementando algún Trait del std.
/// La idea de creación de este error es el poder tratar cualquier error en momento de ejecucióm
/// y poder advertir al usuario del problema, en este caso se cortará la ejecución del programa
/// ante la ocurrencia de cualquier error
pub struct Error {
    happened: bool,
    error_type: ErrorType,
    mensaje: String,
}

impl Error {
    ///Inicializa el error con valores placeholders que no serán tomados en cuenta
    pub fn new() -> Self {
        Error {
            happened: false,
            error_type: ErrorType::NotHappened,
            mensaje: String::new(),
        }
    }

    ///Define el tipo de error que ocurre en el objeto Error
    pub fn con_tipo_error(mut self, error_type: ErrorType) -> Self {
        self.happened = true;
        self.error_type = error_type;
        self
    }

    ///Define el mensaje de error que ocurre en el objeto Error y luego será mostrado por pantalla
    pub fn con_mensaje(mut self, mensaje: String) -> Self {
        self.happened = true;
        self.mensaje = mensaje;
        self
    }

    ///Muestra el error y advierte al usuario que finalizará la ejecución del programa
    pub fn mostrar_error(self) {
        println!("Error: {}", self.mensaje);
        println!("Ending program execution.");
    }
}
