
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
        "address": "<pay:sov:{address}{checksum}>",
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
``` json
    {
    "ver": 1,
    "utxos": 
        [
            {"address" : "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", "seqno": 2, "amount": 10 }, 
            {"address" : "2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", "seqno": 3, "amount": 5 },
        ]
    }
```


## build_payment_req_handler
### inputs:
    wallet_handle
    submitter_did
    inputs_json
```json
{
    "identifier": "F8s1tgmNmHpMq3noQTVNqP6axfE9ATD8s63KSyzThixT",
    "operation": {
        "inputs": [
            ["217PRj6piK5G77AcxS9GAsk1FCfaZn54bYiHzmabyzKtBrWnUk", 4, "3uhXG9gMQ5KeUCd3P4udoFuhZAy4fLCGzSjKNXtYiSp1tjYoY48Tq4EhrPmnqff7TebVFU8zqVpab7CQnNxD7NdT"],
            ["2RKhX72u617CffvqrontubPRh7zBKqcgmT8reDEeBLemFLgYkr", 4, "3mPYwTTZ2fpc3F7XacoPyXW8CgCB64k8HkZ68Tqz7xDv3UNrCmiE4EkKysgE3ACZWWFA3wGmywoeBBTJzX3QVps6"],
            ["2bVtdDaPET8u4dUVRbpiaK3honHYvpVGRudFdzkpe3VyMMWPmX", 4, "3RQbmFyKsR5VS1GswdXJ7eAvik1dw2tDZMMts2Fg7yws8oYcw521nDRDD9SoerQXzE5DjxEj6mfo5yrjem2r7d9F"]
        ],
        "type": "10001",
        "extra": None,
        "outputs": [
            ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 27]
        ]
    },
    "reqId": 1527711037434862
}
```    
    
    outputs_json
    
    
### return:

    payment_req_json
