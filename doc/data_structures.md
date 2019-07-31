
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
* [indy_build_verify_payment_request](#method-indy_build_verify_payment_request)
* [indy_parse_verify_payment_response](#method-indy_parse_verify_payment_response)

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

    command_handle: command handle to map callback to context
    wallet_handle: wallet handle where payment keys are stored
    submitter_did : DID of request sender
    req_json: initial transaction request as json
    inputs_json: The list of payment sources as json array:
``` 
[
    <str: source_string>, 
]
    // Each source string is of the format: "src:sov:<base58 string>"
    // The base58 string can be decoded internally as {"address": <str:address>, "seqNo": <int>}

```
    outputs_json: The list of outputs as json array:
``` 
[
    {
        "recipient" : <str>,   // the payment address
        "amount": <int>,    // the payment amount
    },
]
    extra: // optional information for payment operation, this field is not used in the Sovrin fees or payment ledger
```
Example inputs_json:
``` 
[
    "src:sov:fkjZEd8eTBnYJsw7m7twMph3UYD7j2SoWcDM45DkmRx8eq2SkQnzxoLxyMT1RBAat9x86MwXNJH88Pxf9u7JsM5m8ApXn3bvgbtS5cegZzNp7"
]

```
Example outputs_json:
``` 
[
    {
        "address": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
        "amount": 11,
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
    payment_method - used payment method
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
This API call is handled by LibSovToken parse_response_with_fees_handler.

### inputs:
    command_handle: Command handle to map callback to caller context
    payment_method: payment method to use
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
    "protocolVersion": 2,
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
    receipts_json - parsed (payment method and node version agnostic) receipts info as json:
```
[
    {
        "recipient": <str>,     // sovrin payment address of recipient: "pay:sov:<address><checksum>"
        "receipt": <str>,       // receipt that can be used for payment referencing and verification: "rec:sov:<base58 encoded txn identifier>"
        "amount": <int>,        // amount of tokens in this input
        "extra": <str>          // optional data from payment transaction (Sovrin payment and fees ledger does not use this field for FEES transactions).
    }
]
```
Example receipts_json:
```
[
    {
        "recipient": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
	    "receipt": "rec:sov:fkjZEd8eTBnYJsw7m7twMph3UYD7j2SoWcDM45DkmRx8eq2SkQnzxoLxyMT1RBAat9x86MwXNJH88Pxf9u7JsM5m8ApXn3bvgb"
        "amount": 11,
        "extra":
    }
]
```


## method: indy_build_get_sources_request
This API call is handled by LibSovToken build_get_sources_request_handler
### inputs:
    wallet_handle: wallet handle
    submitter_did : DID of request sender
    payment_address: "pay:sov:<address><checksum>"  //target payment address

### return:
    get_sources_txn_json: Indy request for getting sources list for payment address
```
{
    "identifier": <str>,        // the payment address
    "operation":
    {
        "address": <str>,       // the payment address
        "type": 10002,
        "from": <int>           // shift to the next slice of payment sources
    },
    "reqId": <int>,             // a random identifier
    "protocolVersion": <int>    // (optional)  the version of the client/node communication protocol
}

```
    payment_method - used payment method
Example get_sources_txn_json:
```

{
    "identifier": "2jyMWLv8NuxUV4yDc46mLQMn9WUUzeKURX3d2yQqgoLqEQC2sf",
    "operation":
    {
        "address": "2jyMWLv8NuxUV4yDc46mLQMn9WUUzeKURX3d2yQqgoLqEQC2sf",
        "type": "10002",
        "from": 1
    },
    "reqId": 6284,
    "protocolVersion": 1
}

```

## method: indy_parse_get_sources_response
This API call is handled by LibSovToken parse_get_sources_response_handler 
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
        "next": <int>           // (optional) pointer to the next slice of payment sources
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
        "next": 1,
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
    next - (optional) pointer to the next slice of payment sources
    sources_json - parsed (payment method and node version agnostic) sources info as json:
```
[
    {
        "paymentAddress": <str>,// full sovrin payment address: "pay:sov:<address><checksum>"
        "source": <str>,        // source string: "src:sov:<base58 encoding of: {"address": <str:address, "seqNo": <int>}>
        "amount": <int>,        // amount of tokens in this input
        "extra": <str>          // optional data from payment transaction
    }
]
```
Example sources_json:
```
[
    {
        "paymentAddress": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
	    "source": "src:sov:fkjZEd8eTBnYJsw7m7twMph3UYD7j2SoWcDM45DkmRx8eq2SkQnzxoLxyMT1RBAat9x86MwXNJH88Pxf9u7JsM5m8ApXn3bvgb"
        "amount": 11,
        "extra":
    },
    {
        "paymentAddress": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
	    "source": "src:sov:WqXg36yxheP7wzUZnhnkUY6Qeaib5uyUZuyaujr7atPHRH3d2Aat9x86MwXNw88RAojPpdgxLPQyC1oJH88Pxf9u7JsM5m8ApXn"
        "amount": 3,
        "extra":
    }
]
```

## method: indy_build_payment_req
This API call is handled by LibSovToken build_payment_req_handler. 
### inputs:
    command_handle: Command handle to map callback to caller context.
    wallet_handle: wallet handle
    submitter_did : DID of request sender
    inputs_json: The list of payment sources as json array:
```
[
    <str: source_string>, 
]
    // Each source string is of the format: "src:sov:<base58 string>"
    // The base58 string can be decoded internally as {"address": <str:address>, "seqNo": <int>}

```
    outputs_json: The list of outputs as json array:
``` 

[
    {
        "address" : <str>,   // the payment address
        "amount": <int>,    // the payment amount
    },
]

```
    "extra": <str>      // optional field
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
    },
    {
        "address": "pay:sov:2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h",
        "amount": 19,
   },
   {
       "address": "pay:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy",
       "amount": 9,
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
    payment_method - used payment method
    
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
    command_handle: Command handle to map callback to caller context.
    payment_method: payment method to use
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
    receipts_json: parsed (payment method and node version agnostic) receipts info as json:
```
[
    {
        "recipient": <str>,     // sovrin payment address: "pay:sov:<address><checksum>"
        "receipt": <str>,       // receipt that can be used for payment referencing and verification: "rec:sov:<base58 encoded txn identifier>"
        "amount": <int>,        // amount of tokens in this input
        "extra": <str>          // optional data from payment transaction
    }
]
```
Example receipts_json:
```
[
    {
        "recipient": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
	    "receipt": "rec:sov:fkjZEd8eTBnYJsw7m7twMph3UYD7j2SoWcDM45DkmRx8eq2SkQnzxoLxyMT1RBAat9x86MwXNJH88Pxf9u7JsM5m8ApXn3bvgb"
        "amount": 11,
        "extra":
    },
    {
        "recipient": "pay:sov:2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h",
        "receipt": "rec:sov:2k7K2zwNTF7po3UYD7j2SoWcDM45DkmRx8eq2SkQnzxoLxyM2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h",
        "amount": 19,
        "extra": ""
    },
    {
        "recipient": "pay:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy",
        "receipt": "rec:sov:2SBZcBgBHzU1d9u7jxggsbNJDDM45DkmRx8eq2SkQnzxoLxyMT1RBAat9x86MwX2SBZcBgBHzU1d9u7jx3v13V5oR6eZgTmVMy",
        "amount": 9,
        "extra": ""
    }
]
```


## method: indy_build_mint_req
This API call is handled by LibSovToken build_mint_txn_handler

### inputs:
    command_handle: Command handle to map callback to caller context.
    wallet_handle: wallet handle
    submitter_did : DID of request sender
    outputs_json: The list of outputs as json array:
``` 
[
    {
    "paymentAddress": <str>, // payment address used as output
    "amount": <int>, // amount of tokens to transfer to this payment address
    },
]
```
    "extra": <str>, // optional information for mint operation, not used for Sovrin minting
Example outputs_json:
``` 
[
    {
        "paymentAddress": "sjw1ceG7wtym3VcnyaYtf1xo37gCUQHDR5VWcKWNPLRZ1X8eC",
        "amount": 60,
    },
    {
        "paymentAddress": "dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q",
        "amount": 40,
    }
]
```

### return:
    payment_method: used payment method
    mint_req_json: Indy request for minting tokens
        note: amount to mint is according to the smallest divisible part of the token
        for example, to mint 50 tokens in a system with 10<sup>8</sup> precision, the user must enter 5,000,000,000
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
            ["sjw1ceG7wtym3VcnyaYtf1xo37gCUQHDR5VWcKWNPLRZ1X8eC", 6000000000],
            ["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 4000000000]
        ]
    }
}
```

## method: indy_build_set_txn_fees_req
This API call is handled by LibSovToken build_set_txn_fees_handler

### inputs:
    command_handle: Command handle to map callback to caller context.
    wallet_handle: wallet handle
    submitter_did: DID of request sender
    payment_method: payment method to use
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
    command_handle: Command handle to map callback to caller context.
    wallet_handle: wallet handle
    submitter_did: DID of request sender
    payment_method: payment method to use

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
    command_handle: Command handle to map callback to caller context.
    payment_method: payment method to use
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
## method: indy_build_verify_payment_request
This API call is handled by LibSovToken build_verify_payment_request_handler
### inputs:
    command_handle: Command handle to map callback to caller context
    wallet_handle: wallet handle
    submitter_did : DID of request sender
    receipt: <receipt str>
        
    // Each receipt string is of the format: "rec:sov:<base58 string>"
    // The base58 string can be decoded internally as {"address": <str:address>, "seqNo": <int>}

### return:
    verify_txn_json: Indy request for getting sources list for payment address
```
{
    "identifier": <str>,        // the payment address
    "operation":
    {
        "type": "3"        
        "ledgerId": 1001,
        "data": <int>           // the transaction sequence number
    },
    "reqId": <int>,             // a random identifier
    "protocolVersion": <int>    // (optional)  the version of the client/node communication protocol
}

```
    payment_method - used payment method
Example verify_txn_json:
```
{
	"identifier":"V4SGRU86Z58d6TV7PBUe6f",
	"operation":{
		"type":"3",
		"data":24,	
		"ledgerId":1001
	},
	"reqId":1533834704418243379,
	"protocolVersion":2
}

```

## method: indy_parse_verify_payment_response
This API call is handled by LibSovToken parse_verify_payment_response_handler 
### inputs:
    command_handle: Command handle to map callback to caller context.
    payment_method: payment method to use
    resp_json: the JSON formatted response from the ledger
```
{
    "op": "REPLY",
    "protocolVersion": <int>    // (optional)  the version of the client/node communication protocol
    "result": {
        "type": "3",
        "identifier": <str>,    // the payment address
        "reqId": <int>,         // a random identifier
        
        "seqNo": 69,
        
        "data": {
        
        }
    }
}
{
    'op': 'REPLY', 
    'result': {
        'type': '3',
        'identifier': 'MSjKTWkPLtYoPEaTF1TUDb',
        'reqId': 1514311352551755,
       
        'seqNo': 9,

        'data': {
            'type': '1',
            'identifier': 'MSjKTWkPLtYoPEaTF1TUDb',
            'reqId': 1514311345476031,
            'signature': '4qDmMAGqjzr4nh7S3rzLX3V9iQYkHurrYvbibHSvQaKw3u3BouTdLwv6ZzzavAjS635kAqpj5kKG1ehixTUkzFjK',
            'signatures': None,
            
            'seqNo': 9,
            `txnTime': 1514311348,
            
            'rootHash': '5ecipNPSztrk6X77fYPdepzFRUvLdqBuSqv4M9Mcv2Vn',
            'auditPath': ['Cdsoz17SVqPodKpe6xmY2ZgJ9UcywFDZTRgWSAYM96iA', '3phchUcMsnKFk2eZmcySAWm2T5rnzZdEypW7A5SKi1Qt'],
            
            'alias': 'name',
            'dest': 'WTJ1xmQViyFb67WAuvPnJP',
            'role': '2',
            'verkey': '~HjhFpNnFJKyceyELpCz3b5'
        }
    }
}

