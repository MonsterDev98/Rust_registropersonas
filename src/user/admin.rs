use std::io;
use std::str::FromStr;

use crate::User;
use crate::UserRole;

pub fn new_admin(name:String, email:String, age: u16, role:UserRole, description:Option<String>) -> User{
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

pub fn admin_acces(role: &UserRole) -> bool{
    match role {
        UserRole::ADMIN => true,
        UserRole::BASIC => false
    }
}
