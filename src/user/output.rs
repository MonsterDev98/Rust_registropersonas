

pub fn show_user(name:&String, email:&String, age:&u16, active:&bool, admin:bool, description:&Option<String>){
    println!("\nNombre: {}email: {}edad: {}\nactivo: {}\nPermisos de Administrador: {}",name, email, age, active, admin);
    match description {
        None => println!("No se ingresó ninguna descripción"),
        Some(description) => println!("____Descripcion:____\n{}", description)
    }; 

}