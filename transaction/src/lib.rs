mod check;
mod one_shot;
mod transaction;
mod upsert;
mod utils;

use sqlx::Connection;

pub use check::{
    AssertTransactionBalance, CheckTransactionStore, DistributionCheckDistributionError,
    DistributionCheckFinancialEntryError, DistributionCheckResult,
    DistributionRefSumZeroCheckResult, PendingEntry,
};
pub use one_shot::{
    AcbEntry, AcbEntryType, AssetRebalance, BalanceRecord, CreditCardPadInjection,
    EmergencyRebalance, ExpenseByCategoryResult, GetTransaction, GlBreakDown, NetBalanceRecord,
    StockTransaction, StockUnit, TransactionByAccountKey,
};
pub use upsert::{
    Account, AccountSubtype, AccountType, AssetAllocation, AssetClass, AssetClassName,
    CashbackCard, CashbackCategory, CashbackCategoryName, Currency, Distribution, Exchange,
    FinancialEntry, Institution, Person, PrepaidAccount, Security, Store, StoreCashbackMapping,
    TaxShelterType, TransactionStore,
};

pub struct Transaction<'c>(sqlx::Transaction<'c, sqlx::Sqlite>);

impl<'c> Transaction<'c> {
    pub async fn new(target: &'c mut db::Db) -> db::SqlResult<Transaction<'c>> {
        Ok(Transaction(target.begin().await?))
    }
}
