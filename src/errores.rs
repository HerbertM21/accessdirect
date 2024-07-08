use std::error::Error; // Importa el trait Error para manejar errores
use std::fmt; // Importa el módulo fmt para formatear mensajes de error
use std::io; // Importa el módulo io para manejar errores de E/S
use bincode; // Importa la biblioteca bincode para errores de serialización/deserialización

#[derive(Debug)]
// Define una enumeración GestorError para encapsular diferentes tipos de errores
pub enum GestorError {
    Io(io::Error), // Error de entrada/salida (E/S)
    Bincode(bincode::Error), // Error de serialización Y deserialización
}

// Implementa el trait Display para GestorError para formatear el mensaje de error
impl fmt::Display for GestorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            // Formatea el mensaje para errores de E/S
            GestorError::Io(err) => write!(f, "Error de E/S: {}", err),
            // Formatea el mensaje errores de bincode
            GestorError::Bincode(err) => write!(f, "Error de serialización: {}", err),
        }
    }
}

// Encapsula los demás errores en un error genérico de GestorError
impl Error for GestorError {}

// Implementa la conversión de io::Error a GestorError
impl From<io::Error> for GestorError {
    fn from(err: io::Error) -> GestorError {
        GestorError::Io(err)
    }
}

// Implementa la conversión de bincode::Error a GestorError
impl From<bincode::Error> for GestorError {
    fn from(err: bincode::Error) -> GestorError {
        GestorError::Bincode(err)
    }
}
