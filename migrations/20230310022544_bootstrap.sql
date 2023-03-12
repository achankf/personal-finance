CREATE TABLE Currency (
    currency_id INTEGER NOT NULL PRIMARY KEY,
    currency TEXT NOT NULL UNIQUE CHECK (currency = UPPER(currency)),
    currency_name TEXT NOT NULL UNIQUE,
    currency_symbol TEXT,
    -- This is used when doing aggregation which requirs market price calculation
    market_exchange_rate TEXT NOT NULL DEFAULT 1.0 CHECK (market_exchange_rate >= 0)
) STRICT;

CREATE TABLE Institution (
    institution_id INTEGER NOT NULL PRIMARY KEY,
    institution_name TEXT NOT NULL UNIQUE
) STRICT;

CREATE TABLE Store (
    store_id INTEGER NOT NULL PRIMARY KEY,
    store_key TEXT NOT NULL UNIQUE,
    store_name TEXT NOT NULL UNIQUE
) STRICT;

CREATE TABLE Exchange (
    exchange_id INTEGER NOT NULL PRIMARY KEY,
    exchange_key TEXT UNIQUE NOT NULL CHECK (exchange_key = UPPER(exchange_key)),
    exchange_name TEXT UNIQUE NOT NULL UNIQUE
) STRICT;

CREATE TABLE SECURITY(
    security_id INTEGER NOT NULL PRIMARY KEY,
    exchange_id INTEGER NOT NULL REFERENCES Exchange(exchange_id),
    currency_id INTEGER NOT NULL REFERENCES Currency(currency_id),
    ticker TEXT NOT NULL UNIQUE,
    security_name TEXT NOT NULL UNIQUE,
    price TEXT NOT NULL DEFAULT 0.0 CHECK(price >= 0)
) STRICT;

CREATE INDEX Security_idx_Exchange ON SECURITY(exchange_id);

CREATE TABLE Person (
    person_id INTEGER NOT NULL PRIMARY KEY,
    person_key TEXT NOT NULL UNIQUE CHECK(person_key = UPPER(person_key)),
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL
) STRICT;

CREATE INDEX Person_idx_Name ON Person(first_name, last_name);

CREATE INDEX Person_idx_LastName ON Person(last_name);

CREATE TABLE AssetClassName(
    asset_class_name_id INTEGER NOT NULL PRIMARY KEY,
    asset_class_name TEXT NOT NULL UNIQUE
) STRICT;

CREATE TABLE AssetClass(
    asset_class_id INTEGER NOT NULL PRIMARY KEY,
    person_id INTEGER NOT NULL REFERENCES Person(person_id),
    parent_id INTEGER REFERENCES AssetClass(asset_class_id),
    asset_class_name_id INTEGER NOT NULL REFERENCES AssetClassName(asset_class_name_id),
    weight TEXT NOT NULL CHECK(weight >= 0),
    UNIQUE(person_id, asset_class_name_id)
) STRICT;

CREATE INDEX AssetClass_idx_AssetClassNameId ON AssetClass(asset_class_name_id);

CREATE INDEX AssetClass_idx_ParentId ON AssetClass(parent_id);

CREATE TRIGGER AssetClass_trigger_insert_ExactlyOneParentPerPerson
AFTER
INSERT
    ON AssetClass BEGIN
SELECT
    CASE
        WHEN (
            SELECT
                EXISTS(
                    -- model exist for a person but doesn't have exactly one "root" that represents the portfolio
                    SELECT
                        COUNT(*)
                    FROM
                        AssetClass
                        INNER JOIN Person USING (person_id)
                    WHERE
                        parent_id IS NULL
                    GROUP BY
                        person_id
                    HAVING
                        COUNT(*) <> 1
                )
        ) THEN RAISE(
            FAIL,
            'Each person must have 1 root (i.e. parent is null) in AssetClass that represents their asset allocation model'
        )
    END;

END;

CREATE TABLE AssetAllocation (
    asset_allocation_id INTEGER NOT NULL PRIMARY KEY,
    asset_class_name_id INTEGER NOT NULL REFERENCES AssetClassName(asset_class_name_id),
    security_id INTEGER NOT NULL REFERENCES SECURITY(security_id),
    weight TEXT NOT NULL,
    UNIQUE(asset_class_name_id, security_id)
) STRICT;

CREATE INDEX AssetAllocation_idx_Holding ON AssetAllocation(security_id);

