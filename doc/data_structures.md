
# UTOX data structures

## method: build_get_utxo_request_handler
### inputs: 

    submitter_did
    payment_address : <"pay:sov:{address}{checksum}">
    
### return:

    json
    {
    'identifier': '6ouriXMZkLeHsuXrN1X1fd', 
    'operation': 
     {
      'address': <"pay:sov:{address}{checksum}">, 
      'type': '10002'
     }, 
    'reqId': 6284, 
    'protocolVersion': 1
    }
    
    
    
## method: parse_get_utxo_response_handler
### inputs:
    version 1, it will change
    resp_json
    {
        'op': 'REPLY', 
        'protocolVersion': 1
        'result': {
            'type': '10002', 
            'address': <"pay:sov:{address}{checksum}">, 
            'identifier': '6ouriXMZkLeHsuXrN1X1fd', 
            'reqId': 23887, 
            'outputs': []
        }
    }
    
    
### return:
    utxo_json
