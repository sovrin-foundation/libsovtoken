# Changelog
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
