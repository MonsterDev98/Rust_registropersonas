
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use std::io;
use std::str::FromStr;
use std::fs::File;
use std::io::Write;



use crate::User;
use crate::UserRole;


pub fn new_user(name: String, email:String, age: u16,description:Option<String>) -> User{
    return User {
        name,
        email,
        age,
        active: true,
        role: UserRole::BASIC,
        description
    };
}

pub fn add_user_data()-> (String, String, u16,Option<String>) {

    let mut tupla: (String, String, u16,Option<String>)=("".to_string(),"".to_string(),0,Some("".to_string()));
    println!("Ingrese el nombre: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
    tupla.0 = input.trim().to_string();

    println!("Ingrese el email: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok().expect("Error al leer de teclado");
    tupla.1 = input.trim().to_string();

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
        let description = Some(input.trim().to_string());
        tupla.3 = description;

    }else if i == 0 {

        let description: Option<String> = None;
        tupla.3 = description;

    }

    return tupla;
}

pub fn write_json_user(user:&User, user_file: &str) -> Result<(), Box<dyn std::error::Error>> {

    let user_json = serde_json::to_string(user)?;
    let mut file = File::create(user_file)?;
    file.write_all(user_json.as_bytes())?;

    Ok(())

}

