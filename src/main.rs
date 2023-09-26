//Structs: Una estructura puede estar conformada de diferentes tipos
//A diferencia de las tuplas, acá se le da un nombre a cada dato
//Agregar estos nombres hace que sea mas flexible que una tupla ya que no depende del orden para acceder a los valores 
mod user;
use crate::user::structure::{User, UserRole};
use crate::user::newuser::{new_user, add_user_data, write_json_user};
use crate::user::admin::{new_admin, admin_acces};
use crate::user::output::show_user;

fn main(){
    let user: (String, String, u16, Option<String>);
    user = add_user_data();

    let user: User = new_user(user.0, user.1, user.2,user.3);
    let admin: bool = admin_acces(&user.role);
    let _result = write_json_user(&user, "./users.json").expect("Error");
    show_user(&user.name, &user.email, &user.age, &user.active, admin,&user.description);
    
    let useradmin: User = new_admin(user.name, user.email, user.age, user.role, user.description);
    let admin: bool = admin_acces(&useradmin.role);
    show_user(&useradmin.name, &useradmin.email, &useradmin.age, &useradmin.active, admin,&useradmin.description);
}