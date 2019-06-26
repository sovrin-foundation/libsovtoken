# Setting fees process

## How to set fees for an action
 For setting fees for an action we need to make the following steps:
 * Send a SET_FEES txn with appropriate amount for required alias. 
 For example, we have an alias, like "add_new_steward" (and we want to set fees for adding new nym action). For setting fees for this alias, we need to send a SET_FEES transaction with map {"add_new_steward": 42}.
 Txn will be looked as:
 ```
{
    "reqId": <int>,             //random identifier
    "protocolVersion": <int>,   // the version of the client/node communication protocol
    "operation": {
        "type": "20000",
        "fees": {
            "add_new_steward": 42,,
        }
    },
}
```
 * After that, we need to add metadata into default auth constraint for action "add new nym".
 For this example, txn for changing metadata for default auth_rule will be looked as:
 ```
{
    'operation': {
           'type':'120',
           'constraint':{
                    'constraint_id': 'ROLE', 
                    'role': '0',
                    'sig_count': 1, 
                    'need_to_be_owner': False, 
                    'metadata': {'fees': 'add_new_steward'}
           }, 
           'field' :'role',
           'auth_type': '1', 
           'auth_action': 'ADD',
           'new_value': '2'
    },
    
    'identifier': <str: some identifier>,
    'reqId': <int: timestamp>,
    'protocolVersion': 1,
    'signature': <str: signature>
}
```

The pool performs the following validation for the given example:
* doDynamicValidation for "adding new steward" nym (_`from indy-node's side`_);
    * making general role's authorization (_`from indy-node's side`_)
    * making fees specific validation, using metadata field (_`from plugin's side`_)
        * lookup through config state for getting amount of "add_new_steward" alias (_`from plugin's side`_)
        * making can_pay_fees validation (_`from plugin's side`_)
        
### Notes:
* The order of previous steps is very important. First of all SET_FEES is required, then - AUTH_RULE.
* SET_FEES is "updating" transaction, so that it appends new aliases to the existing FEEs map (either adding or overriding aliases). For example, if current fees are {A: 1, B: 2} then after sending SET_FEES transaction with {A: 42, C:3}, the resulted map will look like {A: 42, B: 2, C:3}. 
* Setting fees without adding or changing metadata in corresponding auth_rule doesn't have any effect.
        
## How to setup fees for whole pool
For setting fees for whole we need to make the following steps:
* Define all the actions which we would like to set fees for
* Repeat all the steps from [How to set fees for an action](#how-to-set-fees-for-an-action) for each action

## How to change fees amount for alias
For changing amount of fees for existing alias, you need to send a SET_FEES (as described in [How to set fees for an action](#how-to-set-fees-for-an-action)) transaction with 'fees' value, like:
```
{<str: fees alias to change>: <int: new amount>}
```
As was mentioned before, SET_FEES is "updating" transaction and this request will update whole fees map in state and new amount of fees_alias will be used for validation.
Full SET_FEES request:
 ```
{
    "reqId": <int>,             //random identifier
    "protocolVersion": <int>,   // the version of the client/node communication protocol
    "operation": {
        "type": "20000",
        "fees": {
            <str: fees alias to change>: <int: new amount>,
        }
    },
}
```

## How to set fees for complex Auth Constraints
For example, we have a constraint like:
```
(TRUSTEE, 2) or (STEWARD, 5)
```
It means, that this action requires "2 signatures from 2 different TRUSTEEs" or "5 signatures from 5 different STEWARDs" and we want to set fees for steward's part of this constraint.
For this case, we need to:
* add new alias, 'some_action_by_steward' for example, as described in [How to set fees for an action](#how-to-set-fees-for-an-action)
* set this alias for corresponding auth_rule into steward's part of constraint:
```
(TRUSTEE, 2) or (STEWARD, 5, 'fees': 'some_action_by_steward')
```
After that, the next requests will be ordered:
* with 2 and more TRUSTEE's signatures
* with 5 and more STEWARD's signature and required amount of fees.

Also, trustee's part of constraint can contains 'fees' field with different fees alias, like:
```
(TRUSTEE, 2, 'fees': 'some_action_by_trustee') or (STEWARD, 5, 'fees': 'some_action_by_steward')
``` 
'some_action_by_trustee' should exists before setting it in AUTH_RULE transaction.

## Recommendation for setting fees.
* **If you want to set an alias for `AND` constraint, then make sure, that all of fees aliases will have the same amount.**
In the other words, fees aliases can be different, but amount should be the same.
For example, there is a constraint:
```
(TRUSTEE, 1) and (STEWARD, 3)    - it means, that 1 trustee's and 3 steward's signatures are required
```
And we want to setup fees, like:
```
(TRUSTEE, 1, {'fees': 'trustees_fees'}) and (STEWARD, 3, {'fees': 'steward_fees'})
```
And after all setup actions we try to send a request with fees, related to 'trustees_fees' alias and signatures from 1 TRUSTEE and 3 STEWARD.
In this case, if amount of 'trustees_fees' doesn't equal to amount of 'steward_fees' then RequestRejectedException will be raised (`AND` constraint and the second part will be failed).
* **Either do not set FEEs for NODE txn, or set equal amount of Fees for all fields (except service)**
For now, we can add only 1 fees field per request, but Node txn can contain several actions so that validation process (including fees validation) will be called for each action. 
For example we can change ip address, port and alias in 1 txn, but 1 fees field would be used for each action validation.
If each of this actions cost 5 tokens, then Node request with 15 token will be rejected, because we don't summarize all action's tokens during validation process.
But Node request with 5 tokens will be ordered.

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
* Use your did: `did use <your_did>`
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