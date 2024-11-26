use std::io::stdin;

use template::db::users::create_user_db;

fn main() {
    let connection = &mut template::configuration::establish_connection();

    let mut loginname = String::new();
    let mut password = String::new();
    println!("Username: ");
    stdin().read_line(&mut loginname).unwrap();
    println!("Password: ");
    stdin().read_line(&mut password).unwrap();

    let loginname = loginname.trim_end(); // Remove the trailing newline
    let password = password.trim_end(); // Remove the trailing newline

    let user = create_user_db(connection, loginname, password);

    println!("finished creating user: {}, hash: {:?}", loginname, user);
}
