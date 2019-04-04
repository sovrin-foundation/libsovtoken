Building Libsovtoken


## Windows Builds
### Prerequisites
Visual Studio, git, etc


### Step 1
Start with following the directions for building indy sdk:

https://github.com/hyperledger/indy-sdk/blob/master/docs/build-guides/windows-build.md

#### Step 1 Notes
some differences in what I did:
downloaded openssl from here: https://slproweb.com/products/Win32OpenSSL.html.  When prompted for where the libraries should go, I chose bin directory


### Step 2
For this example
- all libraries and prebuilt prebuilt indy-sdk dependencies was put in `d:\engineering\libs`
- during open ssl install, selected the option install opensll in bin sub-directory

Set the following environment variables:
```
SET OPENSSL_DIR=C:\PROGRA~1\OPENSS~1\
SET X86_64_PC_WINDOWS_MSVC_OPENSSL_LIB_DIR=%OPENSSL_DIR%lib
SET OPENSSL_INCLUDE_DIR=%OPENSSL_DIR%include
SET X86_64_PC_WINDOWS_MSVC_OPENSSL_INCLUDE_DIR=%OPENSSL_DIR%include
SET SODIUM_LIB_DIR=d:\engineering\libs
SET SODIUM_STATIC=d:\engineering\libs\lib
SET LIBINDY_DIR=d:\engineering\libs
SET INDY_PREBUILT_DEPS_DIR=d:\engineering\libs

SET INDY_PREBUILT_DEPS_DIR=d:\engineering\libs
SET INDY_CRYPTO_PREBUILT_DEPS_DIR=d:\engineering\libs
SET MILAGRO_DIR=d:\engineering\libs
SET LIBZMQ_PREFIX=d:\engineering\libs
```

#### Step 2 notes
Use short file names and short directory names.  There seemed to be some problems with long names.


### Step 3 (optional)
build indy-sdk (if you can get the windows build of indy-sdk then you can skip this) using the standard cargo build command and copied dll and lib to d:\engineering\libs

### Step 4
build libsovtoken using the standard `cargo build` command
