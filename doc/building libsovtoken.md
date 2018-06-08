# Building libsovtoken
This document exists for multiple purposes:
* To assist developers in building libsovtoken

## Requirements

### Indy-SDK
You need to do one of the following.
1) have indy-sdk installed or
2) build indy-sdk from source https://github.com/hyperledger/indy-sdk.git

### Environment variables
create an environment variable LIBINDY_DIR.   Have it point the directory containing indy-sdk binaries.

eg:  assuming you built indy-sdk from source, you might set the environment variable as such
LIBINDY_DIR="/Users/my.home/src/hyperledger/indy-sdk/master/libindy/target/debug"


## building libsovtoken
1) get latest from master at https://github.com/evernym/libsovtoken.git
2) the source code is in a sub-directory called libsovtoken
3) run the following commands
   cargo update
   cargo build
4) run the tests
   cargo test

Please keep in mind, at the time this document was written, some of the cargo crates are github repositories.
If you update libsovtoken code from github you need to run build command as follows
   cargo clean
   cargo update
   cargo build

The cargo update command is necessary to make sure you get latest from the cargo crates in github






