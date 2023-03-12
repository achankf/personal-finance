SELECT year AS "year!:i64",
    MONTH AS "month!:i64",
    balance AS "balance!:f64",
    extra_cashback_rate AS "extra_cashback_rate!:f64",
    incl_amex_cashback AS "incl_amex_cashback!:f64",
    excl_amex_cashback AS "excl_amex_cashback!:f64",
    extra_cashback AS "extra_cashback!:f64",
    extra_cashback_after_fee AS "extra_cashback_after_fee!:f64",
    missed_opportunities AS "missed_opportunities!:f64"
FROM JustifyAmex;