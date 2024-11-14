use crate::commands::password;
use crate::errors::{ERROR_PASSWORD_FAILED_TO_HASH, ERROR_USER_NOT_CREATED};
use crate::models::{NewUser, User};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde_json::json;

pub fn create_user_db(conn: &mut PgConnection, loginname: &str, password: &str) -> String {
    use crate::schema::users;

    let password_hash = match password::hash(password) {
        Ok(result) => result,
        Err(_) => ERROR_PASSWORD_FAILED_TO_HASH.to_string(),
    };

    let new_user = NewUser {
        loginname,
        passwordhash: &password_hash,
    };

    let enter_new_user = diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
        .optional();

    if enter_new_user.is_ok() {
        let data = json!({
            "status": "success",
            "data": enter_new_user.unwrap()
        });
        data.to_string()
    } else {
        ERROR_USER_NOT_CREATED.to_string()
    }
}