CREATE TABLE AccountKind (
    account_kind_id INTEGER NOT NULL PRIMARY KEY,
    account_kind TEXT NOT NULL UNIQUE CHECK(
        account_kind = UPPER (account_kind)
    )
) STRICT;

INSERT INTO
    AccountKind(account_kind)
VALUES
    ('ASSET'),
    ('EQUITY'),
    ('EXPENSE'),
    ('LIABILITIES'),
    ('REVENUE');

CREATE TABLE AccountType(
    account_type_id INTEGER NOT NULL PRIMARY KEY,
    account_type TEXT NOT NULL UNIQUE CHECK(
        account_type = UPPER (account_type)
    )
) STRICT;

CREATE TABLE AccountSubtype (
    account_subtype_id INTEGER NOT NULL PRIMARY KEY,
    account_subtype TEXT NOT NULL UNIQUE CHECK(
        account_subtype = UPPER (account_subtype)
    ),
    account_kind_id INTEGER NOT NULL REFERENCES AccountKind(account_kind_id)
) STRICT;

CREATE TEMP TABLE AccountSubtypePrepopulated (
    account_subtype TEXT NOT NULL PRIMARY KEY,
    account_kind TEXT NOT NULL,
    UNIQUE(account_subtype, account_kind)
) STRICT;

INSERT INTO
    AccountSubtypePrepopulated (account_subtype, account_kind)
VALUES
    ('CASH', 'ASSET'),
    ('FEES', 'EXPENSE'),
    ('INTEREST', 'REVENUE'),
    ('FEE-REIMBURSEMENT', 'REVENUE'),
    ('DISTRIBUTION', 'REVENUE'),
    ('STOCK', 'ASSET'),
    ('COMMISSION', 'EXPENSE'),
    ('DEBT', 'LIABILITIES'),
    ('DEBT-INTEREST', 'EXPENSE'),
    ('CASHBACK', 'REVENUE'),
    -- principal of interest-earning investments (i.e. GIC)
    ('PRINCIPAL', 'ASSET'),
    ('OPEN-BALANCE', 'EQUITY'),
    -- open balance for fiat currency
    ('OPEN-FIAT', 'EQUITY'),
    ('WITHHOLDING-TAX', 'EXPENSE'),
    ('BONUS', 'REVENUE'),
    ('REFUND', 'REVENUE'),
    -- revenue/expense from currency conversion
    ('FOREX-REVENUE', 'REVENUE'),
    ('FOREX-EXPENSE', 'EXPENSE'),
    ('CAPITAL-GAIN', 'REVENUE'),
    ('CAPITAL-LOSS', 'EXPENSE'),
    -- employment
    ('CPP', 'EXPENSE'),
    ('EI', 'EXPENSE'),
    ('GROUP-BENEFIT', 'EXPENSE'),
    ('SALARY', 'REVENUE'),
    ('INCOME-TAX', 'EXPENSE'),
    ('REIMBURSEMENT', 'REVENUE'),
    ('VACATION-PAY', 'REVENUE'),
    ('TERMINATION-PAY', 'REVENUE');

INSERT INTO
    AccountSubtype (account_subtype, account_kind_id)
SELECT
    account_subtype,
    account_kind_id
FROM
    AccountSubtypePrepopulated
    INNER JOIN AccountKind USING (account_kind);

CREATE INDEX AccountSubtype_idx_AccountKind ON AccountSubtype (account_kind_id);

CREATE TABLE Account (
    account_id INTEGER NOT NULL PRIMARY KEY,
    account_key TEXT NOT NULL UNIQUE,
    account_subtype_id INTEGER NOT NULL REFERENCES AccountSubtype(account_subtype_id),
    account_type_id INTEGER NOT NULL REFERENCES AccountType(account_type_id),
    account_name TEXT NOT NULL
) STRICT;

CREATE INDEX Account_idx_AccountSubtype ON Account(account_subtype_id);

CREATE INDEX Account_idx_AccountType ON Account(account_type_id);

CREATE INDEX Account_idx_Type_Name ON Account(account_type_id, account_subtype_id);

CREATE INDEX Account_idx_AccountKeyType ON Account(account_key, account_type_id);