```
Example resp_json from the ledger for a Transfer:
```
{
	"op":"REPLY",
	"result":{
		"reqId":1533834540773581038,
		"seqNo":19,
		"identifier":"V4SGRU86Z58d6TV7PBUe6f",
		"type":"3",
		"data":{
			"txn":{
				"protocolVersion":2,
				"metadata":{
					"digest":"1d9e749cfe8774d83dd2131811cd72fff2cc3707fb1e050ed2f42b579c11e2ac",
					"reqId":1989289532,
					"from":"HWWYHj6Lf92zEfikRBoVzxcmpsLyQf9Apue7Fbj3HHQ9"
				},
				"type":"10001",
				"data":{
					"outputs":[
						["cHKFYXNuaPtX9UcZfmq61mWvXA28rfxXh6s3iZU5ZytxmmzRM",10]
					],
					"inputs":[
						["2s2bzWYoxzDqtNBwb2ATxoNoKSF7DZSnypgXxvpGr8Br71AgDg",16]
					]
				}
			},
			"txnMetadata":{
				"seqNo":19,
				"txnTime":1533834535
			},
			"ver":"1",
			"auditPath":[
				"EAkV5bimQaWKNArEdNV5Dty3FXNNpdaNrGyd4u5qQURx",
				"Atgkhxn1JMTyrnCvzhqrxutR5yJCH9kyXqUXyw2sAf4b"
			],
			"rootHash":"7M995X8oWdmY1PEVMzuB12YZAiEEju9FFBd2s1dPzoiZ",
			"reqSignature":{
				"values":[{
					"from":"2s2bzWYoxzDqtNBwb2ATxoNoKSF7DZSnypgXxvpGr8Br71AgDg",
					"value":"3me9qt7xtup9tBxMQMGg415ACE4eLWWE4noh4QYLiK4gCRfE7HyA5ozGANKUYV44nAfimgmxVfSfQeZXoUhj63jE"
				}],
				"type":"ED25519"
			}
		}
	}
}

