use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct TablaHash {
    tabla: Vec<Option<(String, u64)>>,
    capacidad: usize,
}

// Se implementa la estructura TablaHash
impl TablaHash {
    // Se implementa el constructor, especificando la capacidad de la tabla
    pub fn new(capacidad: usize) -> Self {
        TablaHash {
            tabla: vec![None; capacidad],
            capacidad,
        }
    }

    // Metodo que calcula el índice en la tabla a partir del email
    fn calcular_indice(&self, email: &str) -> usize {
        // Se crea un hash a partir del email
        let mut hasher = DefaultHasher::new();
        email.hash(&mut hasher);
        // Se asegura de que el índice esté dentro de los límites de la tabla
        (hasher.finish() % self.capacidad as u64) as usize
    }

    // Metodo que inserta un email y su posición en la tabla hash
    pub fn insertar(&mut self, email: String, posicion: u64) {
        let mut indice = self.calcular_indice(&email);
        // Mientras que la posición en la tabla no esté vacía, se busca la siguiente posición
        while self.tabla[indice].is_some() {
            indice = (indice + 1) % self.capacidad;
        }
        // Se inserta el email y la posición en la tabla
        self.tabla[indice] = Some((email, posicion));
    }

    // Metodo que obtiene la posición de un email en la tabla hash
    pub fn obtener(&self, email: &str) -> Option<u64> {
        let mut indice = self.calcular_indice(email);
        while let Some((e, pos)) = &self.tabla[indice] {
            if e == email {
                return Some(*pos);
            }
            indice = (indice + 1) % self.capacidad;
        }
        None
    }
}
