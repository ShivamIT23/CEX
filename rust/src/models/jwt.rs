use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: u32,
    pub username: String,
    pub exp: usize,
}
