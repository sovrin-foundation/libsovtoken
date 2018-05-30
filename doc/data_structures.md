
# UTOX data structures

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
