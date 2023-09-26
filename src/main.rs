//Structs: Una estructura puede estar conformada de diferentes tipos
//A diferencia de las tuplas, acá se le da un nombre a cada dato
//Agregar estos nombres hace que sea mas flexible que una tupla ya que no depende del orden para acceder a los valores 

use std::io;//librería necesaria para ingresar datos por consola
use std::str::FromStr;// necesaria para convertir de string a entero en el ingreso de datos por teclado

enum UserRole { // Uso de enum tipo de dato personalisado UserRole puede ser o Basic o Admin nunca las dos al mismo tiempo
    BASIC,
    ADMIN,
}

struct User { // estructura donde definimos un usuarios y los datos con su respectivo tipo de dato
    name: String,
    email: String,
    age: u16,
    active: bool,
    role: UserRole, // Tipo de dato UserRole creado en la línea 8
}

fn main(){
    let user: (String, String, u16);
    user = add_user_data();

    let user: User = User::new_user(user.0, user.1, user.2);
    let admin: bool = admin_acces(user.role); 
    show_user(&user.name, &user.email, &user.age, &user.active, &admin);
    
    let useradmin: User = User::new_admin(user.name, user.email, user.age);
    let admin: bool = admin_acces(useradmin.role);
    show_user(&useradmin.name, &useradmin.email, &useradmin.age, &useradmin.active, &admin);
    
}

impl User {// Implementacion de la estructura User definida en la linea 13
    // fn que recibe datos necesarios para la creación de un usuario
    fn new_user(name: String, email:String, age: u16) -> User{ 
        return User {
            name,
            email,
            age,
            active: true,// Cuando se crea un usuario por defecto se le da true en activo
            role: UserRole::BASIC,// Cuando se crea usuario por defecto se le da rol Basic
        };
    }

    //Función para crear un nuevo administrador
    fn new_admin(name:String, email:String, age: u16) -> User{
        println!("\n¿DESEA CAMBIAR LOS PERMISOS DE CUENTA?");
        println!("INGRESE 1 PARA ADMINISTRADOR");
        println!("INGRESE 0 PARA CUENTA BÁSICA");

        //Con estas lineas podemos capturar datos de teclado
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
        let i: u16 = u16::from_str(&input.trim()).unwrap();// con esta pasamos el dato capturado en string a un u16
        
        //Condicionales que dependiendo de la bandera i, asigna un Role Admin o Basic
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

//Nos ayuda a saber si un usuario es admin o basic retorna un bool false si es basic y true si es admin
fn admin_acces(role: UserRole) -> bool{
    match role {
        UserRole::ADMIN => true,
        UserRole::BASIC => false
    }
}


//funcion para recibir datos por teclado y almacenarlos en una tupla para posteriormente crear el usuario en newuser() linea 37
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

//funcion que muestra en pantalla un usuario 
fn show_user(name:&String, email:&String, age:&u16, active:&bool, admin:&bool) {
    println!("\nNombre: {}email: {}edad: {}\nactivo: {}\nPermisos de Administrador: {}",name, email, age, active, admin);
}

