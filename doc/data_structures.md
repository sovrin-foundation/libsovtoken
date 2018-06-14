
# LibSovToken Data Structures
This document exists for multiple purposes:
* To assist the LibSovToken team and give them the information they need to write the payment handler methods
* To document the inputs and outputs of exposed functions for users of Indy-SDK when payments are handled by LibSovToken.

## method: indy_create_payment_address
This API call is handled by LibSovToken create_payment_address_handler

### inputs:

    command_handle: command handle to map callback to context
    wallet_handle: wallet handle where to save new address
    payment_method: Payment method to use (for example, "sov")
    config: payment address config as json:
```
{
    seed: <str>, // allows deterministic creation of payment address
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
{
    "ver": <int>,   // this field is included to allow for future backward compatability
    "inputs_json":    // Each txo string is of the format: "{"address": "pay:sov:<address>", "seqNo": <int>}"
        [<str: txo_string>, ]
}
```
    outputs_json: The list of UTXO outputs as json array:
```
{
    "ver": <int>,   // this field is included to allow for future backward compatability
    "outputs_json": [
        {
            "address" : <str>   // the payment address
            "amount": <int>,    // the payment amount
            "extra": <str>      // optional field
        },
    ]
}
```
Example inputs_json:
```
{
    "ver": 1,
    "inputs_json": [
        {"address": "pay:sov:QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH", "seqNo": 2}
    ]
}
```
Example outputs_json:
```
{
    "ver": 1,
    "outputs_json": [
        {
            "address": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
            "amount": 11,
            "extra": ""
        },
    ]
}
```

### return:
    req_with_fees_json: modified Indy request with added fees info
```
{
    <req_json>    //initial transaction request
    "fees": [
        [
            [<str: source payment address>, <int: sequence number>, <str: signature over source payment address, sequence number, and all outputs>],
        ],
        [
            [<str: change payment address>, <int: amount of change>],
        ]
    ],
}
```
Example req_with_fees_json:
```
{
    <req_json>    //initial transaction request
    "fees": [
        [
            ["QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH", 2, "5Z7ktpfVQAhj2gMFR8L6JnG7fQQJzqWwqrDgXQP1CYf2vrjKPe2a27borFVuAcQh2AttoejgAoTzJ36wfyKxu5ox"]
        ],
        [
            ["2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC", 11]
        ]
    ],
}
```

## method: indy_parse_response_with_fees
This API call is handled by LibSovToken parse_response_with_fees_handler.

### inputs:
    command_handle
    payment_method
    resp_json: the JSON formatted response from the ledger
