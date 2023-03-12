use bigdecimal::BigDecimal;

mod get_stock_unit;

pub struct StockUnit {
    pub name: String,
    pub account_name: String,
    pub ticker: String,
    pub total_unit: BigDecimal,
    pub market_value: BigDecimal,
}
