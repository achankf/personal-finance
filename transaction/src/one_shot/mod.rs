mod check_cash_inactivity;
mod get_asset_last_update;
mod get_asset_rebalance;
mod get_balance;
mod get_contributions;
mod get_credit_card_pad_injection;
mod get_current_credit_card_balance;
mod get_distribution_ref;
mod get_emergency_rebalance;
mod get_expense_by_category;
mod get_net_asset_balance;
mod get_net_revenue_balance;
mod get_next_transaction_id;
mod get_person_id;
mod get_security_id;
mod get_stock_transaction;
mod get_stock_unit;
mod get_trade_data;
mod get_transaction;
mod get_transaction_by_account_key;
mod justify_amex;
mod sync_pending_transaction;

pub use get_asset_rebalance::AssetRebalance;
pub use get_balance::BalanceRecord;
pub use get_contributions::GetContributionsResult;
pub use get_credit_card_pad_injection::CreditCardPadInjection;
pub use get_emergency_rebalance::EmergencyRebalance;
pub use get_expense_by_category::ExpenseByCategoryResult;
pub use get_stock_transaction::StockTransaction;
pub use get_stock_unit::StockUnit;
pub use get_trade_data::GlBreakDown;
pub use get_trade_data::{AcbEntry, AcbEntryType};
pub use get_transaction::GetTransaction;
pub use get_transaction_by_account_key::TransactionByAccountKey;

#[derive(Clone, serde::Deserialize, Debug)]
pub struct NetBalanceRecord {
    pub first_name: String,
    pub last_name: String,
    pub currency: String,
    pub currency_symbol: String,
    pub balance: f64,
}
