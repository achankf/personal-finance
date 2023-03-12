mod check;
mod one_shot;
mod presentation;
mod transaction;
mod upsert;

use sqlx::Connection;

pub use check::{
    AssertTransactionBalance, CheckTransactionStore, DistributionCheckResult,
    DistributionRefSumZeroCheckResult,
};
pub use one_shot::{
    Acb, AssetRebalance, BalanceRecord, CreditCardPadInjection, EmergencyRebalance,
    ExpenseByCategoryResult, NetBalanceRecord, StockTransaction, StockUnit,
    TransactionByAccountKey,
};
pub use upsert::{
    Account, AccountSubtype, AccountType, AssetAllocation, AssetClass, AssetClassName,
    CashbackCard, CashbackCategory, CashbackCategoryName, Currency, Distribution, Exchange,
    FinancialEntry, Institution, Person, PrepaidAccount, Security, Store, StoreCashbackMapping,
    TaxShelterType, TransactionForex, TransactionStore,
};

pub struct Transaction<'c>(sqlx::Transaction<'c, sqlx::Sqlite>);

impl<'c> Transaction<'c> {
    pub async fn new(target: &'c mut db::Db) -> db::SqlResult<Transaction<'c>> {
        Ok(Transaction(target.begin().await?))
    }
}
