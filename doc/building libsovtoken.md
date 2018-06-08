# Building libsovtoken
This document exists for the purposes of:
* assisting developers in building libsovtoken

## Requirements

### Indy-SDK
You need to do one of the following.
1) have indy-sdk installed or
2) build indy-sdk from source https://github.com/hyperledger/indy-sdk.git

### Environment variables
create an environment variable LIBINDY_DIR.   Have it point the directory containing indy-sdk binaries.

eg:  assuming you built indy-sdk from source, you will set the environment variable as such
```LIBINDY_DIR="/Users/my.home/src/hyperledger/indy-sdk/master/libindy/target/debug"```


## Building libsovtoken
1) get the requirements above, including building the projects if you elect to use source code
2) get latest from master at https://github.com/evernym/libsovtoken.git
3) the source code is in a sub-directory called libsovtoken
4) run the following commands from the libsovtoken subdirectory
.1) cargo update
.2) cargo build
5) run the tests
.1) cargo test

***Please keep in mind*** (at the time this document was written) some of the cargo crates are github repositories.
If you update libsovtoken code from github you need to build libsovtoken as follows:
1) cargo clean
2) cargo update
3) cargo build

The cargo update command is necessary to make sure you get latest from the cargo crates in github