CREATE TABLE FinancialEntry (
    date INTEGER NOT NULL CHECK (date >= 0),
    transaction_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    account_id INTEGER NOT NULL REFERENCES Account(account_id),
    -- can contain fractional shares
    unit TEXT NOT NULL,
    debit TEXT CHECK(
        debit IS NULL
        OR debit >= 0
    ),
    credit TEXT CHECK(
        credit IS NULL
        OR credit >= 0
    ),
    description TEXT NOT NULL,
    PRIMARY KEY (transaction_id, item_id),
    -- one of debit or credit must be null
    CHECK(
        (
            debit IS NOT NULL
            OR credit IS NOT NULL
        )
        AND NOT (
            debit IS NULL
            AND credit IS NULL
        )
    )
) STRICT;

CREATE INDEX FinancialEntry_idx_TransactionCredit ON FinancialEntry(transaction_id, credit);

CREATE INDEX FinancialEntry_idx_ByDate ON FinancialEntry(account_id, transaction_id, date);

CREATE INDEX FinancialEntry_idx_ExpenseQuery ON FinancialEntry(account_id, transaction_id, credit);

CREATE INDEX FinancialEntry_idx_TransactionAccount ON FinancialEntry(transaction_id, account_id);

CREATE INDEX FinancialEntry_idx_Date ON FinancialEntry(date);

CREATE INDEX FinancialEntry_idx_AccountId ON FinancialEntry(account_id, date);

CREATE TABLE TaxShelterType (
    tax_shelter_type_id INTEGER NOT NULL PRIMARY KEY,
    tax_shelter_type TEXT NOT NULL UNIQUE CHECK(tax_shelter_type = UPPER(tax_shelter_type)),
    tax_shelter_name TEXT NOT NULL
) STRICT;

CREATE TABLE CashAccountProduct (
    account_type_id INTEGER NOT NULL PRIMARY KEY REFERENCES AccountType(account_type_id),
    account_name TEXT NOT NULL UNIQUE,
    institution_id INTEGER NOT NULL REFERENCES Institution(institution_id),
    tax_shelter_type_id INTEGER NOT NULL REFERENCES TaxShelterType(tax_shelter_type_id),
    currency_id INTEGER NOT NULL REFERENCES Currency(currency_id),
    -- how much is need for the bank to waive its monthly fees
    min_balance_waiver TEXT NOT NULL,
    -- how many months before banks charges you for inactivity
    inactive_fee_months INTEGER NOT NULL CHECK (inactive_fee_months >= 0)
) STRICT;

CREATE INDEX CashAccount_idx_InactivityFeeMonths ON CashAccountProduct (inactive_fee_months);

CREATE INDEX CashAccount_idx_TaxShelterSubtype ON CashAccountProduct (tax_shelter_type_id, account_type_id);

CREATE TABLE CashAccountHolder (
    cash_account_holder_id INTEGER NOT NULL PRIMARY KEY,
    person_id INTEGER NOT NULL REFERENCES Person(person_id),
    account_type_id INTEGER NOT NULL REFERENCES CashAccountProduct (account_type_id),
    emergency_target TEXT NOT NULL,
    is_closed INTEGER NOT NULL CHECK(
        is_closed = 0
        OR is_closed = 1
    ),
    UNIQUE (person_id, account_type_id)
) STRICT;

CREATE INDEX CashAccountHolder_idx_AccountType ON CashAccountHolder(account_type_id);

CREATE TABLE CashAccountEntryType (
    account_subtype_id INTEGER NOT NULL PRIMARY KEY REFERENCES AccountSubtype(account_subtype_id)
) STRICT;

INSERT INTO
    CashAccountEntryType (account_subtype_id)
SELECT
    account_subtype_id
FROM
    AccountSubtype
WHERE
    account_subtype IN (
        'CASH',
        'FEES',
        'INTEREST',
        'OPEN-BALANCE',
        'OPEN-FIAT',
        'WITHHOLDING-TAX',
        'BONUS',
        'CASHBACK',
        'REFUND',
        'FOREX-REVENUE',
        'FOREX-EXPENSE',
        'FEE-REIMBURSEMENT'
    );

CREATE TABLE CashAccountEntry (
    cash_account_holder_id INTEGER NOT NULL REFERENCES CashAccountHolder(cash_account_holder_id),
    account_subtype_id INTEGER NOT NULL REFERENCES CashAccountEntryType(account_subtype_id),
    account_id INTEGER NOT NULL UNIQUE REFERENCES Account(account_id),
    PRIMARY KEY (cash_account_holder_id, account_subtype_id),
    UNIQUE (
        account_subtype_id,
        account_id,
        cash_account_holder_id
    )
) STRICT;