```
Example resp_json from the ledger for Fees:
```
{
	"op":"REPLY",
	"result":{
		"identifier":"V4SGRU86Z58d6TV7PBUe6f",
		"reqId":1533834704418243379,
		"type":"3",
		"data":{
			"ver":"1",
			"auditPath":[
				"7t3J9iZsH1ELjg6A7KbMpPCMF2gMWGxvoiXPLE3BXWm6",
				"HXLC5nmcNrtdWzMLXVBFcCi7vPfjES5C9GnoGRxv5FRT",
				"EuS2jriy1FoCweMk4ex7zocs4dJNw9JAkBKGLghoDPGk",
				"Atgkhxn1JMTyrnCvzhqrxutR5yJCH9kyXqUXyw2sAf4b"
			],
			"reqSignature":{
				"values":[{
					"value":"3zD3NNMUMK9J1Y2TfBypMxcot3mKER5g7jKSYZFmMebHyBhqpd44XHbrQL6WNGTQujaftjpErTuW4dBpxTJAkpsh",
					"from":"22qZRFLTfCmvFa2zKxAhzbReCcxV6f48Ux5XwT5poviW2RGesK"
				}],
				"type":"ED25519"
			},
			"txn":{
				"data":{
					"ref":"1:74",
					"fees":1,
					"outputs":[
						["22qZRFLTfCmvFa2zKxAhzbReCcxV6f48Ux5XwT5poviW2RGesK",9]
					],
					"inputs":[
						["22qZRFLTfCmvFa2zKxAhzbReCcxV6f48Ux5XwT5poviW2RGesK",21]
					]
				},
				"type":null,
				"metadata":{
					"digest":"99c84be708ab7e03bab13af038027ffffe10ccc03ac4fdd1f7af1ef2f42e340a",
					"reqId":1533834699526507519
				},
				"protocolVersion":2
			},
			"txnMetadata":{
				"txnTime":1533834700,
				"seqNo":24
			},
			"rootHash":"9bY7LVmWcrmzdvs3yJzNf7Apx6dX4YTLTxmLBYQKjNsG"
		},
		"seqNo":24
	}
}

