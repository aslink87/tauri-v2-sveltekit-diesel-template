use diesel::prelude::*;
use std::io::stdin;
use template::{configuration::establish_connection, models::User};

fn main() {
    use template::schema::users::dsl::*;

    let mut name = String::new();
    println!("Who would you like to search for?");
    stdin().read_line(&mut name).unwrap();
    let name = name.trim_end(); // Remove the trailing newline

    println!("Searching...");
    let connection = &mut establish_connection();

    let result = users
        .filter(loginname.eq(name.to_string()))
        .get_result::<User>(connection);

    println!("Found user {:?}", result)
}
