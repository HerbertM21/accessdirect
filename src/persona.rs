use serde::{Serialize, Deserialize};

// Se usa la macro derive para implementar automáticamente los traits Serialize y Deserialize
// Serializar y deserializar una estructura de datos es convertirla a un formato que se pueda
// almacenar en un archivo o transmitir por la red, y viceversa.
// La serialización convierte la estructura de datos en una secuencia de bytes, y la deserialización
// convierte esa secuencia de bytes de vuelta a la estructura de datos original.
#[derive(Debug, Serialize, Deserialize)]
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

// Para que serializamos la estructura Persona?
// Porque queremos almacenarla en un archivo binario, y para eso necesitamos convertirla a bytes.