CREATE INDEX CashAccountEntry_idx_AccountSubtype ON CashAccountEntry(account_subtype_id);

CREATE TABLE CreditCardProduct (
    account_type_id INTEGER NOT NULL PRIMARY KEY REFERENCES AccountType(account_type_id),
    institution_id INTEGER NOT NULL REFERENCES Institution(institution_id),
    account_name TEXT NOT NULL UNIQUE,
    annual_fee TEXT NOT NULL,
    -- null credit_limit represents prepaid cards
    credit_limit TEXT,
    currency_id INTEGER NOT NULL REFERENCES Currency(currency_id)
) STRICT;

CREATE INDEX CreditCardProduct_idx_InstitutionId ON CreditCardProduct(institution_id);

CREATE TABLE CreditCardEntryType (
    account_subtype_id INTEGER NOT NULL PRIMARY KEY REFERENCES AccountSubtype(account_subtype_id)
) STRICT;

INSERT INTO
    CreditCardEntryType (account_subtype_id)
SELECT
    account_subtype_id
FROM
    AccountSubtype
WHERE
    account_subtype IN (
        'DEBT',
        'FEES',
        'DEBT-INTEREST',
        'CASHBACK',
        'OPEN-BALANCE',
        'BONUS'
    );

CREATE TABLE CreditCardHolder (
    credit_card_holder_id INTEGER NOT NULL PRIMARY KEY,
    person_id INTEGER NOT NULL REFERENCES Person(person_id),
    account_type_id INTEGER NOT NULL REFERENCES CreditCardProduct(account_type_id),
    is_closed INTEGER NOT NULL CHECK(
        is_closed = 0
        OR is_closed = 1
    ),
    UNIQUE (person_id, account_type_id)
) STRICT;

CREATE INDEX CreditCardHolder_idx_AccountType ON CreditCardHolder(account_type_id);

CREATE TABLE CreditCardPadSource (
    credit_card_holder_id INTEGER NOT NULL PRIMARY KEY REFERENCES CreditCardHolder(credit_card_holder_id),
    cash_account_holder_id INTEGER NOT NULL REFERENCES CashAccountHolder(cash_account_holder_id)
) STRICT;

CREATE INDEX CreditCardPadSource_idx_CashAccountHolder ON CreditCardPadSource(cash_account_holder_id);

CREATE TABLE CreditCardEntry (
    credit_card_holder_id INTEGER NOT NULL REFERENCES CreditCardHolder(credit_card_holder_id),
    account_subtype_id INTEGER NOT NULL REFERENCES CreditCardEntryType(account_subtype_id),
    account_id INTEGER NOT NULL UNIQUE REFERENCES Account(account_id),
    PRIMARY KEY (
        credit_card_holder_id,
        account_subtype_id
    ),
    UNIQUE(
        account_subtype_id,
        account_id,
        credit_card_holder_id
    )
) STRICT;

CREATE TABLE CashbackCategoryName (
    cashback_category_name_id INTEGER NOT NULL PRIMARY KEY,
    cashback_category_name TEXT NOT NULL UNIQUE
) STRICT;

CREATE TABLE CashbackCard (
    account_type_id INTEGER NOT NULL PRIMARY KEY REFERENCES AccountType(account_type_id)
) STRICT;

CREATE TABLE CashbackCategory (
    account_type_id INTEGER NOT NULL REFERENCES CashbackCard(account_type_id),
    cashback_category_name_id INTEGER NOT NULL REFERENCES CashbackCategoryName(cashback_category_name_id),
    cashback_rate TEXT NOT NULL,
    PRIMARY KEY (account_type_id, cashback_category_name_id)
) STRICT;

CREATE TABLE StoreCashbackMapping (
    store_id INTEGER NOT NULL REFERENCES Store(store_id),
    account_type_id INTEGER NOT NULL REFERENCES AccountType(account_type_id),
    cashback_category_name_id INTEGER NOT NULL REFERENCES CashbackCategoryName(cashback_category_name_id),
    PRIMARY KEY (store_id, account_type_id)
) STRICT;

CREATE TABLE StockAccountEntryType (
    account_subtype_id INTEGER NOT NULL PRIMARY KEY REFERENCES AccountSubtype(account_subtype_id)
) STRICT;

