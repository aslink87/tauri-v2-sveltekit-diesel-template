use diesel::prelude::*;
use std::io::stdin;
use template::commands::password;
use template::{configuration::establish_connection, models::User};

fn main() {
    use template::schema::users::dsl::*;

    let mut name = String::new();
    println!("Who are you?");
    stdin().read_line(&mut name).unwrap();

    let name = name.trim_end(); // Remove the trailing newline
                                //
    println!("Searching...");
    let connection = &mut establish_connection();

    let result = users.filter(loginname.eq(name)).first::<User>(connection);

    println!("What is your password?");
    let mut password = String::new();

    stdin().read_line(&mut password).unwrap();

    let password = password.trim_end(); // Remove the trailing newline

    if result.is_ok() {
        let result = result.unwrap();
        let password_compare = password::compare(password, &result.passwordhash);
        println!("finished test_compare: {:?}", password_compare);
    } else {
        println!("User not found");
    }
}
