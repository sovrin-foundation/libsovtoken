
# LibSovToken Data Structures
This document exists for multiple purposes:
* To assist the LibSovToken team and give them the information they need to write the payment handler methods
* To document the inputs and outputs of exposed functions for users of Indy-SDK when payments are handled by LibSovToken.
## Methods:
* [indy_create_payment_address](#method-indy_create_payment_address)
* [indy_add_request_fees](#method-indy_add_request_fees)
* [indy_parse_response_with_fees](#method-indy_parse_response_with_fees)
* [indy_build_get_utxo_request](#method-build_get_utxo_request)
* [indy_parse_get_utxo_response](#method-indy_parse_get_utxo_response)
* [indy_build_payment_req](#method-indy_build_payment_req)
* [indy_parse_payment_response](#method-indy_parse_payment_response)
* [indy_build_mint_req](#method-indy_build_mint_req)
* [indy_build_set_txn_fees_req](#method-indy_build_set_txn_fees_req)
* [indy_build_get_txn_fees_req](#method-indy_build_get_txn_fees_req)
* [indy_parse_get_txn_fees_response](#method-indy_parse_get_txn_fees_response)

## method: indy_create_payment_address
This API call is handled by LibSovToken create_payment_address_handler

### inputs:

    command_handle: command handle to map callback to context
    wallet_handle: wallet handle where to save new address
    payment_method: Payment method to use (for example, "sov")
    config: payment address config as json:
```
{
    "seed" : <str>, // allows deterministic creation of payment address
}
```

### returns:
    payment_address: public identifier of payment address will be of the format "pay:sov:<ed25519verkey><checksum>"


## method: indy_add_request_fees
This API call is handled by LibSovToken add_request_fees_handler.

### inputs:
    wallet_handle: wallet handle where payment keys are stored
    submitter_did : DID of request sender
    req_json: initial transaction request as json
    inputs_json: The list of UTXO inputs as json array:
``` 
[
    <str: txo_string>, 
]
    // Each txo string is of the format: "txo:sov:<base58 string>"
    // The base58 string can be decoded internally as {"address": <str:address>, "seqNo": <int>}

```
    outputs_json: The list of UTXO outputs as json array:
``` 
[
    {
        "address" : <str>,   // the payment address
        "amount": <int>,    // the payment amount
        "extra": <str>,     // optional field
    },
]

```
Example inputs_json:
``` 
[
    "txo:sov:fkjZEd8eTBnYJsw7m7twMph3UYD7j2SoWcDM45DkmRx8eq2SkQnzxoLxyMT1RBAat9x86MwXNJH88Pxf9u7JsM5m8ApXn3bvgbtS5cegZzNp7"
]

```
Example outputs_json:
``` 
[
    {
        "address": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
        "amount": 11,
        "extra": "",
    },
]
```

### return:
    req_with_fees_json: modified Indy request with added fees info
```
{
    <req_json>    //initial transaction request
    "fees":
    [
        [
            [<str: source payment address1>, <int: sequence number>],
        ],
        [
            [<str: change payment address1>, <int: amount of change>],
        ],
	    [<str: signature over source payment address1, sequence number, and all outputs>, ]
    ]
}
```
Example req_with_fees_json:
```
{
    <req_json>    //initial transaction request
    "fees":
    [
        [
	        ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 2]
	    ],
	    [
	        ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 9]
    	],
	    ["5Z7ktpfVQAhj2gMFR8L6JnG7fQQJzqWwqrDgXQP1CYf2vrjKPe2a27borFVuAcQh2AttoejgAoTzJ36wfyKxu5ox"]
    ]
}
```

## method: indy_parse_response_with_fees
This API call is handled by LibSovToken parse_response_with_fees_handler. *Note This is version 2 updated as of 6/20/18*

### inputs:
    command_handle
    payment_method
    resp_json: the JSON formatted response from the ledger
```
{
    "op": <str>,                //type of operation returned
    "protocolVersion":  <int>, // (optional) the version of the transaction response data structure
    "request":
    {
        "txn":
        {
            "data":
            {
                "alias": <str>,
                "dest": <str>,
                "verkey": <str>
            },
            "metadata":
            {
                "digest": <str>,
                "reqId": <str>
            },
            "protocolVersion": <int>,
            "type": "1"
        },
        "ver": <str>,
        "txnMetadata":
        {
            "seqNo": <int>,
            "txnTime": <int>
        },
        "reqSignature":
        {
            "type": <str>,
            "values":
            [
                {
                    "from": <str: DID that sent txn>,
                    "value": <str: signature of DID on txn>
                }
            ]
        },
        "rootHash": <str: root hash of ledger>,
        "auditPath":    // a list of strings
        [
            <str: hash of node in ledger>,
        ],
        "fees":
        {
            "inputs":   // a list of inputs
            [
                [<str: payment address>, <int: sequence number>],
            ],
            "outputs":
            [
                [<str: payment address>, <int: amount>]
            ],
            "fees": <int: amount>,
            "ref": <str: reference to txn for which fees were paid>,
            "reqSignature":
            {
                "type": <str: signature type>,
                "values":   // a list of signatures
                [
                    {
                        "from": <str: first input payment address>,
                        "value": <str: signature of payment address on outputs>
                    },
                ]
            },
            "txnMetadata":
            {
                "seqNo": <int: sequence number>,
                "txnTime": <int: seconds since the unix epoch>
            },
            "rootHash": <str: root hash of ledger>,
            "auditPath":    // a list of strings
            [
                <str: hash of node in ledger>,
            ]
        }
    }
}
```
Example resp_with_fees_json:
```
{
    "op": "REPLY",
    "protocolVersion": 1,
    "request":
    {
        "txn":
        {
            "data":
            {
                "alias": "508867",
                "dest": "8Wv7NMbsMiNSmNa3iC6fG7",
                "verkey": "56b9wim9b3dYXzzc8wnm8RZePbyuMoWw5XUXxL4Y9gFZ"
            },
            "metadata":
            {
                "digest": "54289ff3f7853891e2ba9f4edb4925a0028840008395ea717df8b1f757c4fc77",
                "reqId": 1529697827844532830
            },
            "protocolVersion": 2,
            "type": "1"
        },
        "ver": 1,
        "txnMetadata":
        {
            "seqNo": 13,
            "txnTime": 1529697829
        },
        "reqSignature":
        {
            "type": "ED25519",
            "values":
            [
                {
                    "from": "MSjKTWkPLtYoPEaTF1TUDb",
                    "value": "5Ngg5fQ4NtqdzgN3kSjdRKo6ffeq5sP264TmzxvGGQX3ieJzP9hCeUCu7RkmAhLjzqZ2Z5y8FLSptWxetS8FCmcs"
                }
            ]
        },
        "rootHash": "FePFuqEX6iJ1SP5DkYn9WTXQrThxqevEkxYXyCxyX4Fd",
        "auditPath":
        [
            "CWQ9keGzhBqyMRLvp7XbMr7da7yUbEU4qGTfJ2KNxMM6",
            "2S9HAxKukY2hxUoEC718fhywF3KRfwPnEQvRsoN168EV"
        ],
        "fees":
        {
            "inputs":
            [
                ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 2]
            ],
            "outputs":
            [
                ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 9]
            ],
            "fees": 4,
            "ref": "1:13",
            "reqSignature":
            {
                "type": "ED25519",
                "values":
                [
                    {
                        "from": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
                        "value": "5Z7ktpfVQAhj2gMFR8L6JnG7fQQJzqWwqrDgXQP1CYf2vrjKPe2a27borFVuAcQh2AttoejgAoTzJ36wfyKxu5ox"
                    }
                ]
            },
            "txnMetadata":
            {
                "seqNo": 2,
                "txnTime": 1529697829
            },
            "rootHash": "A8qwQKyKUMd3PnJTKe4bXRzajCUVgSd1J1A7jdahhNW6",
            "auditPath": ["Gyw5iBPPs4KSiEoAXQcjv8jw1VWsFjTVyCkm1Zp9E3Pa"]
        }
    }
}
```

### return:
    utxo_json - parsed utxo info as json:
```
[
    {
        "paymentAddress": <str>,// sovrin payment address: "pay:sov:<address><checksum>"
        "txo": <str>,           // txo string: "txo:sov:<base58 encoded two identifier>"
        "amount": <int>,        // amount of tokens in this input
        "extra": <str>          // optional data from payment transaction
    }
]
```
Example utxo_json:
```
[
    {
        "paymentAddress": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
	"txo": "txo:sov:fkjZEd8eTBnYJsw7m7twMph3UYD7j2SoWcDM45DkmRx8eq2SkQnzxoLxyMT1RBAat9x86MwXNJH88Pxf9u7JsM5m8ApXn3bvgb"
        "amount": 11,
        "extra":
    }
]
```


## method: indy_build_get_utxo_request
This API call is handled by LibSovToken build_get_utxo_request_handler
### inputs:
    wallet_handle: wallet handle
    submitter_did : DID of request sender
    payment_address: "pay:sov:<address><checksum>"  //target payment address

### return:
    get_utxo_txn_json: Indy request for getting UTXO list for payment address
```
{
    "identifier": <str>,        // the payment address
    "operation":
    {
        "address": <str>,       // the payment address
        "type": 10002
    },
    "reqId": <int>,             // a random identifier
    "protocolVersion": <int>    // (optional)  the version of the client/node communication protocol
}

```
Example get_utxo_txn_json:
```

{
    "identifier": "2jyMWLv8NuxUV4yDc46mLQMn9WUUzeKURX3d2yQqgoLqEQC2sf",
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
This API call is handled by LibSovToken parse_get_utxo_response_handler *Note this should not change because it is parsing a read request. It should stay at version 1*
### inputs:
    resp_json: the JSON formatted response from the ledger
```
{
    "op": "REPLY",
    "protocolVersion": <int>    // (optional)  the version of the client/node communication protocol
    "result": {
        "type": "10002",
        "address": <str>,       // the payment address
        "identifier": <str>,    // the payment address
        "reqId": <int>,         // a random identifier
        "outputs": [
            ["<str: address>", <int: sequence number>, <int: amount>],
        ],
        "state_proof":
        {
            "multi_signature":
            {
                "participants": [ <str>, ],
                "signature": <str>
                "value":
                {
                    "ledger_id": <int>,
                    "pool_state_root_hash": <str>,
                    "state_root_hash": <str>,
                    "timestamp": <int>,
                    "txn_root_hash": <str>
                }
            },
            "proof_nodes": <str>,
            "root_hash": <str>
        }
    }
}

```
Example resp_json from the ledger:
```
{
    "op": "REPLY",
    "result":
    {
        "type": "10002",
        "address": "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
        "identifier": "6ouriXMZkLeHsuXrN1X1fd",
        "reqId": 15424,
        "outputs":
        [
            ["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 1, 40]
        ],
        "state_proof":
        {
            "multi_signature":
            {
                "participants": ["Gamma", "Alpha", "Delta"],
                "signature": "RNUfcr74ekwBxsT7mxnT2RDFaRRYbfuhebnqQW9PsGkf1bsKC8m8DAqsFfMMLGgAy9CSWM8cyXRUdWLrKUywTajbySfy18oxxdg8ZZApGYHZtiuj6y9sbScAyMwWMmxrDErrj8DWVEVZbGMhPnSSUkmkC6SBnZtSDfdRDvHUMQVBRR",
                "value":
                {
                    "ledger_id": 1001,
                    "pool_state_root_hash": "9i3acxaDhCfx9jWXW2JZRoDWzRQEKo7bPBVN7VPE1Jhg",
                    "state_root_hash": "8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea",
                    "timestamp": 1529705683,
                    "txn_root_hash": "67khbUNo8rySwEtW2SPSsyK4rmLCS7JAN4kYnppELajc"
                }
            },
            "proof_nodes": "+I74ObM0Y3RLU1hCYnYyTXkzVEdHVWdURmpreHUxQTlKTTNTc2NkNUZ5ZFk0ZGt4bmZ3QTdxOjGEw4I0MPhRgICAgICAoKwYfN+WIsLFSOuMjp224HzlSFoSXhXc1+rE\\/vB8jh7MoF\\/sqT9NVI\\/hFuFzQ8LUFSymIKOpOG9nepF29+TB2bWOgICAgICAgICA",
            "root_hash": "8tJkWdp9wdz3bpb5s5hPDfrjWCQTPmsFKrSdoPmTTnea"
        }
    }
}

```


### return:
    utxo_json - parsed utxo info as json:
```
[
    {
        "paymentAddress": <str>,// full sovrin payment address: "pay:sov:<address><checksum>"
        "txo": <str>,           // txo string: "{"address": "pay:sov:<address>", "seqNo": <int>}"
        "amount": <int>,        // amount of tokens in this input
        "extra": <str>          // optional data from payment transaction
    }
]
```
Example utxo_json:
```
[
    {
        "paymentAddress": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
	"txo": "txo:sov:fkjZEd8eTBnYJsw7m7twMph3UYD7j2SoWcDM45DkmRx8eq2SkQnzxoLxyMT1RBAat9x86MwXNJH88Pxf9u7JsM5m8ApXn3bvgb"
        "amount": 11,
        "extra":
    },
    {
        "paymentAddress": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
	"txo": "txo:sov:WqXg36yxheP7wzUZnhnkUY6Qeaib5uyUZuyaujr7atPHRH3d2Aat9x86MwXNw88RAojPpdgxLPQyC1oJH88Pxf9u7JsM5m8ApXn"
        "amount": 3,
        "extra":
    }
]
```

## method: indy_build_payment_req
This API call is handled by LibSovToken build_payment_req_handler. *Note this has been changed back after a commit error. This is up to date as of 6/20/18*
### inputs:
    wallet_handle: wallet handle
    submitter_did : DID of request sender
    inputs_json: The list of UTXO inputs as json array:
```
[
    <str: txo_string>, 
]
    // Each txo string is of the format: "txo:sov:<base58 string>"
    // The base58 string can be decoded internally as {"address": <str:address>, "seqNo": <int>}

```
    outputs_json: The list of UTXO outputs as json array:
``` 

[
    {
        "address" : <str>,   // the payment address
        "amount": <int>,    // the payment amount
        "extra": <str>      // optional field
    },
]

```
Example inputs_json:
``` 
[
    "txo:sov:QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBHnzx2mVXsXyVADzSDw88RAojPpdw88RAojPpdgxLPQyCgxLPQyC1oJUqkrLeU",
    "txo:sov:t3gQdtHYZaEHTL92j81QEpv5aUHmHKPGQwjEud6mbyhuwvTjxoLxyMT1RBAat9x86MwXNJH88Pxf9u7JsM5m8ApXn3bvouG3yHqnK2LvVWVjV",
    "txo:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgxggsbNTmVMy1RBAat9x86MwXNJH88Pxf9u7JsM5m8ApXn3bvgbtSxggsbNJDa5z7"
]
```
Example outputs_json:
``` 
[
    {
        "address": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
        "amount": 11,
        "extra": ""
    },
    {
        "address": "pay:sov:2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h",
        "amount": 19,
        "extra": ""
   },
   {
       "address": "pay:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy",
       "amount": 9,
       "extra": ""
   },
]
```

### return:

    payment_req_json
    note: any difference between the sum of the inputs and the sum of outputs is the fees amount
``` 
{
    "identifier": <str>,        // first <source payment address w/o checksum>
    "reqId": <int>,             // a random identifier
    "protocolVersion": <int>,   // (optional) the protocol version
    "operation": {
        "type": "10001",
        "inputs": [
            [<str: source payment address>, <int: sequence number>],
        ],
        "outputs": [
            [<str: change payment address>, <int: amount of change>],
        ],
        "extra": <str>,     // optional field
        "signatures": [
            <string: signature over source payment address, sequence number, and all outputs>,
        ]
    }
}
```
Example payment_req_json:
    note: output to ledger excludes address prefix "pay:sov"
    note: any difference between the sum of the inputs and the sum of outputs is the fees amount
``` 
{
    "identifier": "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1",
    "reqId": 1529682415342024,
    "protocolVersion": 2,
    "operation":
    {
        "type": "10001",
        "inputs":
        [
            ["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 1]
        ],
        "outputs":
        [
            ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 13],
            ["24xHHVDRq97Hss5BxiTciEDsve7nYNx1pxAMi9RAvcWMouviSY", 13],
            ["mNYFWv9vvoQVCVLrSpbU7ZScthjNJMQxMs3gREQrwcJC1DsG5", 13],
            ["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 1]
        ],
        "extra": None,
        "signatures": ["4fFVD1HSVLaVdMpjHU168eviqWDxKrWYx1fRxw4DDLjg4XZXwya7UdcvVty81pYFcng244tS36WbshCeznC8ZN5Z"]
    }
}
```

## method: indy_parse_payment_response
This API call is handled by LibSovToken parse_payment_response_handler.
### inputs:
    resp_json: This is an example of the JSON that will be returned from the ledger after submitting a payment request.

resp_json
```
{
    "op": <str>,        //type of operation returned
    "protocolVersion": <int>,   // (optional) the protocol version
    "result":
    {
        "txn":
        {
            "data":
            {
                "inputs": [
                    [<str: source payment address>, <int: sequence number>],
                ],
                "outputs": [
                    [<str: change payment address>, <int: amount>],
                ],
                "extra": <str>,     // optional field
            },
            "metadata":
            {
                "digest": <str>,    //
                "from": <str>,      // one of the input payment addresses
                "reqId": <int>      // a random identifier
            },
            "protocolVersion": <int>,
            "type": "10001"
        },
        "ver": <str>,
        "reqSignature":
        {
            "type": <str: signature type>,
            "values":   // a list of signatures
            [
                {
                    "from": <str: first input payment address>,
                    "value": <str: signature of payment address on outputs>
                },
            ]
        },
        "txnMetadata":
        {
            "seqNo": <int: sequence number>,
            "txnTime": <int: seconds since the unix epoch>
        },
        "rootHash": <str: root hash of ledger>,
        "auditPath":    // a list of strings
        [
            <str: hash of node in ledger>,
        ]
    }
}
```

Example resp_json:
```
{
    "op": "REPLY",
    "protocolVersion": 2,
    "result":
    {
        "txn":
        {
            "data":
            {
                "extra": None,
                "inputs":
                [
                    ["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 1]
                ],
                "outputs":
                [
                    ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 13],
                    ["24xHHVDRq97Hss5BxiTciEDsve7nYNx1pxAMi9RAvcWMouviSY", 13],
                    ["mNYFWv9vvoQVCVLrSpbU7ZScthjNJMQxMs3gREQrwcJC1DsG5", 13],
                    ["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 1]
                ]
            },
            "metadata":
            {
                "digest": "228af6a0c773cbbd575bf4e16f9144c2eaa615fa81fdcc3d06b83e20a92e5989",
                "from": "6baBEYA94sAphWBA5efEsaA6X2wCdyaH7PXuBtv2H5S1",
                "reqId": 1529682415342024
            },
            "protocolVersion": 2,
            "type": "10001"
        },
        "reqSignature":
        {
            "type": "ED25519",
            "values":
            [
                {
                    "from": "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
                    "value": "4fFVD1HSVLaVdMpjHU168eviqWDxKrWYx1fRxw4DDLjg4XZXwya7UdcvVty81pYFcng244tS36WbshCeznC8ZN5Z"
                }
            ]
        },
        "txnMetadata":
        {
            "seqNo": 2,
            "txnTime": 1529682415
        },
        "ver": "1",
        "auditPath": ["5NtSQUXaZvETP1KEWi8LaxSb9gGa2Qj31xKQoimNxCAT"],
        "rootHash": "GJFwiQt9r7n25PqM1oXBtRceXCeoqoCBcJmRH1c8fVTs"
    }
}
```
### return:
    utxo_json: parsed utxo info as json
```
[
    {
        "paymentAddress": <str>,// sovrin payment address: "pay:sov:<address><checksum>"
        "txo": <str>,           // txo string: "txo:sov:<base58 encoded two identifier>"
        "amount": <int>,        // amount of tokens in this input
        "extra": <str>          // optional data from payment transaction
    }
]
```
Example utxo_json:
```
[
    {
        "paymentAddress": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
	"txo": "txo:sov:fkjZEd8eTBnYJsw7m7twMph3UYD7j2SoWcDM45DkmRx8eq2SkQnzxoLxyMT1RBAat9x86MwXNJH88Pxf9u7JsM5m8ApXn3bvgb"
        "amount": 11,
        "extra":
    },
    {
        "paymentAddress": "pay:sov:2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h",
        "txo": "txo:sov:2k7K2zwNTF7po3UYD7j2SoWcDM45DkmRx8eq2SkQnzxoLxyM2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h",
        "amount": 19,
        "extra": ""
    },
    {
        "paymentAddress": "pay:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy",
        "txo": "txo:sov:2SBZcBgBHzU1d9u7jxggsbNJDDM45DkmRx8eq2SkQnzxoLxyMT1RBAat9x86MwX2SBZcBgBHzU1d9u7jx3v13V5oR6eZgTmVMy",
        "amount": 9,
        "extra": ""
    }
]
```


## method: indy_build_mint_req
This API call is handled by LibSovToken build_mint_txn_handlerr

### inputs:
    wallet_handle: wallet handle
    submitter_did : DID of request sender
    outputs_json: The list of UTXO outputs as json array:
``` 
[
    {
    "paymentAddress": <str>, // payment address used as output
    "amount": <int>, // amount of tokens to transfer to this payment address
    "extra": <str>, // optional data
    },
]
```
Example outputs_json:
``` 
[
    {
        "paymentAddress": "sjw1ceG7wtym3VcnyaYtf1xo37gCUQHDR5VWcKWNPLRZ1X8eC",
        "amount": 60,
        "extra": ""
    },
    {
        "paymentAddress": "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
        "amount": 40,
        "extra": ""
    }
]
```

### return:
    payment_method
    mint_req_json: Indy request for minting tokens
```
{
    "reqId": <int>,             // a random identifier
    "protocolVersion": <int>,   // the version of the client/node communication protocol
    "identifier": <string>
    "operation": {
        "type": "10000",
        "outputs": [
            [<str: output payment address>, <int: amount to mint>],
        ]
    }
}
```
Example mint_req_json:
```
{
    "reqId": 1527799618700635,
    "protocolVersion": 1,
    "identifier": "V4SGRU86Z58d6TV7PBUe6f"
    "operation": {
        "type": "10000",
        "outputs": [
            ["sjw1ceG7wtym3VcnyaYtf1xo37gCUQHDR5VWcKWNPLRZ1X8eC", 60],
            ["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 40]
        ]
    }
}
```

## method: indy_build_set_txn_fees_req
This API call is handled by LibSovToken build_set_txn_fees_handler

### inputs:
    command_handle
    wallet_handle: wallet handle
    submitter_did : DID of request sender
    payment_method
    fees_json
```
{
    <str: txnType>: <int: amount>,
}
```
Example fees_json:
```
{
    "1": 4,
    "10001": 8
}
```

### return:
    set_txn_fees_json - Indy request for setting fees for transactions in the ledger
```
{
    "reqId": <int>,             //random identifier
    "protocolVersion": <int>,   // the version of the client/node communication protocol
    "operation": {
        "type": "20000",
        "fees": {
            <str: txnType>: <int: amount>,
        }
    },
}
```

Example set_txn_fees_json:
```
{
    "reqId": 1527801087197612,
    "protocolVersion": 1,
    "operation": {
        "type": "20000",
        "fees": {
            "1": 4,
            "10001": 8
        }
    }
}
```

## method: indy_build_get_txn_fees_req
This API call is handled by LibSovToken build_get_txn_fees_handler

### inputs:
    command_handle
    wallet_handle: wallet handle
    submitter_did : DID of request sender
    payment_method

### return:
    get_txn_fees_json - Indy request for getting fees for transactions in the ledger
```
{
    "identifier": <str>,        // the submitter DID
    "reqId": <int>,             // a random identifier
    "protocolVersion": <int>,   // the version of the client/node communication protocol
    "operation": {
        "type": "20001"
    }
}
```
Example get_txn_fees_json:
```
{
    "identifier": "6ouriXMZkLeHsuXrN1X1fd",
    "reqId": 47660,
    "protocolVersion": 1,
    "operation": {
        "type": "20001"
    }
}
```

## method: indy_parse_get_txn_fees_response
This API call is handled by LibSovToken parse_get_txn_fees_response_handler. *Note: this transaction format will not change because it"s a read request and not a write request.*

### inputs:
    command_handle
    payment_method
    resp_json: response from the ledger for Indy request for getting fees
```
{
    "op": "REPLY",
    "result": {
        "identifier": <str>,        // the submitter DID
        "reqId": <int>,             // a random identifier
        "type": "20001",
        "fees": {
            <str: txnType>: <int: amount>,
        },
        "state_proof": {
            {
                "participants": [ <str>, ], // the nodes that participated in consensus
                "signature": <str> // the BLS signature of the nodes
                "value":
                {
                    "ledger_id": <int>, // the associated ledger where the state proof came from
                    "pool_state_root_hash": <str>, // the state proof root hash of the pool ledger
                    "state_root_hash": <str>, // the state proof root hash of the total ledgers
                    "timestamp": <int>, // the time the transaction was committed
                    "txn_root_hash": <str> // the transaction root hash of the transaction on a specific ledger
                }
            },
            "rootHash": <str>,      // the root hash of the transaction
            "proof_nodes": <str>,   // the hash of each node in the path
        }
    }
}
```

Example resp_json:
```
{
    "op": "REPLY",
    "result":
    {
        "identifier": "6ouriXMZkLeHsuXrN1X1fd",
        "reqId": 10378,
        "type": "20001",
        "fees":
        {
            "1": 4,
            "10001": 8
        },
        "state_proof":
        {
            "multi_signature": {//TODO add valid json string in here},
            "proof_nodes": "29qFIGZlZXOT0pF7IjEiOjQsIjEwMDAxIjo4fQ==",
            "root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms"
        },
    }
}
```

### return:
    fees_json
```
{
    <str: txnType>: <int: amount>,
}
```

Example fees_json:
```
{
    "10001": 8,
    "1": 4
}
```
