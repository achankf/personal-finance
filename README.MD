# Personal Finance

This is a personal finance tracker designed to be used through the command line. It works alongside CSV files to input data, which is then converted into a Sqlite database and stored at a specified location. Its features include:

- Rebalancing of assets such as credit card debts, emergency funds, and stock/ETF allocations.
- Calculations for adjusted cost base and capital gain/loss.
- Double-entry accounting for every manually entered transaction (which can be quite involved).

As this utility is intended solely for personal use, I have not provided documentation on how to use it. Essentially, this serves as "open core" code for others to examine my programming skills.

## Note to myself

To install the app, create a .env file to specify a DATABASE_URL that is located at the ~/.pit/data.sqlite3, e.g.

```sh
DATABASE_URL=sqlite:///C:/Users/Alfred/.pit/data.sqlite3
```

then run the following command once to set up the initial database:

```sh
cargo install sqlx-cli
sqlx database create
sqlx migrate run
```
