
# UTOX data structures

## method: build_get_utxo_request_handler
### inputs: 

    submitter_did
    payment_address
    
### return:

    json
    {
    'identifier': '6ouriXMZkLeHsuXrN1X1fd', 
    'operation': 
     {
      'address': '2jyMWLv8NuxUV4yDc46mLQMn9WUUzeKURX3d2yQqgoLqEQC2sf', 
      'type': '10002'
     }, 
    'reqId': 6284, 
    'protocolVersion': 1
    }
    
    
    
    
## method: parse_get_utxo_response_handler
### inputs:
    resp_json
    
### return:
    utxo_json
