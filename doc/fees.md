# Setting fees process

### How to set fees for an action

1) Send a SET_FEES txn with appropriate amount for required alias. 
For example, we have an alias, like "add_new_steward" (and we want to set fees for adding new nym action). 
For setting fees for this alias, we need to send a SET_FEES transaction with map {"add_new_steward": 42}.

2) Add metadata into default auth constraint for action "add new nym". 
For this example, constraint for changing metadata for default auth_rule will be looked as:
```
{
   'constraint_id': 'ROLE', 
   'role': '0',
   'sig_count': 1, 
   'need_to_be_owner': False, 
   'metadata': {'fees': 'add_new_steward'}
}
```

#### Notes:
* The order of these steps is very important. First of all SET_FEES is required, then - AUTH_RULE.
* SET_FEES is "updating" transaction, so that it appends new aliases to the existing FEEs map (either adding or overriding aliases). For example, if current fees are {A: 1, B: 2} then after sending SET_FEES transaction with {A: 42, C:3}, the resulted map will look like {A: 42, B: 2, C:3}. 
* Setting fees without adding or changing metadata in corresponding auth_rule doesn't have any effect.

## How to set fees on an example of Indy CLI.

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
* Make a transaction with a command `ledger set-fees-prepare payment_method=sov fees=add_new_steward:42` to set fees to 42 sovatom for add_new_steward.

Indy CLI will print the transaction after these steps. Example output:

```json
{"operation":{"type":"20000","fees":{"add_new_steward":42}},"reqId":3782930813,"protocolVersion":2,"identifier":"V4SGRU86Z58d6TV7PBUe6f"}
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
{"identifier":"V4SGRU86Z58d6TV7PBUe6f","operation":{"fees":{"add_new_steward":42},"type":"20000"},"protocolVersion":2,"reqId":3782930813,"signatures":{"V4SGRU86Z58d6TV7PBUe6f":"DpiKv5n5es9yTkPv1py8mMb6PtL1tWrYdpVS9qp5bJ6GtNPRfNME8ThAbxW7hFbAPfsDzQsBMMEarJ4qDS4CgEF"}}
```

The output should be send to the next trustee to sign it.

### Sending the signed SET_FEES transaction to the ledger

* Open `indy-cli`
* Connect to the pool: `pool connect <pool_name>`
* Send transaction: `ledger custom <signed_transaction_json>`

Received ledger reply json will be printed in cli. Example output:

```json
{"op":"REPLY","result":{"txn":{"type":"20000","metadata":{"from":"V4SGRU86Z58d6TV7PBUe6f","reqId":3782930813,"digest":"94952d32bd83f1b63fed28cb502b704fd225cb02dca3cb02f4ebab94f2168370"},"data":{"fees":{"1":1,"10001":2}},"protocolVersion":2},"reqSignature":{"type":"ED25519","values":[{"value":"CFvstbmLLbWL2dtNxPiDkSR2v4aB7ADX41t3hVk4uvsnjVRXSFSwGs7KXcVdVQU9Qgzpp7moLdfKbjsD2QbwW8q","from":"4kyq92WXWVPKARnou6kWr7"},{"value":"YsNUcj1Hkpjfiykqs4C2nRqr8P8Xet2AZthQWgtjKEFxotYR99zHXQxRTBfzD4BRzUx7eL19HvrGdP495wmcrAb","from":"FT5Rx4RZZrVF1SjXtwcX7g"},{"value":"DpiKv5n5es9yTkPv1py8mMb6PtL1tWrYdpVS9qp5bJ6GtNPRfNME8ThAbxW7hFbAPfsDzQsBMMEarJ4qDS4CgEF","from":"V4SGRU86Z58d6TV7PBUe6f"}]},"auditPath":["FXoJDLDmTtc8x4FuNUZyazMTnHeEdqRMrkqiaUg9BivZ","hya3KgvwSti8uwbMv3h4yog6pu7ufSaM37EQFoikyp5","SmgEKUnFjZhC4FbaGwVfipvQMVyHyDW4BxzLSWYhkY2","Hf3CrReW4qNNGrShjpru6VLkfr5eCQn1YCYtuTePX3BD","3JxvbWb6zv7Vsj152frDKezsMGEwgjxCu6AcbPhZM5rq"],"ver":"1","rootHash":"Fg8uKJozQUAgKhjLvNTdPk3ZbduhjRju9pVjcyvXo8n2","txnMetadata":{"txnTime":1536940234,"seqNo":317}}}
```

### Creating and Sending a AUTH_RULE transaction.

* Open `indy-cli`
* Open wallet: `wallet open <wallet_name> key=<wallet_encryption_key>`
* Connect to the pool: `pool connect <pool_name>`
* Use your did: `did use <your_did>`
* Send a transaction with a command `ledger auth-rule txn_type=NYM action=ADD field=role new_value=2 constraint="{"sig_count":1,"role":"0","metadata":{"fees":"add_new_steward"},"constraint_id":"ROLE","need_to_be_owner":false}"` to add fees metadata for the action.

Indy CLI will print result of this transaction.
 
After these steps adding of a new Steward to the ledger will require paying of the fee set above.
```
ledger nym did=VsKV7grR1BUE29mG2Fm2kX role=2 fees_inputs=txo:sov:fkjZEd8eTBnYJsw7m7twMph3UYD6KZCuNwGWnmmtGVgkXafzy7fgaWrpKnwVbNnxTdHF5T4vsAZPe3BVkk3Pg5dYdnGedFHaFhWW2PsgqGAyTLfh4Vit fees_outputs=(pay:sov:dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q,10)
```