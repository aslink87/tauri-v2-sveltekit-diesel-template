// DB user errors
pub const ERROR_USER_NOT_FOUND: &str = r#"
    {
        "status": "error",
        "data": "user not found"
    }
"#;
pub const ERROR_USER_NOT_CREATED: &str = r#"
    {
        "status": "error",
        "data": "user not created"
    }
"#;

// DB password errors
pub const ERROR_PASSWORD_IS_EMPTY: &str = r#"
    {
        "status": "error",
        "data": "password is empty"
    }
"#;
pub const ERROR_PASSWORD_EXCEEDS_MAX_LENGTH: &str = r#"
    {
        "status": "error",
        "data": "password exceeds max length"
    }
"#;
pub const ERROR_PASSWORD_FAILED_TO_HASH: &str = r#"
    {
        "status": "error",
        "data": "password failed to has"
    }
"#;
pub const ERROR_PASSWORD_FAILED_TO_MATCH: &str = r#"
    {
        "status": "error",
        "data": "password did not match"
    }
"#;
