//! The crate contains type definition and shared module for the SDK
//!
//!

use serde::{Deserialize, Serialize};

/// the environment helps decide if the application is running in the development env or in the production
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Environment {
    Production,
    Development,
}

impl Default for Environment {
    fn default() -> Self {
        Environment::Development
    }
}
impl ToString for Environment {
    fn to_string(&self) -> String {
        match self {
            &Self::Development => "development".to_string(),
            &Self::Production => "production".to_string(),
        }
    }
}

/// create a merchantId
#[derive(Debug, Serialize, Deserialize)]
pub struct MerchantId(String);

impl MerchantId {
    pub fn from(value: &str) -> Self {
        MerchantId(value.to_string())
    }
}

/// create a publicKey
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicKey(String);

impl PublicKey {
    /// crate a new PublicKey
    ///
    /// ### Example
    /// ```rust
    /// use opay::PublicKey
    /// let public_key = PublicKey::from("pub_MYB7i4ojZGQVkyEUoKaDrGmb0kVcDQzhTJe1jFnlqUHIm")
    /// ```
    ///
    pub fn from(value: &str) -> Self {
        PublicKey(value.to_string())
    }

    /// return the the string the public key type wraps around
    pub fn value(&self) -> &String {
        &self.0
    }
}


pub struct Country {}

pub type Countries = Vec<Country>;


pub struct Bank;

pub type BankList = Vec<Bank>;