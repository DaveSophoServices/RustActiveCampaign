use serde::Deserialize;

#[derive(Deserialize,Debug)]
pub struct Users {
    pub users: Vec<User>,
    pub meta: super::Meta,
}
#[derive(Deserialize,Debug)]
pub struct User {
    username: String,
}