INSERT INTO
    StockAccountEntryType (account_subtype_id)
SELECT
    account_subtype_id
FROM
    AccountSubtype
WHERE
    account_subtype IN (
        'STOCK',
        'DISTRIBUTION',
        'COMMISSION',
        'OPEN-BALANCE',
        'WITHHOLDING-TAX',
        'FOREX-REVENUE',
        'FOREX-EXPENSE',
        'CAPITAL-GAIN',
        'CAPITAL-LOSS'
    );

CREATE TABLE StockAccount (
    -- a stock account should also be a cash account
    account_type_id INTEGER NOT NULL PRIMARY KEY REFERENCES CashAccountProduct (account_type_id)
) STRICT;

CREATE TABLE StockAccountHolder (
    stock_account_holder_id INTEGER NOT NULL PRIMARY KEY,
    person_id INTEGER NOT NULL REFERENCES Person(person_id),
    account_type_id INTEGER NOT NULL REFERENCES StockAccount(account_type_id),
    security_id INTEGER NOT NULL REFERENCES SECURITY(security_id),
    UNIQUE(person_id, account_type_id, security_id)
) STRICT;

CREATE INDEX StockAccountHolder_idx_AccountType ON StockAccountHolder(account_type_id);

CREATE TABLE StockAccountEntry (
    stock_account_holder_id INTEGER NOT NULL REFERENCES StockAccountHolder(stock_account_holder_id),
    account_subtype_id INTEGER NOT NULL REFERENCES StockAccountEntryType(account_subtype_id),
    account_id INTEGER NOT NULL UNIQUE REFERENCES Account(account_id),
    PRIMARY KEY (stock_account_holder_id, account_subtype_id),
    UNIQUE (
        account_subtype_id,
        account_id,
        stock_account_holder_id
    )
) STRICT;

CREATE TABLE GicAccount (
    -- a GIC account should also be a cash account
    account_type_id INTEGER NOT NULL PRIMARY KEY REFERENCES CashAccountProduct (account_type_id)
) STRICT;

CREATE TABLE GicAccountHolder (
    gic_account_holder_id INTEGER NOT NULL PRIMARY KEY,
    person_id INTEGER NOT NULL REFERENCES Person(person_id),
    account_type_id INTEGER NOT NULL REFERENCES GicAccount(account_type_id),
    issue_date TEXT NOT NULL CHECK(strftime('%s', issue_date) >= 0),
    maturity_date TEXT NOT NULL CHECK(
        strftime('%s', maturity_date) > strftime('%s', issue_date)
    ),
    -- assuming negative interest rate isn't possible (for now)
    apr TEXT NOT NULL,
    UNIQUE (
        person_id,
        account_type_id,
        issue_date,
        maturity_date,
        apr
    )
) STRICT;

CREATE INDEX GicAccountHolder_idx_AccountType ON GicAccountHolder(account_type_id);

CREATE TABLE GicAccountSubtype (
    account_subtype_id INTEGER NOT NULL PRIMARY KEY REFERENCES AccountSubtype(account_subtype_id)
) STRICT;

INSERT INTO
    GicAccountSubtype (account_subtype_id)
SELECT
    account_subtype_id
FROM
    AccountSubtype
WHERE
    account_subtype IN ('PRINCIPAL', 'INTEREST', 'OPEN-BALANCE');

CREATE TABLE GicEntry (
    gic_account_holder_id INTEGER NOT NULL REFERENCES GicAccountHolder(gic_account_holder_id),
    account_subtype_id INTEGER NOT NULL REFERENCES GicAccountSubtype(account_subtype_id),
    account_id INTEGER NOT NULL UNIQUE REFERENCES Account(account_id),
    PRIMARY KEY (gic_account_holder_id, account_subtype_id),
    UNIQUE (
        account_subtype_id,
        account_id,
        gic_account_holder_id
    )
) STRICT;

CREATE TABLE IncomeAccount (
    account_type_id INTEGER NOT NULL PRIMARY KEY REFERENCES AccountType(account_type_id),
    account_name TEXT NOT NULL UNIQUE,
    currency_id INTEGER NOT NULL REFERENCES Currency (currency_id)
) STRICT;

CREATE TABLE IncomeAccountHolder (
    income_account_holder_id INTEGER NOT NULL PRIMARY KEY,
    person_id INTEGER NOT NULL REFERENCES Person(person_id),
    account_type_id INTEGER NOT NULL REFERENCES IncomeAccount(account_type_id),
    UNIQUE (person_id, account_type_id)
) STRICT;

