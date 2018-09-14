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
* Make a transaction with a command `ledger set-fees-prepare payment_method=sov fees=NYM:1,ATTRIB:2` to set fees to 1 sovatom for NYM and 2 sovatoms for ATTRIB

Indy CLI will print the transaction after these steps. Example output:

```json
{"operation":{"type":"20000","fees":{"10001":2,"1":1}},"reqId":3782930813,"protocolVersion":2,"identifier":"V4SGRU86Z58d6TV7PBUe6f"}
```

### Putting your signature on a prepared request

This step should be made by multiple trustees.

* Open `indy-cli`
* Load libsovtoken: `load-plugin library=libsovtoken.[so|dll] initializer=sovtoken_init`
* Open your wallet: `wallet open <name_of_wallet> key=<wallet_encryption_key>`
* Use your did: `did use <your_did>`
* Sign the transaction you created or received: `ledger sign-multi txn=<transaction_json>`

Indy CLI will print the transaction with your signature after these steps. Example output:

```json
{"identifier":"V4SGRU86Z58d6TV7PBUe6f","operation":{"fees":{"1":1,"10001":2},"type":"20000"},"protocolVersion":2,"reqId":3782930813,"signatures":{"V4SGRU86Z58d6TV7PBUe6f":"DpiKv5n5es9yTkPv1py8mMb6PtL1tWrYdpVS9qp5bJ6GtNPRfNME8ThAbxW7hFbAPfsDzQsBMMEarJ4qDS4CgEF"}}
```

The output should be send to the next trustee to sign it.

### Sending the signed transaction to the ledger

* Open `indy-cli`
* Connect to the pool: `pool connect <pool_name>`
* Send transaction: `ledger custom <signed_transaction_json>`

Received ledger reply json will be printed in cli. Example output:

```json
{"op":"REPLY","result":{"txn":{"type":"20000","metadata":{"from":"V4SGRU86Z58d6TV7PBUe6f","reqId":3782930813,"digest":"94952d32bd83f1b63fed28cb502b704fd225cb02dca3cb02f4ebab94f2168370"},"data":{"fees":{"1":1,"10001":2}},"protocolVersion":2},"reqSignature":{"type":"ED25519","values":[{"value":"CFvstbmLLbWL2dtNxPiDkSR2v4aB7ADX41t3hVk4uvsnjVRXSFSwGs7KXcVdVQU9Qgzpp7moLdfKbjsD2QbwW8q","from":"4kyq92WXWVPKARnou6kWr7"},{"value":"YsNUcj1Hkpjfiykqs4C2nRqr8P8Xet2AZthQWgtjKEFxotYR99zHXQxRTBfzD4BRzUx7eL19HvrGdP495wmcrAb","from":"FT5Rx4RZZrVF1SjXtwcX7g"},{"value":"DpiKv5n5es9yTkPv1py8mMb6PtL1tWrYdpVS9qp5bJ6GtNPRfNME8ThAbxW7hFbAPfsDzQsBMMEarJ4qDS4CgEF","from":"V4SGRU86Z58d6TV7PBUe6f"}]},"auditPath":["FXoJDLDmTtc8x4FuNUZyazMTnHeEdqRMrkqiaUg9BivZ","hya3KgvwSti8uwbMv3h4yog6pu7ufSaM37EQFoikyp5","SmgEKUnFjZhC4FbaGwVfipvQMVyHyDW4BxzLSWYhkY2","Hf3CrReW4qNNGrShjpru6VLkfr5eCQn1YCYtuTePX3BD","3JxvbWb6zv7Vsj152frDKezsMGEwgjxCu6AcbPhZM5rq"],"ver":"1","rootHash":"Fg8uKJozQUAgKhjLvNTdPk3ZbduhjRju9pVjcyvXo8n2","txnMetadata":{"txnTime":1536940234,"seqNo":317}}}
```