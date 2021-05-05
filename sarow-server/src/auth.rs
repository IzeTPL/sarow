
use serde::Deserialize;
#[derive(Debug, Deserialize)]
pub struct BasicAuth {
    pub username: String,
    pub password: String
}

impl BasicAuth {

}