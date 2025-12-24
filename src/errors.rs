use easy_errors::define_errors;

define_errors!(
    AuthError {
        PasswordHashFailed => {
            code: "A0001",
            status: INTERNAL_SERVER_ERROR,
            message: "Failed to process password"
        },
        UsernameExists => {
            code: "23505",
            status: CONFLICT,
            message: "Account already exists"
        },
        UserNotFound => {
            code: "P0000",
            status: NOT_FOUND,
            message: "User not found"
        },
        AccountNotFound => {
            code: "P0001",
            status: UNAUTHORIZED,
            message: "Account does not exist"
        },
        SessionNotFound => {
            code: "P0002",
            status: UNAUTHORIZED,
            message: "Session expired"
        },
        InvalidPassword => {
            code: "NONE",
            status: UNAUTHORIZED,
            message: "Invalid password"
        }
    }
);
