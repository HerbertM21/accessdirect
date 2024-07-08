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
        let mut hasher = DefaultHasher::new();
        email.hash(&mut hasher);
        (hasher.finish() % self.capacidad as u64) as usize
    }

    // Le pasamos como argumento el email original, ya que solo queremos que se ingrese a la tabla
    // No importa que después se elimine de la memoria, dado que, igualmente llamamos a esta función en
    // gestor_personas.rs, con un clone del email original.
    pub fn insertar(&mut self, email: String, posicion: u64) {
        if self.num_elementos >= self.capacidad / 2 {
            // Se redimensiona la tabla al doble de su tamaño si se llena a más del 50%
            self.redimensionar();
        }

        // la funcion calcular_indice se implementa con valor referencia para más eficiencia, 
        // porque así no tendriamos un valor más dentro de la memoria, y solo se usaría un valor 
        // que luego se eliminará automaticamente de la función despues de terminar la función insertar.
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

// Por eso, se implementa la referenciacion a la tupla dentro de la función redimensionar
// para que no se modifique la tabla original, y se pueda iterar sobre ella sin problemas.
// Dado que se está haciendo un asignación de un valor de la tabla, eso le quita la propiedad a la tabla
// original y se esta modificando dentro de la iteracion y el lenguaje no lo permite.