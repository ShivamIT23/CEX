use serde::{Deserialize, Serialize};

pub struct User {
    pub id: u32,
    pub username: String,
    pub password: String,
    pub usd: u32,
    pub sol: u32,
    pub eth: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TokenType {
    Sol,
    Eth,
    Usd,
}

impl User {
    pub fn get_balance(&self, asset: &TokenType) -> u32 {
        match asset {
            TokenType::Sol => self.sol,
            TokenType::Usd => self.usd,
            TokenType::Eth => self.eth,
        }
    }
}
