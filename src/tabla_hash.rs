use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct TablaHash {
    tabla: Vec<Option<(String, u64)>>,
    capacidad: usize,
    num_elementos: usize,
}

impl TablaHash {
    pub fn new(capacidad: usize) -> Self {
        TablaHash {
            tabla: vec![None; capacidad],
            capacidad,
            num_elementos: 0,
        }
    }

    fn calcular_indice(&self, email: &str) -> usize {
        let mut hasher = DefaultHasher::new();
        email.hash(&mut hasher);
        (hasher.finish() % self.capacidad as u64) as usize
    }

    pub fn insertar(&mut self, email: String, posicion: u64) {
        if self.num_elementos >= self.capacidad / 2 {
            self.redimensionar();
        }

        let mut indice = self.calcular_indice(&email);
        while self.tabla[indice].is_some() {
            indice = (indice + 1) % self.capacidad;
        }
        self.tabla[indice] = Some((email, posicion));
        self.num_elementos += 1;
    }

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

    fn redimensionar(&mut self) {
        let nueva_capacidad = self.capacidad * 2;
        let mut nueva_tabla = vec![None; nueva_capacidad];
        
        for entrada in self.tabla.iter().filter_map(|entry| entry.as_ref()) {
            let (email, posicion) = entrada;
            let mut indice = {
                let mut hasher = DefaultHasher::new();
                email.hash(&mut hasher);
                (hasher.finish() % nueva_capacidad as u64) as usize
            };
            
            while nueva_tabla[indice].is_some() {
                indice = (indice + 1) % nueva_capacidad;
            }
            nueva_tabla[indice] = Some((email.clone(), *posicion));
        }
        
        self.tabla = nueva_tabla;
        self.capacidad = nueva_capacidad;
    }
}
