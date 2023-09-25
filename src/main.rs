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
 

    loop {
        println!("----------\n * MAIN MENU *\n----------
        [1] Opciones de Usuario\n
        [2] Opciones de administración\n
        [3] Mostrar Usuario\n
        [0] Salir\n
        SELECCIONE UNA OPCIÓN:
        ----------------------");

    

        let mut admin: bool = false;
        let mut userstruct = User { name: "".to_string(), email: "".to_string(), age: 0, active: false, role: UserRole::BASIC, description: None };
            
        let mut input = String::new();
        io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
        let option: u8 = u8::from_str(&input.trim()).unwrap();

        match option {
            1 => {
     
                loop {
                    println!("----------\n * MAIN MENU *\n----------
                        [1] Crear un nuevo usuario\n
                        [2] Opciones avanzadas de Usuario\n
                        [0] Salir\n
                        SELECCIONE UNA OPCIÓN:
                        ----------------------");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
                    let option2: u8 = u8::from_str(&input.trim()).unwrap();
                            
                    match option2{

                        1 => {
                            let mut user: (String,String,u16,Option<String>);
                            user = add_user_data();
                            loop {
                                println!("----------\n * MAIN MENU *\n----------
                                    [1] Guardar Usuario\n
                                    [2] Volver a llenar informacion\n
                                    [0] No guardar y salir\n
                                    SELECCIONE UNA OPCIÓN:
                                    ----------------------");
                                let mut input = String::new();
                                io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
                                let option3: u8 = u8::from_str(&input.trim()).unwrap();
                                        
                                match option3 {
                                    1 => {
                                        userstruct = User::new_user(user.0, user.1, user.2,user.3);
                                        let copyuserstruct: &User = &userstruct;
                                        admin = admin_acces(&copyuserstruct.role);
                                        break;
                                    }
                                    2 => user = add_user_data(),
                                    0 => break,
                                     _ => println!("Ingrese una opcion valida")
                                };
                            
                            }
                        }
                        2 => {
                            loop {
                                println!("----------\n * MAIN MENU *\n----------
                                    [1] Crear un nuevo usuario\n
                                    [2] Opciones de\n
                                    [3] Buscar Usuario\n
                                    [4] Editar Usuario\n
                                    [5] Mostrar Usuario\n
                                    [0] Salir\n
                                    SELECCIONE UNA OPCIÓN:
                                    ---------------------");
                                let mut input = String::new();
                                io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
                                let option4: u8 = u8::from_str(&input.trim()).unwrap();
                                            
                                match option4{
                                    3 => println!("Cooming soon"),
                                    4 => println!("Cooming soon"),
                                    5 => {
                                        show_user(&userstruct.name, &userstruct.email, &userstruct.age, &userstruct.active, admin,&userstruct.description);
                                        break;
                                        }
                                          
                                    0 => break,
                                    _ => println!("Ingrese una opción válida")                                  
                                }
                                        

                            }
                                           
                        }
                        
                        0 => break,
                        _ => println!("Ingrese una opción válida")         
                    }
                               
                };
                                            
            }

            2 => {
                let useradmin: User;
                useradmin = User::new_admin(userstruct.name, userstruct.email, userstruct.age, userstruct.role, userstruct.description);
                show_user(&useradmin.name, &useradmin.email, &useradmin.age, &useradmin.active, admin, &useradmin.description);
                break;
            }

            0 => break,
            _ =>println!("Ingrese una opción válida")
        }
                
    };
            

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
            println!("Permiso de administrador concedido al usuario: {email}");
            return User {
                name,
                email,
                age,
                active: true,
                role: UserRole::ADMIN,
                description
            };
            
        }else if i == 0 {
            println!("Permisos de aministrado no asignados al usuario: {email}");
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
    let boolean: bool;
    match role {
        UserRole::ADMIN => boolean=true,
        UserRole::BASIC => boolean=false
    }
    println!("Administrador: {boolean}");
    return boolean;
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

