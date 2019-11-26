![img](https://github.com/sovrin-foundation/sovrin/blob/master/banner.png)

# Table of Contents

1.  [LibSovToken](#orged7f66f)
    1.  [Requirements](#requirements)
        1.  [Installing Indy-SDK](#installing-indy-sdk)
    2.  [Running Indy Pool](#running-indy-pool)
        1.  [Build the pool](#build-the-pool)
        2.  [run the poool](#run-the-poool)
        3.  [Compiling libsovtoken and running tests](#compiling-libsovtoken-and-running-tests)
    3.  [How To Contribute](#how-to-contribute)

<a href="https://www.apache.org/licenses/LICENSE-2.0.txt" target="_blank">![Hex.pm](https://img.shields.io/hexpm/l/plug.svg?style=plastic)</a>
<a href="https://badge.fury.io/gh/sovrin-foundation%2Flibsovtoken">[![GitHub version](https://badge.fury.io/gh/sovrin-foundation%2Flibsovtoken.svg)](https://badge.fury.io/gh/sovrin-foundation%2Flibsovtoken)</a>

<a id="orged7f66f"></a>

# LibSovToken

Adds Sovrin's token functionality to HyperLedger's Indy-SDK.


<a id="requirements"></a>

## Requirements

-   Rust Lang (Stable)
-   LibIndy (Stable)
-   Indy Pool (Provided)


<a id="installing-indy-sdk"></a>

### Installing Indy-SDK

1.  Ubuntu

        sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
        sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial stable"
        sudo apt-get update
        sudo apt-get install -y libindy>=1.6.0

2.  macOS

        git clone https://github.com/hyperledger/indy-sdk.git
        cd indy-sdk
        git checkout stable
        cd libindy
        cargo clean
        cargo update
        cargo build

    Libsovtoken build needs to know how to find Indy-SDK. This is done
    through the environment variable LIBINDY_DIR.

    Create an environment variable LIBINDY_DIR. Have it point the directory
    containing indy-sdk binaries.

    Use `pwd` to get path to current directory

    Add this to your bash profile:

        # EXAMPLE
        export LIBINDY_DIR='/my/path/to/indy-sdk/libindy/target/debug/'

    *Note* anytime you get latest for indy-sdk, you must rebuild the
    libraries before building libsovtoken, as the libsovtoken build does not
    compile indy-sdk.


<a id="running-indy-pool"></a>

## Running Indy Pool


<a id="build-the-pool"></a>

### Build the pool

    cd devops/indy-pool/ && docker build -t indy_pool .


<a id="run-the-pool"></a>

### run the pool

    docker run -itd -p 9701-9708:9701-9708 indy_pool


<a id="compiling-libsovtoken-and-running-tests"></a>

### Compiling libsovtoken and running tests

1.  Make sure you meet the requirements above, including building the
    projects, if you elect to use source code.
2.  Get latest for libsovtoken at
    <https://github.com/sovrin-foundation/libsovtoken.git>
3.  The source code is in a sub-directory called libsovtoken
4.  Run the following commands from the libsovtoken subdirectory
    1.  cargo update
    2.  cargo build

5.  Run the tests to ensure everything is in good order.
    1.  cargo test


### How to build Libsovtoken from source
* [Windows](doc/build-guides/windows-build.md)

<a id="how-to-contribute"></a>

## Installing the Sovtoken

### Release channels
The Sovtoken release process defines the following release channels:

* `master` - development builds for each push to master branch.
* `stable` - stable releases.

### Ubuntu based distributions (Ubuntu 16.04)
It is recommended to install the SDK packages with APT:

    sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
    sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb xenial {release channel}"
    sudo apt-get update
    sudo apt-get install -y libsovtoken

### Windows

1. Go to `https://repo.sovrin.org/windows/libsovtoken/{release-channel}`
2. Download last version of libsovtoken.
3. Unzip archives to the directory where you want to save library.

### iOS
1. Go to `https://repo.sovrin.org/ios/libsovtoken/{release-channel}/libsovtoken-core/`.
2. 5 target architecture are supported as of now aarch64-apple-ios, armv7-apple-ios, armv7s-apple-ios, i386-apple-ios, x86_64-apple-ios and universal.
3. Download latest version of libsovtoken.
4. Unzip archives to the directory where you want to save library.

### Android

1. Go to `https://repo.sovrin.org/android/libsovtoken/{release-channel}`.
2. 3 architecture are supported as of now arm,arm64 and x86.
3. Download latest version of libsovtoken.
4. Unzip archives to the directory where you want to save library.

## How To Contribute

Please follow the guide [here](./doc/pull-request.md).