```
{
    "op" : <String>, // //type of operation returned
    "ver": <int>, // the version of the transaction response data structure
    "txn": {
        "type": <String>, //the transaction type based upon the request submitted
        "protocolVersion": <int>, // the version of the transaction data structure
        "data": {
            "ver": <int>, //version of payload data structure
            "dest":<String>, // 
            "verkey":<String>, // verification key of the DID associated with the transaction
            "role":<int>, //role of DID associated with the transaction
        },
        “Fees”: {
            'inputs':[
                        [<String>, <int>], // [first payment address that sent payment, seqNo used to send payment], 
                        [<String>, <int>], // [second payment address that sent payment, seqNo used to send payment], 
                        [<String>, <int>], // [third payment address that sent payment, seqNo used to send payment]
                     ],
            'outputs':[
                        [<String>, <int>], // [first payment address that received payment, amount being received],
                        [<String>, <int>], // [second payment address that received payment, amount being received],
                        [<String>, <int>], // [third payment address that received payment, amount being received]
                     ],
        },
        "metadata": {
            "reqId":<int>,  // random nonce submitted used by the nodes to track the request
            "from": <String>, // the did associated with the submitted transaction
        }
    },
    "txnMetadata": {
        "txnTime": <int>, // time of request creation in UNIX epoch time format
        "seqNo": <int>, // the seqNo of the transaction (on which ledger? payment ledger or Domain ledger)
        "txnId": <String> // Txn ID as State Trie key (address or descriptive data). It must be unique within the ledger. (again which ledger?)
    },
     "reqSignature": {
        "type": <string> //the type of key used for signatures,
        "values": [
            {
                "from": <str>, // first payment address that sent payment
                "Value": <integer>, // signature of inputs[0] and 
            },
            {
                “from” : <payment address with type str from inputs[1]>,
                “Value” :  <associated signature of inputs[1] and all outputs>
            },
            { 
                “from” : <payment address with type str from inputs[2]>,
                “Value” :  <associated signature of inputs[2] and all outputs>
            }
        ]
    }
    'rootHash': <String>, // the root hash used to validate that the transaction exists on the ledger
    'auditPath': [<String>, <String>], //a list of hashes used to verify that the transaction exists on the ledger
}
```
Example resp_with_fees_json:
```
{
    "op" : "REPLY",
    "ver": 2,
    "txn": {
        "type":1,
        "protocolVersion":2,
        "data": {
            "ver": 1,
            "dest":"GEzcdDLhCpGCYRHW82kjHd",
            "verkey":"~HmUWn928bnFT6Ephf65YXv",
            "role": None,
        },
        “Fees”: {
            'inputs':[
                        ['2i83FoT5vLeSqdnUrmV7n6dJkqwNxA6Dmgesx5c71Fjza2T1nC', 3], 
                        ['knD8ACByNXftEbfsihNrJUQWcy31Wh1Bjk55iJdZcpAPid9oL', 3], 
                        ['24q9X14ShgeUPmzQwtCDnfjt7jD8zNVtFZkbZECGsWpCkiCfVb', 3]
                     ],
            'outputs':[
                        ['iu4wAP3TycMGCEh6tudajEwwSYspP9kBcgAkbBwqLQAxoyKHt', 17],
                        ['2e6yD9dWwCbgMMdc59ZK5ikoZJFoLA2eBLfez65Next4vBW2pm', 7],
                        ['24q9X14ShgeUPmzQwtCDnfjt7jD8zNVtFZkbZECGsWpCkiCfVb', 15]
                     ],
        },
        "metadata": {
            "reqId":1513945121191691,
            "from":"L5AD5g65TDQr1PPHHRoiGf",
        }
    },
    "txnMetadata": {
        "txnTime":1513945121,
        "seqNo": 10,
        "txnId": "N22KY2Dyvmuu2PyyqSFKue|01"
    },
    "reqSignature": {
        "type": "ED25519",
        "values": [
	        {
                "from": "2i83FoT5vLeSqdnUrmV7n6dJkqwNxA6Dmgesx5c71Fjza2T1nC",
                "Value": "3RqpRBNrNEDjdH6SPEtHBz1SjzeySGCZRdCX5z5Vwc4DmCDkVgxAvc2jnZjkHwNJqbxKFT7cfbkkBfAbooGRwZMr"
            },
            {
                “from” : 'knD8ACByNXftEbfsihNrJUQWcy31Wh1Bjk55iJdZcpAPid9oL',
                “Value” :  'PV5Pt1aep3ejrcFBq4VkfYcuJCkWNfSfC3zMckUPVpbKXAhQApH8rrxaChzbhdDXVXdiGjz1S1gkiUfbnjsqVux'
            }, 
                “from” : '24q9X14ShgeUPmzQwtCDnfjt7jD8zNVtFZkbZECGsWpCkiCfVb',
                “Value” :  '2a9gbMxZiV7CNacEmrvj4W36aeQC7XCHxzAcPrAsX7cmJMRnsNA4RmeRNW8Rwy2qs8GRcUBaFbmdAKpqsTLzaEYM'
            }
        ]
    'rootHash': '5ecipNPSztrk6X77fYPdepzFRUvLdqBuSqv4M9Mcv2Vn',
    'auditPath': ['Cdsoz17SVqPodKpe6xmY2ZgJ9UcywFDZTRgWSAYM96iA', '3phchUcMsnKFk2eZmcySAWm2T5rnzZdEypW7A5SKi1Qt'],
}
```

### return:
    utxo_json - parsed utxo info as json:
