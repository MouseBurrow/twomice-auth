use burrow_db::define_errors;

define_errors!(
    AuthError {
        UsernameExists => "23505",
        UserNotFound => "P0000",
        TokenInvalid => "P0001",
        SessionExpired => "P0002",
        InvalidPassword => "NONE"
    }
);
