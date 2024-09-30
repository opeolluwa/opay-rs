//! contains the core SDK

use crate::{
    endpoints::{checkout::CheckoutRequest, inquire::InquireRequests, transfer::TransferRequests},
    opay::{Environment, MerchantId, PublicKey},
};

pub struct Opay {
    /// the env in which the application runs see
    pub environment: Environment,
    /// the public key obtainable from dashboard
    pub public_key: PublicKey,
    /// the merchant Id obtainable from dashboard
    pub merchant_id: MerchantId,
    /// the API base URL is as provided by Opay development team
    api_base_url: String,
}

impl Opay {
    pub fn new(environment: Environment, public_key: PublicKey, merchant_id: MerchantId) -> Self {
        Self {
            environment,
            public_key,
            merchant_id,
            api_base_url: Self::base_url(environment),
        }
    }

    fn base_url(environment: Environment) -> String {
        match environment {
            Environment::Development => "dev".to_string(),
            Environment::Production => "prod url".to_string(),
        }
    }
}

impl TransferRequests for Opay {}

impl CheckoutRequest for Opay {}

impl InquireRequests for Opay {}
