use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct CredentialsBody {
    pub username: String,
    pub password: String,
}
