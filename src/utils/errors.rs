use burrow_db::define_errors;

define_errors!(
    AuthError {
        UsernameExists => "23505",
        UserNotFound => "P0000",
        AccountNotFound => "P0001",
        SessionNotFound => "P0002",
        InvalidPassword => "NONE"
    }
);
