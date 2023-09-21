//Structs: Una estructura puede estar conformada de diferentes tipos
//A diferencia de las tuplas, acá se le da un nombre a cada dato
//Agregar estos nombres hace que sea mas flexible que una tupla ya que no depende del orden para acceder a los valores 

use std::io;
//use std::path::PrefixComponent;
//librería necesaria para ingresar datos por consola
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
    description: Option<String>
}

fn main(){
    let user: (String, String, u16, Option<String>);
    user = add_user_data();

    let user: User = User::new_user(user.0, user.1, user.2,user.3);
    let admin: bool = admin_acces(&user.role); 
    show_user(&user.name, &user.email, &user.age, &user.active, admin,&user.description);
    
    let useradmin: User = User::new_admin(user.name, user.email, user.age, user.role, user.description);
    let admin: bool = admin_acces(&useradmin.role);
    show_user(&useradmin.name, &useradmin.email, &useradmin.age, &useradmin.active, admin,&useradmin.description);
    
}

impl User {
    fn new_user(name: String, email:String, age: u16,description:Option<String>) -> User{
        return User {
            name,
            email,
            age,
            active: true,
            role: UserRole::BASIC,
            description
        };
    }

    fn new_admin(name:String, email:String, age: u16, role:UserRole, description:Option<String>) -> User{
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
                description
            };
        }else if i == 0 {
            return User {
                name,
                email,
                age,
                active: true,
                role: UserRole::BASIC,
                description
            };
        }else {
            println!("Ingrese un valor correcto");
            return User {
                name,
                email,
                age,
                active: true,
                role,
                description
            };
        }
    
    }
}

fn admin_acces(role: &UserRole) -> bool{
    match role {
        UserRole::ADMIN => true,
        UserRole::BASIC => false
    }
}


fn add_user_data()-> (String, String, u16,Option<String>) {
    let mut tupla: (String, String, u16,Option<String>)=("".to_string(),"".to_string(),0,Some("".to_string()));
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
    println!("Desea ingresar una descripción?");
    println!("ingresar información: 1 \n No ingresar información: 0");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
    let i: u16 = u16::from_str(&input.trim()).unwrap();
    if i == 1 {
        println!("Ingrese una descripción personal: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
        let description = Some(input.to_string());
        tupla.3 = description;
    }else if i == 0 {
        let description: Option<String> = None;
        tupla.3 = description;
    }
    return tupla;
}

fn show_user(name:&String, email:&String, age:&u16, active:&bool, admin:bool, description:&Option<String>){
    println!("\nNombre: {}email: {}edad: {}\nactivo: {}\nPermisos de Administrador: {}",name, email, age, active, admin);
    match description {
        None => println!("No se ingresó ninguna descripción"),
        Some(description) => println!("____Descripcion:____\n{}", description)
    }; 



}

