mod persona; // Importa la estructura Persona
mod tabla_hash; // Importa la estructura TablaHash
mod gestor_personas; // Importa la estructura GestorPersonas
mod errores; // Importa la encapsulación de errores

use errores::GestorError; // Importa la estructura GestorError
use std::io::{self, Write}; // Importa el módulo io y el trait Write
use persona::Persona; // Importa la estructura Persona
use gestor_personas::GestorPersonas; // Importa la estructura GestorPersonas

fn main() -> Result<(), GestorError> {
    let mut gestor = GestorPersonas::new("personas.bin", 500)?;
    menu_principal(&mut gestor)
}   

// Función que implementa la interfaz de usuario del programa.
fn menu_principal(gestor: &mut GestorPersonas) -> Result<(), GestorError> {
    // Se crea un bucle infinito que se rompe cuando el usuario elige la opción 4
    loop {
        println!("\n--- Menú Principal ---");
        println!("1. Ingresar nuevo registro");
        println!("2. Buscar registro por email");
        println!("3. Modificar registro existente");
        println!("4. Salir");
        print!("Seleccione una opción: ");
        io::stdout().flush()?;

        let mut opcion = String::new(); 
        io::stdin().read_line(&mut opcion)?; 

        match opcion.trim() {
            "1" => ingresar_registro(gestor)?,
            "2" => buscar_registro(gestor)?,
            "3" => modificar_registro(gestor)?,
            "4" => {
                println!("Saliendo del programa...");
                break;
            },
            _ => println!("Opción no válida. Por favor, intente de nuevo."),
        }
    }
    Ok(()) // Retorna un resultado vacío si no hubo errores
}

// Función que permite ingresar un nuevo registro
fn ingresar_registro(gestor: &mut GestorPersonas) -> Result<(), GestorError> {
    println!("\n--- Ingresar Nuevo Registro ---");
    let persona = leer_datos_persona()?;
    gestor.ingreso(persona)?;
    println!("Registro ingresado exitosamente.");
    
    Ok(()) // Retorna un resultado vacío si no hubo errores: Result<Valor con exito, Error> 
}

// Función que permite buscar un registro por email
fn buscar_registro(gestor: &mut GestorPersonas) -> Result<(), GestorError> {
    println!("\n--- Buscar Registro por Email ---");
    print!("Ingrese el email a buscar: ");
    io::stdout().flush()?; 
    let mut email = String::new(); // Crea un nuevo string vacío
    io::stdin().read_line(&mut email)?;
    let email = email.trim(); // Elimina los espacios en blanco al inicio y al final

    // Si gestor.busqueda(email) retorna "algo", entonces...
    match gestor.busqueda(email)? {
        // Si se encuentra un registro, se imprime en pantalla
        Some(persona) => println!("Registro encontrado: {:?}", persona), 
        // Si no se encuentra ningún registro, se imprime un mensaje
        None => println!("No se encontró ningún registro con ese email."),
    }

    Ok(()) // Retorna un resultado vacío si no hubo errores: Result<Valor con exito, Error>
}

// Función que permite modificar un registro existente
fn modificar_registro(gestor: &mut GestorPersonas) -> Result<(), GestorError> {
    println!("\n--- Modificar Registro Existente ---");
    print!("Ingrese el email del registro a modificar: ");
    io::stdout().flush()?;
    let mut email = String::new(); // Crea un nuevo string vacío
    io::stdin().read_line(&mut email)?;
    let email = email.trim(); // Elimina los espacios en blanco al inicio y al final

    // si gestor.buscar_registro(email) es "algo", entonces...
    if gestor.busqueda(email)?.is_some() {
        println!("Ingrese los nuevos datos:");
        let nueva_persona = leer_datos_persona()?;
        if gestor.modificacion(email, nueva_persona)? {
            println!("Registro modificado exitosamente.");
        } else {
            println!("Error al modificar el registro.");
        }
    // En caso de retornar None, entonces..
    } else {
        println!("No se encontró ningún registro con ese email.");
    }
    
    Ok(()) // Retorna un resultado vacío si no hubo errores: Result<Valor con exito, Error>
}

// Función que permite leer los datos de una persona desde el stdin
fn leer_datos_persona() -> Result<Persona, GestorError> {
    let mut persona = Persona {
        nombres: String::new(),
        apellidos: String::new(),
        compania: String::new(),
        direccion: String::new(),
        ciudad: String::new(),
        pais: String::new(),
        provincia: String::new(),
        telefono1: String::new(),
        telefono2: String::new(),
        email: String::new(),
    };

    print!("Nombres: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut persona.nombres)?;
    persona.nombres = persona.nombres.trim().to_string();

    print!("Apellidos: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut persona.apellidos)?;
    persona.apellidos = persona.apellidos.trim().to_string();

    print!("Compañía: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut persona.compania)?;
    persona.compania = persona.compania.trim().to_string();

    print!("Dirección: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut persona.direccion)?;
    persona.direccion = persona.direccion.trim().to_string();

    print!("Ciudad: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut persona.ciudad)?;
    persona.ciudad = persona.ciudad.trim().to_string();

    print!("País: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut persona.pais)?;
    persona.pais = persona.pais.trim().to_string();

    print!("Provincia: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut persona.provincia)?;
    persona.provincia = persona.provincia.trim().to_string();

    print!("Teléfono 1: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut persona.telefono1)?;
    persona.telefono1 = persona.telefono1.trim().to_string();

    print!("Teléfono 2: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut persona.telefono2)?;
    persona.telefono2 = persona.telefono2.trim().to_string();

    print!("Email: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut persona.email)?;
    persona.email = persona.email.trim().to_string();

    Ok(persona) // Retorna la persona creada si no hubo errores
}