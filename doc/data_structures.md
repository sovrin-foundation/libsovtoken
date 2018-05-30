
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
            {"address" : "pay:sov:217PRj6piK5G77AcxS9GAsk1FCfaZn54bYiHzmabyzKtBrWnUk", "seqno": 2, "amount": 10 }, 
            {"address" : "pay:sov:2RKhX72u617CffvqrontubPRh7zBKqcgmT8reDEeBLemFLgYkr", "seqno": 5, "amount": 15 },
            {"address" : "pay:sov:2bVtdDaPET8u4dUVRbpiaK3honHYvpVGRudFdzkpe3VyMMWPmX", "seqno": 14, "amount": 5 },
        ]
    }
```    
   
    outputs_json
```json
    {
    "ver": 1,
    "destinationAddresses": 
        [
            {"address" : "pay:sov:2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", "amount": 27 },
            {"address" : "pay:sov:DhzPC0E3WNMdxP6PcdspMJpjwNI1tk4jKlXYNJNXzFNcpJce", "amount": 3 }, 
        ]
    }
```    
    
### return:

    payment_req_json
    note: output to ledger excludes address prefix "pay:sov"
    note: any difference between the sum of the inputs and the sum of outputs is the fees amount
``` json
        {
            "identifier": "F8s1tgmNmHpMq3noQTVNqP6axfE9ATD8s63KSyzThixT",
            "operation": {
                "inputs": [
                    ["217PRj6piK5G77AcxS9GAsk1FCfaZn54bYiHzmabyzKtBrWnUk", 2, "3uhXG9gMQ5KeUCd3P4udoFuhZAy4fLCGzSjKNXtYiSp1tjYoY48Tq4EhrPmnqff7TebVFU8zqVpab7CQnNxD7NdT"],
                    ["2RKhX72u617CffvqrontubPRh7zBKqcgmT8reDEeBLemFLgYkr", 5, "3mPYwTTZ2fpc3F7XacoPyXW8CgCB64k8HkZ68Tqz7xDv3UNrCmiE4EkKysgE3ACZWWFA3wGmywoeBBTJzX3QVps6"],
                    ["2bVtdDaPET8u4dUVRbpiaK3honHYvpVGRudFdzkpe3VyMMWPmX", 14, "3RQbmFyKsR5VS1GswdXJ7eAvik1dw2tDZMMts2Fg7yws8oYcw521nDRDD9SoerQXzE5DjxEj6mfo5yrjem2r7d9F"]
                ],
                "type": "10001",
                "extra": null,
                "outputs": [
                    ["2jS4PHWQJKcawRxdW6GVsjnZBa1ecGdCssn7KhWYJZGTXgL7Es", 27],
                    ["DhzPC0E3WNMdxP6PcdspMJpjwNI1tk4jKlXYNJNXzFNcpJce", 3 ]
                ]
            },
            "reqId": 1527711037434862
        }
```     
    
## method: parse_payment_response_handler
### inputs:
    resp_json
```json 
        {
            "op": "REPLY",
            "result": {
                "auditPath": ["Ca86JVfFnnRiKdsUCwkHjKXUhMcRCcuD44GoW8ChdJZU", "B9aBg94HJD68k3FS7Xik5VfwCA3vBXDSthr6eXhZDStG"],
                "signatures": null,
                "reqId": 1527712589780601,
                "extra": null,
                "seqNo": 4,
                "inputs": [
                    ["2i83FoT5vLeSqdnUrmV7n6dJkqwNxA6Dmgesx5c71Fjza2T1nC", 3, "3RqpRBNrNEDjdH6SPEtHBz1SjzeySGCZRdCX5z5Vwc4DmCDkVgxAvc2jnZjkHwNJqbxKFT7cfbkkBfAbooGRwZMr"],
                    ["knD8ACByNXftEbfsihNrJUQWcy31Wh1Bjk55iJdZcpAPid9oL", 3, "PV5Pt1aep3ejrcFBq4VkfYcuJCkWNfSfC3zMckUPVpbKXAhQApH8rrxaChzbhdDXVXdiGjz1S1gkiUfbnjsqVux"],
                    ["24q9X14ShgeUPmzQwtCDnfjt7jD8zNVtFZkbZECGsWpCkiCfVb", 3, "2a9gbMxZiV7CNacEmrvj4W36aeQC7XCHxzAcPrAsX7cmJMRnsNA4RmeRNW8Rwy2qs8GRcUBaFbmdAKpqsTLzaEYM"]
                ],
                "signature": null,
                "identifier": "AT32EYyf4WTqbNfkGofQ7vFrXcbL24DoCZQK4WHWjMZM",
                "rootHash": "ELouWgpqJrT6ENreQo6afXx9aBtuhnWtDKyriaM4fEgb",
                "txnTime": 1527712991,
                "outputs": [
                    ["iu4wAP3TycMGCEh6tudajEwwSYspP9kBcgAkbBwqLQAxoyKHt", 17],
                    ["2e6yD9dWwCbgMMdc59ZK5ikoZJFoLA2eBLfez65Next4vBW2pm", 7],
                    ["24q9X14ShgeUPmzQwtCDnfjt7jD8zNVtFZkbZECGsWpCkiCfVb", 15]
                ],
                "type": "10001"
            }
        }
```
    
### return:
    json
