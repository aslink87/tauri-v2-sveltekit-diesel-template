// Run AFTER migration 'diesel migration run' from server dir
#[cfg(test)]
mod tests {
    use crate::{configuration::establish_connection, models::User};
    use diesel::prelude::*;

    #[test]
    fn get_user() {
        use crate::schema::users::dsl::*;
        let connection = &mut establish_connection();

        let result = users
            .filter(loginname.eq("john".to_string()))
            .get_result::<User>(connection);

        let name = result.unwrap().loginname;
        assert_eq!(name, "john");
    }
}