```
Example resp_json from the ledger for Mint:
```
{	
	"op":"REPLY",
	"result":{
		"reqId":1533834288055293486,
		"seqNo":8,
		"identifier":"V4SGRU86Z58d6TV7PBUe6f",
		"type":"3",
		"data":{
			"txn":{
				"protocolVersion":2,
				"metadata":{
					"digest":"6f83fb082eb50742c574b96f34b81850e8f7d8107ca887598cae3a96a5be0d9a",
					"reqId":3795564132,
					"from":"V4SGRU86Z58d6TV7PBUe6f"
				},
				"type":"10000",
				"data":{
					"outputs":[
						["2s4Ldd2Hr1adbmAmxFcsb7B6HqyY4bADaGW78b1CXbwPECk9Xo",10]
					]
				}
			},
			"txnMetadata":{
				"seqNo":8,
				"txnTime":1533834244
			},
			"ver":"1",
			"auditPath":[
				"3M1ukcEDUoaajGATWn6XUANwqUjgM4fv9XdCcRFuj9wu",
				"AR3eZNQccupXhLUZvx54xrvXf7LKrZPXjHMwt1gWFHNw",
				"3XH6Ep6rWcUgaE8mYHVBbXVw7QqKfXZ8j6ykowXDWWP8"
			],
			"rootHash":"2E1k4omwJEPVX4rXT59ybJZWoMHBuoTA9kC5nvLPPQnv",
			"reqSignature":{
				"values":[{
					"from":"V4SGRU86Z58d6TV7PBUe6f",
					"value":"2kT9SgpEwEUfLaMMf5tbsw1kLYU7kz1ix2MUo8w2avV5Bwz4VirfCKy9tGGZbymZokgTrSvAxmXnj7uyVE6zLRfi"
				}],
				"type":"ED25519"
			}
		}
	}
}

