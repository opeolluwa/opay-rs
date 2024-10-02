//! contains the core SDK

use serde::{Deserialize, Serialize};

use crate::{
    client_requests::{
        checkout::CheckoutRequest, inquiry::InquireRequests, transfer::TransferRequests,
    },
    opay::{Environment, MerchantId, PublicKey},
};


#[derive(Debug, Serialize, Deserialize)]
pub struct OpayClient {
    /// the env in which the application runs see
    pub environment: Environment,
    /// the public key obtainable from dashboard
    pub public_key: PublicKey,
    /// the merchant Id obtainable from dashboard
    pub merchant_id: MerchantId,
    /// the API base URL is as provided by Opay development team
    api_base_url: String,
}

impl OpayClient {
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
            Environment::Development => "http://sandbox-cashierapi.opayweb.com/api/v3".to_string(),
            Environment::Production => "https://cashierapi.opayweb.com/api/v3".to_string(),
        }
    }
}

impl TransferRequests for OpayClient {
    fn get_countries(&self) -> Result<crate::opay::Countries, crate::error::OpayClientError> {
        todo!()
    }

    fn get_bank_list(&self) -> Result<crate::opay::BankList, crate::error::OpayClientError> {
        todo!()
    }
}

impl CheckoutRequest for OpayClient {}

impl InquireRequests for OpayClient {}
