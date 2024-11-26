use std::io::stdin;
use template::commands::password;

fn main() {
    let mut password = String::new();

    println!("What would you like your password to be?");
    stdin().read_line(&mut password).unwrap();

    let password = password.trim_end(); // Remove the trailing newline

    let test_hash = match password::hash(password) {
        Ok(result) => result,
        Err(e) => e.to_string(),
    };

    println!("finished test_hash: {:?}", test_hash);

    let test_compare = password::compare(password, &test_hash);
    println!("finished test_compare: {:?}", test_compare);
}
