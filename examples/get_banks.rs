use opay_rs::{
    error::OpayClientError,
    opay::{Environment, MerchantId, PublicKey},
    opay_client::OpayClient,
    prelude::*,
};

#[tokio::main]
async fn main() -> Result<(), OpayClientError> {
    let env = Environment::Development; // Environment::Default
    let public_key = PublicKey::from(&std::env::var("OPAY_PUBLIC_KEY").ok().unwrap());
    let merchant_id = MerchantId::from(&std::env::var("OPAY_MERCHANT_ID").ok().unwrap());
    let opay_client = OpayClient::new(env, public_key, merchant_id);

    let bank_list = opay_client.get_bank_list().await;

    Ok(())
}
