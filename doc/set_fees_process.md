# Setting fees process

It will be explained on an example of Indy CLI.

### Prerequisites

* libindy
* Indy CLI
* libsovtoken
* Wallet with Trustee DID
* Created pool in Indy CLI

[Small guide](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/001-cli#commands) to Indy CLI commands: 

### Creating a SET_FEES transaction.

* Open `indy-cli`
* Open wallet: `wallet open <wallet_name> key=<wallet_encryption_key>`
* Load libsovtoken: `load-plugin library=libsovtoken.[so|dll] initializer=sovtoken_init`
* Make a transaction with a command `ledger set-fees-prepare payment_method=sov fees=NYM:1` to set fees to 1 sovatom for NYM

Indy CLI will print the transaction after these steps.

### Putting your signature on a prepared request

* Open `indy-cli`
* Load libsovtoken: `load-plugin library=libsovtoken.[so|dll] initializer=sovtoken_init`
* Open your wallet: `wallet open <name_of_wallet> key=<wallet_encryption_key>`
* Use your did: `did use <your_did>`
* Sign the transaction you created or received: `ledger sign-multi txn=<transaction_json>`

Indy CLI will print the transaction with your signature after these steps.

### Sending the signed transaction to the ledger

* Open `indy-cli`
* Connect to the pool: `pool connect <pool_name>`
* Send transaction: `ledger custom <signed_transaction_json>`

Received ledger reply json will be printed in cli