use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

// Se define la estructura TablaHash con un vector de tuplas (email, posición)
pub struct TablaHash {
    tabla: Vec<Option<(String, u64)>>,
    capacidad: usize,
    num_elementos: usize,
}

// Se implementa la estructura TablaHash
impl TablaHash {
    // Constructor de la estructura 
    pub fn new(capacidad: usize) -> Self {
        TablaHash {
            tabla: vec![None; capacidad],
            capacidad,
            num_elementos: 0,
        }
    }

    // Se encarga de calcular el índice de la tabla a partir del email con el algoritmo de hashing rust
    fn calcular_indice(&self, email: &str) -> usize {
        let mut hasher = DefaultHasher::new(); // Algoritmo de hashing de Rust
        email.hash(&mut hasher); // Se calcula el hash del email
        (hasher.finish() % self.capacidad as u64) as usize // Se obtiene el índice de la tabla en un rango de 0 a capacidad
    }

    // Se encarga de insertar la tupla (email, posición) en la tabla Hash
    pub fn insertar(&mut self, email: String, posicion: u64) {
        if self.num_elementos >= self.capacidad / 2 {
            // Se redimensiona la tabla al doble de su tamaño si se llena a más del 50%
            self.redimensionar();
        }

        // Se calcula el índice de la tabla a partir del email
        let mut indice: usize = self.calcular_indice(&email);
        let mut i: usize = 1;

        // Se implementa el sondeo cuadrático para evitar las colisiones
        while self.tabla[indice].is_some() {
            indice = (indice + i * i) % self.capacidad;
            i += 1;
        }
        self.tabla[indice] = Some((email, posicion));
        self.num_elementos += 1;
    }

    // Se encarga de obtener la posición de la estructura en el archivo.bin a partir de su email
    pub fn obtener(&self, email: &str) -> Option<u64> {
        let mut indice = self.calcular_indice(email);
        let mut i = 1;
        while let Some((e, pos)) = &self.tabla[indice] {
            if e == email {
                return Some(*pos);
            }
            // Se implementa el sondeo cuadrático para evitar colisiones
            indice = (indice + i * i) % self.capacidad;
            i += 1;
        }
        None
    }

    // Se encarga de redimensionar la tabla al doble de su tamaño para evitar colisiones
    // y mantener la eficiencia en la tabla Hash
    fn redimensionar(&mut self) {
        let nueva_capacidad = self.capacidad * 2;
        let mut nueva_tabla: Vec<Option<(String, u64)>> = vec![None; nueva_capacidad];
        
        // Iteramos sobre la tabla actual y copiamos los elementos a la nueva tabla
        // No se modifica directamente la tabla original porque el lenguaje no permite modificar
        // si se está iterando sobre ella.
        for entrada in self.tabla.iter().filter_map(|entry| entry.as_ref()) {
            let (email, posicion) = entrada; // `email` y `posicion` son referencias
            let mut indice = {
                let mut hasher = DefaultHasher::new();
                email.hash(&mut hasher);
                (hasher.finish() % nueva_capacidad as u64) as usize
            };
            
            let mut i = 1;
            // Se implementa el sondeo cuadrático para evitar colisiones
            while nueva_tabla[indice].is_some() {
                indice = (indice + i * i) % nueva_capacidad;
                i += 1;
            }
            nueva_tabla[indice] = Some((email.clone(), *posicion)); 
        }
        
        self.tabla = nueva_tabla;
        self.capacidad = nueva_capacidad;
    }
}
