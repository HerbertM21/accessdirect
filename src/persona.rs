use serde::{Serialize, Deserialize};

// Se usa la macro derive para implementar automáticamente los traits Serialize y Deserialize
// Archivo especializado para obtener la estructura Persona 
// con las funcionalidades de serialización y deserialización.
#[derive(Debug, Serialize, Deserialize)]

// Se define la estructura Persona con los datos requeridos
pub struct Persona {
    pub nombres: String,
    pub apellidos: String,
    pub compania: String,
    pub direccion: String,
    pub ciudad: String,
    pub pais: String,
    pub provincia: String,
    pub telefono1: String,
    pub telefono2: String,
    pub email: String,
}