```
{
    "ver": <int>,                    // this field is included to allow for future backward compatability
    "utxo_json":
        [{
            "paymentAddress": <str>,// sovrin payment address: "pay:sov:<address><checksum>"
            "txo": <str>,           // txo string: "{"address" : "pay:sov:<address>", "seqNo": <int>}"
            "amount": <int>,        // amount of tokens in this input
            "extra": <str>          // optional data from payment transaction
        }]
```
Example utxo_json:
```
    "ver": 1,
    "utxo_json":
        [{
            "paymentAddress": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
            "txo": {"address": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC", "seqNo": 3},
            "amount": 11,
            "extra":
        }]
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
    "protocolVersion": <int>    // the version of the client/node communication protocol
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
This API call is handled by LibSovToken parse_get_utxo_response_handler
### inputs:
    resp_json: the JSON formatted response from the ledger
```
{
    "op": "REPLY",
    "protocolVersion": <int>    // the version of the client/node communication protocol
    "result": {
        "type": "10002",
        "address": <str>,       // the payment address
        "identifier": <str>,    // the payment address
        "reqId": <int>,         // a random identifier
        "outputs": [
            ["<str: address>", <int: sequence number>, <int: amount>],
        ],
    }
}

```
Example resp_json from the ledger:
```
{
    "op": "REPLY",
    "protocolVersion": 1,
    "result": {
        "type": "10002",
        "address": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
        "identifier": "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
        "reqId": 23887,
        "outputs": [
            ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 2, 10],
            ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 3, 3]
        ],
    }
}