```
### return:
    txn_json - parsed (payment method and node version agnostic) transaction info as json:
```
{
    "sources": [<source str>, ],
    "receipts":
    [
        {
            "recipient": <str>,     // full sovrin payment address: "pay:sov:<address><checksum>"
            "receipt": <str>,        // receipt string: "rec:sov:<base58 encoding of: {"address": <str:address, "seqNo": <int>}>
            "amount": <int>,        // amount of tokens in this input
        },
    ],
    "extra": <str>          // optional data from payment transaction
]
```
Example sources_json:
```
{
	"sources":[
		"src:sov:E1zP66C1U8bKVqA5pz6JQUvhrTM7GqBPJtjJvcRt6ymuRXNnFXruPVSxArfDeBVMLpRFo2g84tGXdopVwS5HWw8TMJ7vHAZFuYC18BHJaXn6bTHj2X9KAJ1"
	],
	"receipts":[{
		"recipient":"pay:sov:cHKFYXNuaPtX9UcZfmq61mWvXA28rfxXh6s3iZU5ZytxmmzRM",
		"receipt":"rec:sov:3x42qH8UkJac1BuorqjSEvuVjvYkSKtBqeNdKzNr5ZFpMjFbybaqVsZ8SfTnZhY9fiAGD9TXEcQ9DaTB4i69KX2m1x3vCBVG2jAn8NFQrtF87YXtw4nETY",
		"amount":10,
	}],
	"extra":null
}
```