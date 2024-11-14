use crate::schema::users;
use diesel::prelude::*;
use serde::Serialize;

#[derive(Debug, Queryable, Selectable, Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub loginname: String,
    pub passwordhash: String,
}

#[derive(Debug, Serialize)]
pub struct UserPublic {
    pub id: i32,
    pub loginname: String,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub loginname: &'a str,
    pub passwordhash: &'a str,
}
