
# UTXO data structures

## method: build_get_utxo_request_handler
### inputs: 

    submitter_did
    payment_address : <"pay:sov:{address}{checksum}">
    
### return:

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
    
    
    
## method: parse_get_utxo_response_handler
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
    note: ver field is just in case the output format changes in the future
```json
    {
    "ver": 1,
    "utxos": 
        [
            {"address" : "pay:sov:2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", "seqno": 2, "amount": 10 }, 
            {"address" : "pay:sov:2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", "seqno": 3, "amount": 5 },
        ]
    }
```


## method: build_payment_req_handler
### inputs:
    wallet_handle
    submitter_did
    inputs_json
```json
    {
    "ver": 1,
    "utxos": 
        [
            {"address" : "pay:sov:QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH", "seqno": 2, "amount": 10 }, 
            {"address" : "pay:sov:t3gQdtHYZaEHTL92j81QEpv5aUHmHKPGQwjEud6mbyhuwvTjV", "seqno": 5, "amount": 15 },
            {"address" : "pay:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", "seqno": 14, "amount": 15 },
        ]
    }
```    
   
    outputs_json
```json
    {
    "ver": 1,
    "destinationAddresses": 
        [
            {"address" : "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC", "amount": 11 },
            {"address" : "pay:sov:2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h", "amount": 19 }, 
            {"address" : "pay:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", "amount": 9 },
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
            "outputs": [
                ["2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC", 11],
                ["2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h", 19],
                ["2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", 9]
            ],
            "inputs": [
                ["QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH", 2, "3TMn17XTUd7Qr93hiuBWJFyihZ7aQSDbZTwqJEepUFQ5NRoCYYA2ARih2eQLNUZcB2wDSeQaxRFXhrcW2a5RyXrx"],
                ["t3gQdtHYZaEHTL92j81QEpv5aUHmHKPGQwjEud6mbyhuwvTjV", 5, "4hPYHU1gBnC3ViQEyWf4zz3UPSrT364BfgP5YupBFv6HiuTh7JNLKKDLiiuwxHDHRd4o8AQwGVTT7nJHNTVq8NZy"],
                ["2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", 14, "2VvANwBDYNcHyyheGSHx2og7Pc31hw5Box74xZ1EYrm6HijeKqAnKGX6dHF8gL6x78vWUgTpHRA5V41YB7EJMcKq"]
            ]
        }
    }
```     
    
## method: parse_payment_response_handler
### inputs:
    resp_json
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
    json
``` 
    {
        "outputs": [
            ["2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC", 11],
            ["2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h", 19],
            ["2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", 9]
        ],
        "inputs": [
                ["QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH", 3, "3TMn17XTUd7Qr93hiuBWJFyihZ7aQSDbZTwqJEepUFQ5NRoCYYA2ARih2eQLNUZcB2wDSeQaxRFXhrcW2a5RyXrx"],
                ["t3gQdtHYZaEHTL92j81QEpv5aUHmHKPGQwjEud6mbyhuwvTjV", 3, "4hPYHU1gBnC3ViQEyWf4zz3UPSrT364BfgP5YupBFv6HiuTh7JNLKKDLiiuwxHDHRd4o8AQwGVTT7nJHNTVq8NZy"],
                ["2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", 3, "2VvANwBDYNcHyyheGSHx2og7Pc31hw5Box74xZ1EYrm6HijeKqAnKGX6dHF8gL6x78vWUgTpHRA5V41YB7EJMcKq"]
        ],
        "txnTime": 1527714130
    }
```
