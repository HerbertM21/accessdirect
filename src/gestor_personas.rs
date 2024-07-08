use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use bincode::{deserialize, serialize};
use crate::errores::GestorError;
use crate::persona::Persona;
use crate::tabla_hash::TablaHash;

pub struct GestorPersonas {
    archivo: File,
    tabla_hash: TablaHash,
}

impl GestorPersonas {
    pub fn new(nombre_archivo: &str, capacidad_hash: usize) -> Result<Self, GestorError> {
        let archivo = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(nombre_archivo)?;
        
        let mut gestor = GestorPersonas {
            archivo,
            tabla_hash: TablaHash::new(capacidad_hash),
        };
        gestor.reconstruir_tabla_hash()?;
        Ok(gestor)
    }

    // Se encarga de ingresar una nueva persona al archivo y a la tabla hash
    pub fn ingreso(&mut self, persona: Persona) -> Result<(), GestorError> {
        // Se mueve el cursor al final del archivo
        self.archivo.seek(SeekFrom::End(0))?;
        // stream_position() devuelve la posición actual del cursor en el archivo
        let posicion: u64 = self.archivo.stream_position()?;
        let bytes: Vec<u8> = serialize(&persona)?;
        self.archivo.write_all(&bytes)?; 
        self.tabla_hash.insertar(persona.email.clone(), posicion);
        Ok(())
    }

    // Se encarga de buscar una persona en el archivo a partir de su email
    pub fn busqueda(&mut self, email: &str) -> Result<Option<Persona>, GestorError> {
        if let Some(posicion) = self.tabla_hash.obtener(email) {
            self.archivo.seek(SeekFrom::Start(posicion))?;
            let mut buffer: Vec<u8> = Vec::new();
            self.archivo.read_to_end(&mut buffer)?;
            let persona: Persona = deserialize(&buffer)?;
            Ok(Some(persona))
        } else {
            Ok(None)
        }
    }

    // Se encarga de modificar una persona en el archivo a partir de su email
    pub fn modificacion(&mut self, email: &str, nueva_persona: Persona) -> Result<bool, GestorError> {
        if let Some(posicion) = self.tabla_hash.obtener(email) {
            self.archivo.seek(SeekFrom::Start(posicion))?;
            let bytes: Vec<u8> = serialize(&nueva_persona)?;
            self.archivo.write_all(&bytes)?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // En el caso de que exista el archivo binario, se reconstruye la tabla hash con los datos del archivo
    fn reconstruir_tabla_hash(&mut self) -> Result<(), GestorError> {
        self.archivo.seek(SeekFrom::Start(0))?;
        let mut posicion = 0;
        loop {
            match bincode::deserialize_from::<_, Persona>(&mut self.archivo) {
                // Ok(persona) indica que se pudo leer una Persona del archivo
                Ok(persona) => {
                    self.tabla_hash.insertar(persona.email.clone(), posicion);
                    posicion = self.archivo.stream_position()?;
                },
                Err(e) => {
                    // Verificar si el error es debido al final del archivo
                    if let bincode::ErrorKind::Io(ref io_error) = *e {
                        if io_error.kind() == std::io::ErrorKind::UnexpectedEof {
                            break; // Llegamos al final del archivo, salimos del loop
                        }
                    }
                    // Si no es un error de EOF, se imprime el error y se retorna
                    return Err(GestorError::Bincode(e));
                },
            }
        }
        Ok(())
    }
}  

// Se usa la estructura Result para manejar los errores como try-except en Python
// Si hay un error, se retorna un Result con el error, de lo contrario se retorna un Result con un valor vacío

// Mientras que el uso de la estructura Option es para manejar los valores nulos o vacíos
// Si hay un valor, se retorna un Some(valor), de lo contrario se retorna un None