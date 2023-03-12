SELECT first_name,
    last_name,
    account_name AS "account_name",
    last_payment_date AS "last_payment_date:DateTime<Local>",
    balance AS "balance!:f64",
    has_pad AS "has_pad:bool"
FROM CreditCardView
WHERE "balance!:f64" > 0