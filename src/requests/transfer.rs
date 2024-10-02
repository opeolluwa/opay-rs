
use crate::error::OpayClientError;
use crate::opay::Countries;
use  crate::opay::BankList;
pub trait TransferRequests {
    fn get_countries(&self) ->  Result<Countries, OpayClientError>;
    fn get_bank_list(&self)-> Result<BankList, OpayClientError>;
}