CREATE TABLE IncomeAccountSubtype (
    account_subtype_id INTEGER NOT NULL PRIMARY KEY REFERENCES AccountSubtype(account_subtype_id)
) STRICT;

INSERT INTO
    IncomeAccountSubtype (account_subtype_id)
SELECT
    account_subtype_id
FROM
    AccountSubtype
WHERE
    account_subtype IN (
        'CPP',
        'EI',
        'GROUP-BENEFIT',
        'SALARY',
        'INCOME-TAX',
        'REIMBURSEMENT',
        'VACATION-PAY',
        'TERMINATION-PAY'
    );

CREATE TABLE IncomeAccountMapping (
    income_account_holder_id INTEGER NOT NULL REFERENCES IncomeAccountHolder(income_account_holder_id),
    account_subtype_id INTEGER NOT NULL REFERENCES IncomeAccountSubtype(account_subtype_id),
    account_id INTEGER NOT NULL UNIQUE REFERENCES Account(account_id),
    PRIMARY KEY (
        income_account_holder_id,
        account_subtype_id
    ),
    UNIQUE (
        account_subtype_id,
        account_id,
        income_account_holder_id
    )
) STRICT;

-- mainly for calculating credit card cashback for prepaid credits (like Presto)
CREATE TABLE PrepaidAccount (
    account_type_id INTEGER NOT NULL PRIMARY KEY REFERENCES CashAccountProduct(account_type_id)
) STRICT;

CREATE TABLE TransactionStore (
    transaction_id INTEGER NOT NULL PRIMARY KEY,
    store_id INTEGER NOT NULL REFERENCES Store(store_id),
    UNIQUE (store_id, transaction_id)
) STRICT;

CREATE TABLE TransactionForex (
    transaction_id INTEGER NOT NULL PRIMARY KEY,
    exchange_rate TEXT NOT NULL
) STRICT;

CREATE VIEW AllAccountSubtypes (account_subtype_id) AS
SELECT
    account_subtype_id
FROM
    CashAccountEntryType
UNION
SELECT
    account_subtype_id
FROM
    CreditCardEntryType
UNION
SELECT
    account_subtype_id
FROM
    IncomeAccountSubtype
UNION
SELECT
    account_subtype_id
FROM
    GicAccountSubtype
UNION
SELECT
    account_subtype_id
FROM
    StockAccountEntryType;

CREATE VIEW NonCashbackSubtypes (account_subtype_id) AS
SELECT
    account_subtype_id
FROM
    AccountSubtype
WHERE
    account_subtype IN (
        'INSURANCE-CLAIM',
        'CANADA-BENEFIT',
        'CANADA-TAX-RETURN'
    );

-- tax bookkeeping for ETF distribution (for T3 form)
CREATE TABLE CadDistribution (
    security_id INTEGER NOT NULL REFERENCES SECURITY(security_id),
    ex_dividend_date INTEGER NOT NULL CHECK(ex_dividend_date >= 0),
    record_date INTEGER NOT NULL CHECK(record_date >= ex_dividend_date),
    payment_date INTEGER NOT NULL CHECK(payment_date >= record_date),
    -- summary
    total_cash_distribution TEXT NOT NULL,
    total_non_cash_distribution TEXT NOT NULL,
    -- tax breakdown
    -- box 21
    capital_gain TEXT NOT NULL,
    -- box 49
    eligible_dividend TEXT NOT NULL,
    -- box 23
    non_eligible_dividend TEXT NOT NULL,
    -- box 24
    foreign_business_income TEXT NOT NULL,
    -- box 25
    foreign_non_business_income TEXT NOT NULL,
    -- box 26
    other_income TEXT NOT NULL,
    -- box 42
    return_of_capital TEXT NOT NULL,
    -- box X
    non_reportable_distribution TEXT NOT NULL,
    -- box 30
    capital_gains_eligible_for_deduction TEXT NOT NULL,
    -- box 33
    foreign_business_income_tax_paid TEXT NOT NULL,
    -- box 34
    foreign_non_business_income_tax_paid TEXT NOT NULL,
    PRIMARY KEY (security_id, record_date),
    UNIQUE (security_id, ex_dividend_date),
    UNIQUE (security_id, payment_date)
) STRICT;