use crate::{
    commands::password, configuration::establish_connection, db::users::create_user_db,
    errors::ERROR_USER_NOT_FOUND, models::User,
};
use diesel::prelude::*;
use serde_json::{self, json};

#[tauri::command]
pub fn get_user(name: &str) -> String {
    use crate::schema::users::dsl::*;

    let connection = &mut establish_connection();
    // TODO: do not allow hashed password to be returned to frontend
    let user = users.filter(loginname.eq(name)).first::<User>(connection);

    if user.is_ok() {
        let data = json!({
            "status": "success",
            "data": user.unwrap()
        });
        data.to_string()
    } else {
        ERROR_USER_NOT_FOUND.to_string()
    }
}

#[tauri::command]
pub fn create_user(name: &str, password: &str) -> String {
    let connection = &mut establish_connection();

    create_user_db(connection, name.trim_end(), password.trim_end())
}

#[tauri::command]
pub fn verify_user(name: &str, password: &str) -> String {
    use crate::schema::users::dsl::*;
    let connection = &mut establish_connection();

    let result = users.filter(loginname.eq(name)).first::<User>(connection);

    if result.is_ok() {
        let result = result.unwrap();
        password::compare(password, &result.passwordhash)
    } else {
        ERROR_USER_NOT_FOUND.to_string()
    }
}
