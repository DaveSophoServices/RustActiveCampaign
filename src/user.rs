use serde::Deserialize;
use crate::active_client;

#[derive(Deserialize,Debug)]
pub struct Users {
    pub users: Vec<User>,
    pub meta: active_client::Meta,
}
#[derive(Deserialize,Debug)]
pub struct User {
    username: String,
}

