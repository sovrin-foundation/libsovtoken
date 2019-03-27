# Mint tokens process

It will be explained on an example of Indy CLI.

### Prerequisites

* libindy
* Indy CLI
* libsovtoken
* Wallet with Trustee DID
* Created pool in Indy CLI

[Small guide](https://github.com/hyperledger/indy-sdk/tree/master/doc/design/001-cli#commands) to Indy CLI commands: 

### Creating a MINT transaction.

##### Preparation

* Open `indy-cli`
* Open wallet: `wallet open <wallet_name> key=<wallet_encryption_key>`
* Load libsovtoken: `load-plugin library=libsovtoken.[so|dll] initializer=sovtoken_init`

##### Action

Make a transaction with a command 
```
mint-prepare outputs=(<recipient payment addres 1>,<amount>),..,(<recipient payment addres-n>,<amount>)` 
```
to mint sovatoms on payment addresses.

Example input: 
```
ledger mint-prepare outputs=(pay:sov:2749JKC5gFV5fAa89iwwJVRJzTYDR8XsJk7ERKkKFG9boRea6j,100)
```
This command will create transaction for minting 100 sovatoms on `pay:sov:2749JKC5gFV5fAa89iwwJVRJzTYDR8XsJk7ERKkKFG9boRea6j` payment address.

Indy CLI will print the transaction after these steps. 

Example output:

```json
{"operation":{"type":"10000","outputs":[{"address":"2749JKC5gFV5fAa89iwwJVRJzTYDR8XsJk7ERKkKFG9boRea6j","amount":100}]},"reqId":3866864878,"protocolVersion":2,"identifier":"V4SGRU86Z58d6TV7PBUe6f"}
```

### Putting your signature on a prepared request

This step should be made by multiple trustees.

##### Preparation
* Open `indy-cli`
* Open wallet: `wallet open <wallet_name> key=<wallet_encryption_key>`
* Use your did: `did use <did>`

#### Action

Sign transaction you create or received with a command 
```
ledger sign-multi txn=<transaction_json>` 
```

Example input: 
```
ledger sign-multi txn={"operation":{"type":"10000","outputs":[{"address":"2749JKC5gFV5fAa89iwwJVRJzTYDR8XsJk7ERKkKFG9boRea6j","amount":100}]},"reqId":3866864878,"protocolVersion":2,"identifier":"V4SGRU86Z58d6TV7PBUe6f"}
```

This command will append your signature to transaction.

Indy CLI will print the signed transaction after these steps. 

Example output:

```json
{"identifier":"V4SGRU86Z58d6TV7PBUe6f","operation":{"outputs":[{"address":"2749JKC5gFV5fAa89iwwJVRJzTYDR8XsJk7ERKkKFG9boRea6j","amount":100}],"type":"10000"},"protocolVersion":2,"reqId":3866864878,"signatures":{"V4SGRU86Z58d6TV7PBUe6f":"2RMb9Wjt7Gbu3XVQXon8q1dDAJQbA6sJ2wkHcKfwDJtcqMQp1gVFgmZj9A625BM5fSwKebZtUh8r6fQYuzMorsy7"}}
```

The output should be send to the next Trustee to sign it.

### Sending the signed transaction to the ledger

##### Preparation
* Open `indy-cli`
* Open pool `pool connect <pool_name>`

##### Action

Send signed transaction to the ledger with a command 
```
ledger custom <signed_transaction_json>
```

Example input: 
```
ledger custom {"identifier":"V4SGRU86Z58d6TV7PBUe6f","operation":{"outputs":[{"address":"2749JKC5gFV5fAa89iwwJVRJzTYDR8XsJk7ERKkKFG9boRea6j","amount":100}],"type":"10000"},"protocolVersion":2,"reqId":3866864878,"signatures":{"V4SGRU86Z58d6TV7PBUe6f":"2RMb9Wjt7Gbu3XVQXon8q1dDAJQbA6sJ2wkHcKfwDJtcqMQp1gVFgmZj9A625BM5fSwKebZtUh8r6fQYuzMorsy7"}}
```

Indy CLI will print ledger response json.

Example output:

```json
{"op":"REPLY","result":{"auditPath":["DypRLG7cfWcK3V9wVk9tz5WU1aNindKX4KichNmUFQnJ"],"reqSignature":{"values":[{"value":"2RMb9Wjt7Gbu3XVQXon8q1dDAJQbA6sJ2wkHcKfwDJtcqMQp1gVFgmZj9A625BM5fSwKebZtUh8r6fQYuzMorsy7","from":"V4SGRU86Z58d6TV7PBUe6f"}],"type":"ED25519"},"ver":"1","rootHash":"GFsj1r5PZW55gMPbFELD68JjupHMD56nKvRzNHsdExvX","txn":{"data":{"outputs":[{"amount":100,"address":"2749JKC5gFV5fAa89iwwJVRJzTYDR8XsJk7ERKkKFG9boRea6j"}]},"metadata":{"digest":"f51f8cdab2644ffc25a37bad0bf05f9cfaf4eb2f8ed6e352d2a9b2b27342a8d5","reqId":3866864878,"from":"V4SGRU86Z58d6TV7PBUe6f"},"protocolVersion":2,"type":"10000"},"txnMetadata":{"txnTime":1553672180,"seqNo":2}}}
```