# LibSovToken


This is a payment handler library to work with libindy. It may one day be merged into libindy.


## Requirements

* LibIndy (Stable)

### Installing Indy-SDK

##### Ubuntu


``` shell
sudo apt-key adv --keyserver keyserver.ubuntu.com --recv-keys 68DB5E88
sudo add-apt-repository "deb https://repo.sovrin.org/sdk/deb stable"
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

Use `pwd` to get path to current directory

Add this to your bash profile:

```shell 
# EXAMPLE 
export LIBINDY_DIR='/my/path/to/indy-sdk/libindy/target/debug/'
```

## Running Indy Pool 

### Build the pool
```shell
    cd devops/indy-pool/ && docker build -t indy_pool . 
```

### run the poool
``` shell
    docker run -itd -p 9701-9708:9701-9708 indy_pool
```
