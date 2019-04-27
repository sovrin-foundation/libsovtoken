# Building Libsovtoken for Windows

### Step 1 Build Liibndy 
Go through [indy sdk on windows build documentation](https://github.com/hyperledger/indy-sdk/blob/master/docs/build-guides/windows-build.md) to get libindy artifacts.
The most important steps are:
* [Downloading of the Indy prebuilt dependencies](https://github.com/hyperledger/indy-sdk/blob/master/docs/build-guides/windows-build.md#getbuild-dependencies)
* [Libindy building]( https://github.com/hyperledger/indy-sdk/blob/master/docs/build-guides/windows-build.md#build)

### Step 2 Environment Configuration
For this example
- all libraries and prebuilt indy-sdk dependencies was put in `C:\BIN\x64`

Set the following environment variables:
```
SET OPENSSL_DIR=C:\BIN\x64
SET SODIUM_LIB_DIR=C:\BIN\x64\lib
SET LIBINDY_DIR={libindy source directory}\target\[debug | release]
```

### Step 4 Building Libsovtoken
Build libsovtoken using the standard `cargo build` command
