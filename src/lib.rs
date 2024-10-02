pub mod opay;

pub mod error;
#[path = "sdk.rs"]
pub mod opay_client;

#[path = "requests/mod.rs"]
mod client_requests;

/// contains the trait implemented by the OpayClient
pub mod prelude {
    pub use crate::client_requests::checkout::CheckoutRequest;
    pub use crate::client_requests::inquiry::InquireRequests;
    pub use crate::client_requests::transfer::TransferRequests;
}


pub use serde_value::Value as TransactionResponse;