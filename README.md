![sovrinlogo](https://github.com/sovrin-foundation/sovrin/blob/master/banner.png "insert humor here")

<a href="https://www.apache.org/licenses/LICENSE-2.0.txt" target="_blank">![Hex.pm](https://img.shields.io/hexpm/l/plug.svg?style=plastic)</a>
<a href="https://badge.fury.io/gh/sovrin-foundation%2Flibsovtoken">[![GitHub version](https://badge.fury.io/gh/sovrin-foundation%2Flibsovtoken.svg)](https://badge.fury.io/gh/sovrin-foundation%2Flibsovtoken)</a>
# LibSovToken


This is a payment handler library to work with libindy. It may one day be merged into libindy.


## Requirements

* Rust Lang (Stable)
* LibIndy (Stable)
* Indy Pool (Provided)

### Installing Indy-SDK

##### Ubuntu


``` shell
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial stable"
sudo apt-get update
sudo apt-get install -y libindy>=1.6.0
```


##### macOS

```shell
git clone https://github.com/hyperledger/indy-sdk.git
cd indy-sdk
git checkout stable 
cd libindy
cargo clean 
cargo update
cargo build
```
Libsovtoken build needs to know how to find Indy-SDK. This is done through the environment variable LIBINDY_DIR.

Create an environment variable LIBINDY_DIR. Have it point the directory containing indy-sdk binaries.

Use `pwd` to get path to current directory

Add this to your bash profile:

```shell 
# EXAMPLE 
export LIBINDY_DIR='/my/path/to/indy-sdk/libindy/target/debug/'
```

*Note* anytime you get latest for indy-sdk, you must rebuild the libraries before building libsovtoken, as the libsovtoken build does not compile indy-sdk.

## Running Indy Pool 

### Build the pool
```shell
cd devops/indy-pool/ && docker build -t indy_pool . 
```

### run the poool
``` shell
docker run -itd -p 9701-9708:9701-9708 indy_pool
```
### Compiling libsovtoken and running tests
1) Make sure you meet the requirements above, including building the projects, if you elect to use source code.
2) Get latest for libsovtoken from master at https://github.com/evernym/libsovtoken.git
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

## How To Contribute

Please follow the guide [here](./doc/pull-request.md).
