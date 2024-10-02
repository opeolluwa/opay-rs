use std::future::Future;

use crate::error::OpayClientError;
use crate::opay::BankList;
use crate::opay::Countries;
use crate::TransactionResponse;

pub trait TransferRequests {
    fn get_countries(&self) -> impl Future<Output = Result<Countries, OpayClientError>>;

    fn get_bank_list(&self) -> impl Future<Output = Result<BankList, OpayClientError>>;

    fn transfer_to_opay_wallet(
    ) -> impl Future<Output = Result<TransactionResponse, OpayClientError>>;

    fn transfer_to_other_banks(
    ) -> impl Future<Output = Result<TransactionResponse, OpayClientError>>;

    fn query_wallet_transfer_status(
    ) -> impl Future<Output = Result<TransactionResponse, OpayClientError>>;

    fn query_bank_transfer_status(
    ) -> impl Future<Output = Result<TransactionResponse, OpayClientError>>;
}

// impl Future<Output = Result<Vec<Country>, OpayClientError>>
