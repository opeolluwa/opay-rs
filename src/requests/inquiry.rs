use std::future::Future;

use crate::error::OpayClientError;
use crate::TransactionResponse;

pub trait InquireRequests {
    fn query_wallet_balance() -> impl Future<Output = Result<TransactionResponse, OpayClientError>>;

    fn validate_bank_account() -> impl Future<Output = Result<TransactionResponse, OpayClientError>>;

    fn validate_opay_user_account(
    ) -> impl Future<Output = Result<TransactionResponse, OpayClientError>>;

    fn validate_opay_merchant_account(
    ) -> impl Future<Output = Result<TransactionResponse, OpayClientError>>;
}
