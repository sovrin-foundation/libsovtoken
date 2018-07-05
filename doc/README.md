# LibSovToken README.md

Libsovtoken is a payment handling library for hyperledger nodes running indy-sdk.

#### Contents
1) Building libsovtoken

## Building libsovtoken
This document exists for the purposes of assisting developers in building libsovtoken with the need for docker images.

### Requirements

#### Indy-SDK
You need Indy-SDK (or libindy) in order to build libsovtoken.  Two options are:
1) have indy-sdk installed or
2) build indy-sdk from source https://github.com/hyperledger/indy-sdk.git

#### Environment variables
Libsovtoken build needs to know how to find Indy-SDK.  This is done through the environment variable LIBINDY_DIR. 

Create an environment variable LIBINDY_DIR.   Have it point the directory containing indy-sdk binaries.

eg:  assuming you built indy-sdk from source, you will set the environment variable as such
```LIBINDY_DIR="/Users/my.home/src/hyperledger/indy-sdk/master/libindy/target/debug"```

**Note** anytime you get latest for indy-sdk, you must rebuild the libraries before building libsovtoken, as the libsovtoken build does not compile indy-sdk.

### Set up pool of 4 nodes with payment plugin
1) Clone the evernym/plugin directory from https://github.com/evernym/plugin .

2) Navigate into the plugin directory.

3) Run the `build_indy_pool.sh` script.
```
bash ./build_indy_pool.sh
```


### Compiling libsovtoken and running tests
1) Make sure you meet the requirements above, including building the projects, if you elect to use source code.
2) Gget latest for libsovtoken from master at https://github.com/evernym/libsovtoken.git
3) The source code is in a sub-directory called libsovtoken
4) Run the following commands from the libsovtoken subdirectory
   1) cargo update
   2) cargo build
5) Run the tests to ensure everything is in good order.
   1) cargo test

***Please keep in mind*** (at the time this document was written) some of the cargo crates are github repositories.
If you update libsovtoken code from github you need to re-build libsovtoken as follows:
1) cargo clean
2) cargo update
3) cargo build

The cargo update command is necessary to make sure you get latest from the cargo crates in github






