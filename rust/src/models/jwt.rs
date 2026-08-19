use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub id: u32,
    pub username: String,
    pub exp: usize,
}

#[derive(serde::Serialize)]
pub struct SignupReturnType {
    pub token: String,
    pub success: String,
}

#[derive(serde::Serialize)]
pub struct AssetBalanceType {
    pub asset: u32,
    pub success: String,
}
