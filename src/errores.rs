use std::error::Error;
use std::fmt;
use std::io;
use bincode;

#[derive(Debug)]
pub enum GestorError {
    Io(io::Error),
    Bincode(bincode::Error),
}

impl fmt::Display for GestorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GestorError::Io(err) => write!(f, "Error de E/S: {}", err),
            GestorError::Bincode(err) => write!(f, "Error de serialización: {}", err),
        }
    }
}

impl Error for GestorError {}

impl From<io::Error> for GestorError {
    fn from(err: io::Error) -> GestorError {
        GestorError::Io(err)
    }
}

impl From<bincode::Error> for GestorError {
    fn from(err: bincode::Error) -> GestorError {
        GestorError::Bincode(err)
    }
}