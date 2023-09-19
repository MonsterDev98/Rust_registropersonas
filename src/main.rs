//Structs: Una estructura puede estar conformada de diferentes tipos
//A diferencia de las tuplas, acá se le da un nombre a cada dato
//Agregar estos nombres hace que sea mas flexible que una tupla ya que no depende del orden para acceder a los valores 

use std::io;//librería necesaria para ingresar datos por consola
use std::str::FromStr;// necesaria para convertir de string a entero en el ingreso de datos por teclado

enum UserRole {
    BASIC,
    ADMIN,
}

struct User {
    name: String,
    email: String,
    age: u16,
    active: bool,
    role: UserRole,
}

fn main(){
    let user: (String, String, u16);
    user = add_user_data();

    let mut user: User = User::new_user(user.0, user.1, user.2);
    let admin: bool = admin_acces(user.role); 
    user = show_user(user.name, user.email, user.age, user.active, admin);
    
    let useradmin: User = User::new_admin(user.name, user.email, user.age);
    let admin: bool = admin_acces(useradmin.role);
    show_user(useradmin.name, useradmin.email, useradmin.age, useradmin.active, admin);
    
}

impl User {
    fn new_user(name: String, email:String, age: u16) -> User{
        return User {
            name,
            email,
            age,
            active: true,
            role: UserRole::BASIC,
        };
    }

    fn new_admin(name:String, email:String, age: u16) -> User{
        println!("\n¿DESEA CAMBIAR LOS PERMISOS DE CUENTA?");
        println!("INGRESE 1 PARA ADMINISTRADOR");
        println!("INGRESE 0 PARA CUENTA BÁSICA");
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
        let i: u16 = u16::from_str(&input.trim()).unwrap();
        if i == 1 {
            return User {
                name,
                email,
                age,
                active: true,
                role: UserRole::ADMIN,
            };
        }else if i == 0 {
            return User {
                name,
                email,
                age,
                active: true,
                role: UserRole::BASIC,
            };
        }else {
            println!("Ingrese un valor correcto");
            return User {
                name,
                email,
                age,
                active: true,
                role: UserRole::BASIC,
            };
        }
    
    }
}

fn admin_acces(role: UserRole) -> bool{
    match role {
        UserRole::ADMIN => true,
        UserRole::BASIC => false
    }
}

fn add_user_data()-> (String, String, u16) {
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

fn show_user(name:String, email:String, age:u16, active:bool, admin:bool) -> User {
    println!("\nNombre: {}email: {}edad: {}\nactivo: {}\nPermisos de Administrador: {}",name, email, age, active, admin);
    return User {
        name,
        email,
        age,
        active: true,
        role: UserRole::BASIC,
    };
}

