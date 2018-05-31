
# LibSovToken Data Structures
This document exists for multiple purposes:
* To assist the LibSovToken team and give them the information they need to write the payment handler methods
* To document the inputs and outputs of exposed functions for users of Indy-SDK when payments are handled by LibSovToken.

## method: indy_create_payment_address
This API call is handled by LibSovToken create_payment_address_handler

### inputs:

    command_handle: command handle to map callback to context
    wallet_handle: wallet handle where to save new address
    payment_method: Payment method to use (for example, 'sov')
    config: payment address config as json:
```json
{
    seed: <str>, // allows deterministic creation of payment address
}
```

### returns:
    payment_address: public identifier of payment address will be of the format "pay:sov:<ed25519verkey><checksum>"


## method: indy_add_request_fees
This API call is handled by LibSovToken add_request_fees_handler

### inputs:
    wallet_handle: wallet handle where payment keys are stored
    submitter_did : DID of request sender
    req_json: initial transaction request as json
    inputs_json: The list of UTXO inputs as json array:
```json
    {
    "ver": 1,   // this field is included to allow for future backward compatability
    "inputs_json":    // Each of these elements is the same as the txo strings returned by indy_parse_get_utxo_response
        [
            "{\"address\" : \"pay:sov:QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH\", \"seqno\": 2}"
        ]
    }
```

    outputs_json: The list of UTXO outputs as json array:
```json
    {
    "ver": 1,
    "outputs_json":
        [
            {"address" : "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC", "amount": 11, "extra": },
        ]
    }
```

### return:
    req_with_fees_json: modified Indy request with added fees info
```json
    <req_json>    //initial transaction request
    "fees": [
        [
            // [ <source payment address>, <sequence number>, <signature over source payment address, sequence number, and all outputs>]
            ['QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH', 2, '5Z7ktpfVQAhj2gMFR8L6JnG7fQQJzqWwqrDgXQP1CYf2vrjKPe2a27borFVuAcQh2AttoejgAoTzJ36wfyKxu5ox']
        ],
        [
            // [<change payment address>, <amount of change>]
            ['2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC', 11]
        ]
    ],

```

## method: indy_parse_response_with_fees
This API call is handled by LibSovToken parse_response_with_fees_handler

### inputs:
    command_handle
    payment_method
    resp_with_fees_json: response for Indy request with fees
```json
    <resp_json>  //initial transaction response
    "fees": [
        [
            ['QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH', 2, '5Z7ktpfVQAhj2gMFR8L6JnG7fQQJzqWwqrDgXQP1CYf2vrjKPe2a27borFVuAcQh2AttoejgAoTzJ36wfyKxu5ox']
        ],
        [
            ['2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC', 11]
        ],
        3   // the sequence number of the fees transaction
    ],

```

### return:
    utxo_json - parsed utxo info as json:
```json
    "ver": 1,
    "utxo_json":
        [{
            "paymentAddress": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC"
            "txo": "{\"address\" : \"pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC\", \"seqno\": 3}"
            "amount": 11,   // amount of tokens in this input
            "extra":        // optional data from payment transaction
        }]
```


## method: indy_build_get_utxo_request
This API call is handled by LibSovToken build_get_utxo_request_handler
### inputs:
    wallet_handle: wallet handle
    submitter_did : DID of request sender
    payment_address: <"pay:sov:{address}{checksum}">  //target payment address
    
### return:
    get_utxo_txn_json: Indy request for getting UTXO list for payment address
```json
{
    "identifier": "6ouriXMZkLeHsuXrN1X1fd",
    "operation":
    {
        "address": "2jyMWLv8NuxUV4yDc46mLQMn9WUUzeKURX3d2yQqgoLqEQC2sf",
        "type": "10002"
    },
    "reqId": 6284,
    "protocolVersion": 1
}

```

    
## method: indy_parse_get_utxo_response
This API call is handled by LibSovToken parse_get_utxo_response_handler
### inputs:
version 1, it will change in a later version.  
    resp_json
```json
{
    "op": "REPLY",
    "protocolVersion": 1,
    "result": {
        "type": "10002",
        "address": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
        "identifier": "6ouriXMZkLeHsuXrN1X1fd",
        "reqId": 23887,
        "outputs": [
            ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 2, 10], 
            ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 3, 3]
        ],
    }
}

```
    
    
### return:
    utxo_json
```json
    {
    "ver": 1,
    "utxo_json":[
        {
            "paymentAddress": "pay:sov:2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es"
            "txo": "{\"address\" : \"pay:sov:2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es\", \"seqno\": 2}",
            "amount": 10,   // amount of tokens in this input
            "extra":        // optional data from payment transaction
        },
        {
            "paymentAddress": "pay:sov:2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es"
            "txo": "{\"address\" : \"pay:sov:2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es\", \"seqno\": 3}",
            "amount": 3,   // amount of tokens in this input
            "extra":        // optional data from payment transaction
        }
    ]
}
```


## method: indy_build_payment_req
This API call is handled by LibSovToken build_payment_req_handler
### inputs:
    wallet_handle: wallet handle
    submitter_did : DID of request sender
    inputs_json: The list of UTXO inputs as json array:
