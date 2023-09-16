//Structs: Una estructura puede estar conformada de diferentes tipos
//A diferencia de las tuplas, acá se le da un nombre a cada dato
//Agregar estos nombres hace que sea mas flexible que una tupla ya que no depende del orden para acceder a los valores 

use std::io;//librería necesaria para ingresar datos por consola
use std::str::FromStr;// necesaria para convertir de string a entero en el ingreso de datos por teclado

enum Sexo{
    M,
    F
}

struct Usuario {
    nombre: String,
    sexo: Sexo,
    email: String,
    edad: u16,
    activo: bool
}

fn main(){
    let user: (String, String, u16);
    user = ingresar_datos();

    let user: Usuario = new_user(user.0, user.1, user.2);
    mostrar_user(user.nombre, user.email, user.edad, user.activo);
}

fn new_user(nombre: String, email:String, edad: u16) -> Usuario{
    return Usuario {
        nombre,
        sexo::M,
        email,
        edad,
        activo: true
    };
    
}

fn ingresar_datos()-> (String, String, u16) {
    let mut tupla: (String, String, u16) = ("".to_string(),"".to_string(),0);
    println!("Ingrese el nombre: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
    tupla.0 = input;
    println!("Ingrese el email: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
    tupla.1 = input;
    println!("Ingrese la edad: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
    let edad: u16 = u16::from_str(&input.trim()).unwrap();
    tupla.2 = edad;
    
    return tupla;
}

fn mostrar_user(nombre:String, email:String, edad:u16, activo:bool){
    print!("\nNombre: {}email: {}edad: {}\nactivo: {}\n",nombre,email,edad,activo);
}