```
    
    
### return:
    utxo_json - parsed utxo info as json:
```
{
    "ver": <int>,                    // this field is included to allow for future backward compatability
    "utxo_json":
        [{
            "paymentAddress": <str>,// full sovrin payment address: "pay:sov:<address><checksum>"
            "txo": <str>,           // txo string: "{"address": "pay:sov:<address>", "seqNo": <int>}"
            "amount": <int>,        // amount of tokens in this input
            "extra": <str>          // optional data from payment transaction
        }]
```
Example utxo_json:
```
    {
    "ver": 1,
    "utxo_json":[
        {
            "paymentAddress": "pay:sov:2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es",
            "txo": {"address": "pay:sov:2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", "seqNo": 2},
            "amount": 10,   // amount of tokens in this input
            "extra":        // optional data from payment transaction
        },
        {
            "paymentAddress": "pay:sov:2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es"
            "txo": "{"address": "pay:sov:2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", "seqNo": 3}",
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
```
{
    "ver": <int>,   // this field is included to allow for future backward compatability
    "inputs_json":  // Each txo string is of the format: "{"address": "pay:sov:<address>", "seqNo": <int>}"
        [<str: txo_string>, ]
}
```
    outputs_json: The list of UTXO outputs as json array:
```
{
    "ver": <int>,   // this field is included to allow for future backward compatability
    "outputs_json": [
        {
            "address": <str>   // full sovrin payment address: "pay:sov:<address><checksum>"
            "amount": <int>,    // the payment amount
            "extra": <str>      // optional field
        },
    ]
}
```
Example inputs_json:
```
    {
    "ver": 1,
    "inputs_json":
        [
            {"address": "pay:sov:QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH", "seqNo": 2 },
            {"address": "pay:sov:t3gQdtHYZaEHTL92j81QEpv5aUHmHKPGQwjEud6mbyhuwvTjV", "seqNo": 5 },
            {"address": "pay:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", "seqNo": 14 },
        ]
    }
```
Example outputs_json:
```
    {
    "ver": 1,
    "outputs_json":
        [
            {
                "address": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
                "amount": 11,
                "extra": ""
            },
            {
                "address": "pay:sov:2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h",
                "amount": 19
                "extra": ""
            },
            {
                "address": "pay:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy",
                "amount": 9
                "extra": ""
            },
        ]
    }
```


    
### return:

    payment_req_json
    note: any difference between the sum of the inputs and the sum of outputs is the fees amount
```
{
    'op': 'REPLY', //type of operation returned
    "ver": <int>, // the version of the transaction response data structure
    "txn": { // the payload containing the specific transaction data relevant to the request
        "type": <String>, //the transaction type based upon the request submitted
        "protocolVersion": <int>, // the version of the transaction data structure
        "data": {
            "ver": <int>, //the version number of the payload data structure
            'inputs':[
                        [<String>, <int>], // [first payment address that sent payment, seqNo used to send payment], 
                        [<String>, <int>], // [second payment address that sent payment, seqNo used to send payment], 
                        [<String>, <int>], // [third payment address that sent payment, seqNo used to send payment]
                        ],
            'outputs': [
                        [<String>, <int>], // [first payment address that received payment, amount being received] ,
                        [<String>, <int>], // [second payment address that received payment, amount being received],
                        [<String>, <int>], // [third payment address that received payment, amount being received]
                    ],
        },
        "metadata": {
            "reqId": <int>, // random nonce submitted used by the nodes to track the request
            "from": <String> // first payment address in inputs - This should become optional to prevent correlation before version 2 becomes official
        },
    },
    "txnMetadata": {
        "creationTime": <int>, // time of request creation in UNIX epoch time format
        "seqNo": <int>, // the seq_no of the transaction on the payment ledger  
    },
    "reqSignature": {
        "type": <string> //the type of key used for signatures,
        "values": [
            {
                "from": <str>, // first payment address that sent payment
                "Value": <integer>, // signature of inputs[0] and 
            },
            {
                “from” : <payment address with type str from inputs[1]>,
                “Value” :  <associated signature of inputs[1] and all outputs>
            },
            { 
                “from” : <payment address with type str from inputs[2]>,
                “Value” :  <associated signature of inputs[2] and all outputs>
            }
        ]
    }
    'rootHash': <String>, // the root hash used to validate that the transaction exists on the ledger
    'auditPath': [<String>, <String>], //a list of hashes used to verify that the transaction exists on the ledger
}
```

Example payment_req_json:
    note: output to ledger excludes address prefix "pay:sov"
    note: any difference between the sum of the inputs and the sum of outputs is the fees amount
```
{
    'op': 'REPLY', 
    "ver": 2,
    "txn": {
            "type": 10001,
            "protocolVersion": 2,
            "data": {
             	"ver": 2,
             	'inputs':[
                            ['2i83FoT5vLeSqdnUrmV7n6dJkqwNxA6Dmgesx5c71Fjza2T1nC', 3], 
                            ['knD8ACByNXftEbfsihNrJUQWcy31Wh1Bjk55iJdZcpAPid9oL', 3], 
                            ['24q9X14ShgeUPmzQwtCDnfjt7jD8zNVtFZkbZECGsWpCkiCfVb', 3]
                         ],
               'outputs': [
                            ['iu4wAP3TycMGCEh6tudajEwwSYspP9kBcgAkbBwqLQAxoyKHt', 17],
                            ['2e6yD9dWwCbgMMdc59ZK5ikoZJFoLA2eBLfez65Next4vBW2pm', 7],
                            ['24q9X14ShgeUPmzQwtCDnfjt7jD8zNVtFZkbZECGsWpCkiCfVb', 15]
                       ], 
            },   
            "metadata": {
                "reqId": 1527712589780601,
                "from": '2i83FoT5vLeSqdnUrmV7n6dJkqwNxA6Dmgesx5c71Fjza2T1nC'
            }
    },
    "txnMetadata": {
        "creationTime": 1527712991,
        "seqNo": 4,  
    },
    "reqSignature": {
        "type": "ED25519",
        "values": [
            {
                "from": "2i83FoT5vLeSqdnUrmV7n6dJkqwNxA6Dmgesx5c71Fjza2T1nC",
                "Value": "3RqpRBNrNEDjdH6SPEtHBz1SjzeySGCZRdCX5z5Vwc4DmCDkVgxAvc2jnZjkHwNJqbxKFT7cfbkkBfAbooGRwZMr"
            },
            {
                “from” : 'knD8ACByNXftEbfsihNrJUQWcy31Wh1Bjk55iJdZcpAPid9oL',
                “Value” :  'PV5Pt1aep3ejrcFBq4VkfYcuJCkWNfSfC3zMckUPVpbKXAhQApH8rrxaChzbhdDXVXdiGjz1S1gkiUfbnjsqVux'
            },
            { 
                “from” : '24q9X14ShgeUPmzQwtCDnfjt7jD8zNVtFZkbZECGsWpCkiCfVb',
                “Value” :  '2a9gbMxZiV7CNacEmrvj4W36aeQC7XCHxzAcPrAsX7cmJMRnsNA4RmeRNW8Rwy2qs8GRcUBaFbmdAKpqsTLzaEYM'
            }
        ]
    }
    'rootHash': '5ecipNPSztrk6X77fYPdepzFRUvLdqBuSqv4M9Mcv2Vn',
    'auditPath': ['Cdsoz17SVqPodKpe6xmY2ZgJ9UcywFDZTRgWSAYM96iA', '3phchUcMsnKFk2eZmcySAWm2T5rnzZdEypW7A5SKi1Qt'],
}
```
    
## method: indy_parse_payment_response
This API call is handled by LibSovToken parse_payment_response_handler. *Note: this transaction format will not change because it's a read request and not a write request.*
### inputs:
    resp_json: This is an example of the JSON that will be returned from the ledger after submitting a payment request.
```
    {
        "op": "REPLY",
        "result": {
            "identifier": <str>,        // the first input payment address
            "type": "10001",
            "seqNo": <int>,             // the sequence number of the transaction
            "txnTime": <int>,           // the posix time the transaction was written to the ledger
            "signature": <str>,         // not used in this transaction
            "signatures": <str>,        // not used in this transaction
            "extra": <str>,             // optional field
            "reqId": <int>,             // a random identifier
            "inputs": [
                [<str: source payment address>, <int: sequence number>, <int: signature over source payment address, sequence number, and all outputs>],
            ],
            "outputs": [
                [<str: change payment address>, <int: amount of change>],
            ]
            "rootHash": <str>,          // the root hash of the transaction
            "auditPath": [
                <str: hash>,            // the hash of each node in the path
            ]
        }
    }
```
Example resp_json:
```
{
    "op": "REPLY",
    "result": {
        "identifier": "QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH",
        "type": "10001",
        "seqNo": 4,
        "txnTime": 1527714130,
        "signature": null,
        "signatures": null,
        "extra": null,
        "reqId": 1527714086374556,
        "inputs": [
            ["QEb3MVVWv1McB8YpgXAvj8SbZDLRRHaPpWt9jFMgfRss3CYBH", 3, "3TMn17XTUd7Qr93hiuBWJFyihZ7aQSDbZTwqJEepUFQ5NRoCYYA2ARih2eQLNUZcB2wDSeQaxRFXhrcW2a5RyXrx"],
            ["t3gQdtHYZaEHTL92j81QEpv5aUHmHKPGQwjEud6mbyhuwvTjV", 3, "4hPYHU1gBnC3ViQEyWf4zz3UPSrT364BfgP5YupBFv6HiuTh7JNLKKDLiiuwxHDHRd4o8AQwGVTT7nJHNTVq8NZy"],
            ["2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", 3, "2VvANwBDYNcHyyheGSHx2og7Pc31hw5Box74xZ1EYrm6HijeKqAnKGX6dHF8gL6x78vWUgTpHRA5V41YB7EJMcKq"]
        ],
        "outputs": [
            ["2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC", 11],
            ["2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h", 19],
            ["2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", 9]
        ],
        "rootHash": "FRkqRd5jyNRK3SGSGNoR6xMmYQvLVnotGLGWYxR1dCN4",
        "auditPath": [
            "6QFFFVbio2q8viWBbuVfvQsv3Qgd3Ub64Qv41i5wH8Bo", "8vDzQmeYb8ecQ7Nyv5i6V8nUwT3fsebqTHMXqgzYi1NU"
        ]
    }
}
```
    
### return:
    utxo_json: parsed utxo info as json
```
{
    "ver": <int>,                    // this field is included to allow for future backward compatability
    "utxo_json": [
        {
            "paymentAddress": <str>,// full sovrin payment address: "pay:sov:<address><checksum>"
            "txo": <str>,           // txo string: "{"address": "pay:sov:<address>", "seqNo": <int>}"
            "amount": <int>,        // amount of tokens in this input
            "extra": <str>          // optional data from payment transaction
        },
    ]
}

```
Example utxo_json:
```
    {
        "ver" : 1,
        "utxo_json": [
            {
                "paymentAddress": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC",
                "txo": {"address": "pay:sov:2mVXsXyVADzSDw88RAojPpdgxLPQyC1oJUqkrLeU5AdfEq2PmC", "seqNo" : 4},
                "amount": 11,
                "extra": ""
            },
            {
                "paymentAddress": "pay:sov:2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h",
                "txo": {"address": "pay:sov:2k7K2zwNTF7pouG3yHqnK2LvVWVj1FdVEUSTkdwtoWYxeULu8h", "seqNo" : 4},
                "amount": 19,
                "extra": ""
            },
            {
                "paymentAddress": "pay:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy",
                "txo": {"address": "pay:sov:2SBZcBgBHzU1d9u7jxggsbNJDa5zKZRqa3v13V5oR6eZgTmVMy", "seqNo" : 4},
                "amount": 9,
                "extra": ""
            }
        ]
    }
```


## method: indy_build_mint_req
This API call is handled by LibSovToken build_mint_txn_handlerr

### inputs:
    wallet_handle: wallet handle
    submitter_did : DID of request sender
    outputs_json: The list of UTXO outputs as json array:
```
    [
        [<str: output payment address>, <int: amount to mint>],
    ]
```
Example outputs_json:
```
    [
        ["sjw1ceG7wtym3VcnyaYtf1xo37gCUQHDR5VWcKWNPLRZ1X8eC", 60],
        ["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 40]
    ]
```

### return:
    payment_method
    mint_req_json: Indy request for minting tokens
```
{
    "reqId": <int>,             // a random identifier
    "protocolVersion": <int>,   // the version of the client/node communication protocol
    "operation": {
        "type": "10000",
        "outputs": [
            [<str: output payment address>, <int: amount to mint>],
        ]
    },
    "signatures" {
        <str: Trustee DID>: <str: Trustee Signature over operation>,
    }
}
```
Example mint_req_json:
```
{
    "reqId": 1527799618700635,
    "protocolVersion": 1,
    "operation": {
        "type": "10000",
        "outputs": [
            ["sjw1ceG7wtym3VcnyaYtf1xo37gCUQHDR5VWcKWNPLRZ1X8eC", 60],
            ["dctKSXBbv2My3TGGUgTFjkxu1A9JM3Sscd5FydY4dkxnfwA7q", 40]
        ]
    },
    "signatures": {
        "E7QRhdcnhAwA6E46k9EtZo": "j7kFGUmdmCjfuDFxotwKUZTCZ6veExaZTsqwxnTi2R6EsabUFQPR2VaAhaCKpR6bqHns2d2LUqG4czAkb1fNab3",
        "CA4bVFDU4GLbX8xZju811o": "2KmN6kGKFCb9gDiCMvC6P2uXdFC95dHXsY2BYnetiasuq837zRiyVvLDyR8ud2dzXtaKvxFw7Jb6YWEzm4JWXnDS",
        "B8fV7naUqLATYocqu7yZ8W": "4AwJ7pBJXUBeCDXQ7tveFCd96fhYhXUysLWYc6TWp9MK2ovCMgCienpZwkMsLX3p6u5pd2oHN3WuLhbJtU6BEcr2",
        "M9BJDuS24bqbJNvBRsoGg3": "5j2DYSL8aa442pAKaaFZAhkUCdYX6UgioaLqGLXShMubgEX1EZhAmuPTnkgP7K36hRPXjTjSSYaWBJHQH48qqv55"
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
    "signatures": {
        <str: Trustee DID>: <str: Trustee Signature over operation>,
    }
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
    },
    "signatures": {
        "CA4bVFDU4GLbX8xZju811o": "67p5SSwPAKg26WJGCNWr5vHVA5U9eiWfntLjViurm4z57qnUU9Hbo3K8SZT3Q6NKFPk2RC3BBBPhcggFjkFuwL69",
        "B8fV7naUqLATYocqu7yZ8W": "dydGPoozPnbKRKVkSwidYNCrDN6FtswGoS9roMRaALtjDC49q1DZGSKKUyoLbd1jcn3sVEpCk9rZFpEMMCMGNMF",
        "E7QRhdcnhAwA6E46k9EtZo": "2D2TFByP4b9pj9uzibSwAPCVchgRwFanAk82k1S25XaXHit7sLbwdyPxEN1AzkQU3qUNBx1ndr69La4QuAU6K1tx",
        "M9BJDuS24bqbJNvBRsoGg3": "5Mn8D8JBSg7pA3dpRsC2e7Zi1XskMkrurJaShF3ziFv4tM3s32dvrhe9WKz59wGRKQPGeRP1NAngZuBEGdBVgC9E"
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
This API call is handled by LibSovToken parse_get_txn_fees_response_handler. *Note: this transaction format will not change because it's a read request and not a write request.*

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
    "result": {
        "identifier": "6ouriXMZkLeHsuXrN1X1fd",
        "reqId": 47660,
        "type": "20001",
        "fees": {
            "10001": 8,
            "1": 4
        },
        "state_proof": {
            "root_hash": "5BU5Rc3sRtTJB6tVprGiTSqiRaa9o6ei11MjH4Vu16ms",
            "proof_nodes": "29qFIGZlZXOT0pF7IjEiOjQsIjEwMDAxIjo4fQ=="
        }
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
