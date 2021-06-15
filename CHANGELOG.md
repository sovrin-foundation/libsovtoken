# Changelog

## 1.0.5
* bugfixes 

## 1.0.4
* bugfixes 

## 1.0.3
* bugfixes 

## 1.0.2
* bugfixes

## 1.0.1
* Updated `build_get_utxo_request_handler` function to accept an additional parameter `from` used for pagination.
* Updated `parse_get_utxo_response_handler` function to return an additional parameter `next` pointing to the next slice of payment sources.
* Added `sign_with_address_handler` function to sign a message with a payment address.
* Added `verify_with_address_handler` function to verify a signature with a payment address.
* bugfixes

## 0.10.0
* Updated `build_set_txn_fees_handler` function to accept any aliases.
* Updated `build_payment_req_handler` function to accept and to handle `Transaction Author Agreement` passed inside `extra_json`.
* bugfixes

## 0.9.7
* Updated logging initialization to use libindy pattern.
* bugfixes

## 0.9.3
* Changed CI/CD to new stable branch.
* CD added to the new stable branch with new stable release process
* Source code is now public Sovrin repository
* changed base-58 library dependency.
* changed rust-indy-sdk dependency to rust crate
* bugfixes

## 0.9.2
* Android and iOS builds in CD pipeline
* Changed transactions to use JSON objects rather than unlabeled arrays
* bugfixes

## 0.9.1
* Set fees accepts strings
* relies on stable versions of all artifacts
* optimization of size for ios build
* refactoring and code improvements
* bugfixes

## 0.9.0
* Added functionality
    * build_verify_req_handler
    * parse_verify_response_handler
* iOS build artifacts in CI/CD pipeline
* Increased System testing
* Android log support
* Bugfixes

## 0.8.0

* Follows new Indy-SDK Payments API
* Android and iOS builds
* A docker image that provides an easy-install build environment
* A CI/CD pipeline
    * that requires passing tests for merging
    * that requires signed commits
* Bugfixes

Note: This release requires the user to have libindy installed locally from [the github repository](https://github.com/hyperledger/indy-sdk). 


## 0.7.0
Initial release