```json
    {
    "ver": 1,
    "inputs_json":
        [
            "{\"address\" : \"pay:sov:QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH\", \"seqNo\": 2 }",
            "{\"address\" : \"pay:sov:t3gQdtHYZaEHTL92j81QEpv5aUHmHKPGQwjEud6mbyhuwvTjV\", \"seqNo\": 5 }",
            "{\"address\" : \"pay:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy\", \"seqNo\": 14 }",
        ]
    }
```
    outputs_json: The list of UTXO outputs as json array:
```json
    {
    "ver": 1,
    "outputs_json":
        [
            {
                "paymentAddress" : "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
                "amount": 11,
                "extra":
            },
            {
                "address" : "pay:sov:2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h",
                "amount": 19
                "extra":
            },
            {
                "address" : "pay:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy",
                "amount": 9
                "extra":
            },
        ]
    }
```


    
### return:

    payment_req_json
    note: output to ledger excludes address prefix "pay:sov"
    note: any difference between the sum of the inputs and the sum of outputs is the fees amount
``` json
    {
        "identifier": "DiHngdSyNFVs1CRcLxVA84xuKZLNhVWzSkdsnwJveKtN",
        "reqId": 1527714086374556,
        "operation": {
            "type": "10001",
            "extra": null,
            "inputs": [
                ["QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH", 2, "3TMn17XTUd7Qr93hiuBWJFyihZ7aQSDbZTwqJEepUFQ5NRoCYYA2ARih2eQLNUZcB2wDSeQaxRFXhrcW2a5RyXrx"],
                ["t3gQdtHYZaEHTL92j81QEpv5aUHmHKPGQwjEud6mbyhuwvTjV", 5, "4hPYHU1gBnC3ViQEyWf4zz3UPSrT364BfgP5YupBFv6HiuTh7JNLKKDLiiuwxHDHRd4o8AQwGVTT7nJHNTVq8NZy"],
                ["2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", 14, "2VvANwBDYNcHyyheGSHx2og7Pc31hw5Box74xZ1EYrm6HijeKqAnKGX6dHF8gL6x78vWUgTpHRA5V41YB7EJMcKq"]
            ],
            "outputs": [
                ["2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC", 11],
                ["2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h", 19],
                ["2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", 9]
            ]
        }
    }
```     
    
## method: indy_parse_payment_response
This API call is handled by LibSovToken parse_payment_response_handler
### inputs:
    resp_json - This is an example of the JSON that will be returned from the ledger after submitting a payment request. It is shown here for information only.
```json 
    {
        "op": "REPLY",
        "result": {
            "identifier": "DiHngdSyNFVs1CRcLxVA84xuKZLNhVWzSkdsnwJveKtN",
            "outputs": [
                ["2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC", 11],
                ["2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h", 19],
                ["2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", 9]
            ],
            "seqNo": 4,
            "rootHash": "FRkqRd5jyNRK3SGSGNoR6xMmYQvLVnotGLGWYxR1dCN4",
            "signature": null,
            "extra": null,
            "inputs": [
                ["QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH", 3, "3TMn17XTUd7Qr93hiuBWJFyihZ7aQSDbZTwqJEepUFQ5NRoCYYA2ARih2eQLNUZcB2wDSeQaxRFXhrcW2a5RyXrx"],
                ["t3gQdtHYZaEHTL92j81QEpv5aUHmHKPGQwjEud6mbyhuwvTjV", 3, "4hPYHU1gBnC3ViQEyWf4zz3UPSrT364BfgP5YupBFv6HiuTh7JNLKKDLiiuwxHDHRd4o8AQwGVTT7nJHNTVq8NZy"],
                ["2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", 3, "2VvANwBDYNcHyyheGSHx2og7Pc31hw5Box74xZ1EYrm6HijeKqAnKGX6dHF8gL6x78vWUgTpHRA5V41YB7EJMcKq"]
            ],
            "signatures": null,
            "reqId": 1527714086374556,
            "auditPath": [
                "6QFFFVbio2q8viWBbuVfvQsv3Qgd3Ub64Qv41i5wH8Bo", "8vDzQmeYb8ecQ7Nyv5i6V8nUwT3fsebqTHMXqgzYi1NU"
            ],
            "type": "10001",
            "txnTime": 1527714130
        }
    }
```
    
### return:
    utxo_json - parsed utxo info as json:
    json
``` 
    {
        "ver" : 1,
        "utxo_json": [
            {
                "paymentAddress": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
                "txo": "{\"address\" : \"pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC\", \"seqNo\" : 4}",
                "amount": 11,
                "extra":
            },
            {
                "paymentAddress": "pay:sov:2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h",
                "txo": "{\"address\" : \"pay:sov:2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h\", \"seqNo\" : 4}",
                 "amount": 19,
                "extra":
            },
            {
                "paymentAddress": "pay:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy",
                "txo": "{\"address\" : \"pay:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy\", \"seqNo\" : 4}",
                "amount": 9,
                "extra":
            }
        ]
    }
```

## method: indy_build_set_txn_fees_req
This API call is handled by LibSovToken build_set_txn_fees_handler

### inputs:

### return:

## method: indy_build_get_txn_fees_req
This API call is handled by LibSovToken build_get_txn_fees_handler

### inputs:

### return:

## method: indy_parse_get_txn_fees_response
This API call is handled by LibSovToken parse_get_txn_fees_response_handler

### inputs:

### return: