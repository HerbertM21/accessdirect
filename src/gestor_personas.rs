use std::fs::{File, OpenOptions}; // Se importa el modulo para trabajar con archivos
use std::io::{Read, Seek, SeekFrom, Write}; // Se importa el modulo para manejar las funciones de entrada/salida
use bincode::{deserialize, serialize}; // Se importa la función serialize y deserialize de la biblioteca bincode
use crate::errores::GestorError; 
use crate::persona::Persona;
use crate::tabla_hash::TablaHash;

// Se define la estructura GestorPersonas con un archivo binario y una tabla hash
pub struct GestorPersonas {
    archivo: File,
    tabla_hash: TablaHash,
}

// Implementación de la estructura GestorPersonas
impl GestorPersonas {
    pub fn new(nombre_archivo: &str, capacidad_hash: usize) -> Result<Self, GestorError> {
        // Se abre el archivo binario en modo lectura y escritura
        let archivo = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(nombre_archivo)?;
        
        // Se crea una nueva instancia de GestorPersonas con el archivo y la tabla hash
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
        let posicion: u64 = self.archivo.stream_position()?; // Retorna la posición actual del cursor
        let bytes: Vec<u8> = serialize(&persona)?;
        self.archivo.write_all(&bytes)?; 
        // Insertamos la ultima posicion de la estructura en el archivo
        self.archivo.seek(SeekFrom::End(0))?;
        let posicion_final: u64 = self.archivo.stream_position()?;
        self.tabla_hash.insertar(persona.email.clone(), posicion, posicion_final);
        Ok(())
    }

    // Se encarga de buscar una persona en el archivo a partir de su email
    pub fn busqueda(&mut self, email: &str) -> Result<Option<Persona>, GestorError> {
        if let Some((posicion_inicial, posicion_final)) = self.tabla_hash.obtener(email) {
            self.archivo.seek(SeekFrom::Start(posicion_inicial))?; 
            let mut buffer = vec![0; (posicion_final - posicion_inicial) as usize];
            self.archivo.read_exact(&mut buffer)?;
            let persona: Persona = deserialize(&buffer)?;
            Ok(Some(persona))
        } else {
            Ok(None)
        }
    }

    // Se encarga de modificar una persona en el archivo a partir de su email
    pub fn modificacion(&mut self, email: &str, nueva_persona: Persona) -> Result<bool, GestorError> {
        if let Some((posicion_inicial, posicion_final)) = self.tabla_hash.obtener(email) {
            let nueva_bytes: Vec<u8> = serialize(&nueva_persona)?;
            let longitud_original = posicion_final - posicion_inicial;
            let longitud_nueva = nueva_bytes.len() as u64;
    
            self.archivo.seek(SeekFrom::Start(posicion_inicial))?;
    
            let longitud_escrita = if longitud_nueva <= longitud_original {
                // Si la nueva estructura cabe en el espacio original, la escribimos en su lugar
                self.archivo.write_all(&nueva_bytes)?;
                
                // Si sobra espacio, lo llenamos con bytes nulos
                if longitud_nueva < longitud_original {
                    let padding = vec![0; (longitud_original - longitud_nueva) as usize];
                    self.archivo.write_all(&padding)?;
                }
                longitud_nueva
            } else {
                // Si la nueva estructura es más grande, evitamos escribir mas del espacio original
                self.archivo.write_all(&nueva_bytes[..longitud_original as usize])?;
                longitud_original
            };
    
            // Actualizamos la tabla hash con el nuevo email
            self.tabla_hash.eliminar(email);
            self.tabla_hash.insertar(nueva_persona.email.clone(), posicion_inicial, posicion_inicial + longitud_escrita);
    
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
            let posicion_inicial = posicion;
            // Se deserializa la estructura Persona del archivo
            // Se manejan los resultados de exito y error
            match bincode::deserialize_from::<_, Persona>(&mut self.archivo) {
                // Si se obtiene la estructura Persona, se inserta en la tabla hash
                Ok(persona) => {
                    let posicion_final = self.archivo.stream_position()?;
                    self.tabla_hash.insertar(persona.email.clone(), posicion_inicial, posicion_final);
                    posicion = posicion_final;
                },
                // Si se produce un error, se verifica si es un error de fin de archivo
                Err(e) => {
                    if let bincode::ErrorKind::Io(ref io_error) = *e {
                        if io_error.kind() == std::io::ErrorKind::UnexpectedEof {
                            break;
                        }
                    }
                    return Err(GestorError::Bincode(e));
                },
            }
        }
        Ok(())
    }
}