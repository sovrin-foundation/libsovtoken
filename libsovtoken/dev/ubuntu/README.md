# Building and Testing Libsovtoken Client

Libsovtoken can be built for linux using the **build_docker.sh** script and Docker.

Libsovtoken depends on the libindy library.

**build_docker.sh** supports three options for installing libindy:
1. From a debian package via [apt-get](https://github.com/hyperledger/indy-sdk#ubuntu-based-distributions-ubuntu-1604).
1. Use rust to compile libindy from source in a local folder
1. Performs a git clone of [libindy](https://github.com/hyperledger/indy-sdk) to a local folder before doing step 2.

## Script flags

* **-h** - Displays a help message
* **-a** - Installs libindy using apt package manager from a specific channel. Default channel is 'stable'.
            Can be 'master|stable|rc'. This is the default method for install libindy.
            Options -i or -g will cause this option to be ignored.
* **-b** -  Use named branch for git clone. Default branch is 'master'
             Can be 'master|tags/v1.4|stable'. If the current branch in the local clone is
             not set to branch, the **build_docker.sh** will try to set it. If it fails, the program will abort.
* **-c** - Run a custom command instead of cargo \$mode.
            This is useful when you need to use more options with cargo
            like 'cargo test -- --nocapture' or 'cargo build --verbose' or 'cargo check'
* **-d** - Directory to find libsovtoken/src/Cargo.toml. Default is '..'.
* **-D** - Local directory where to clone libindy. Default is '/var/tmp/indy-sdk'.
            This option will be selected over -g if both are used.
* **-f** - Dockerfile to use to for building docker instance. Default is 'Dockerfile'
* **-g** - Use git to clone libindy from this URL and compile from source.
            Example: https://github.com/hyperledger/indy-sdk.git.
* **-i** - Compile libindy from local source directory. This is root folder to indy-sdk.
* **-j** - The number of cpus to give docker to run. Default is number of system physical cores. 0.000 means no limit.
* **-m** - The mode to run cargo inside docker. Default is 'build'.
            Valid options are 'build|release|test|check'.
* **-n** - Name to give the built docker image. Default is 'libsovtoken'
* **-o** - When combined with -g, force git clone in existing directory overwriting existing contents.
            Default is '0'
* **-r** - Combined with -i or -g, will force rebuilding of libindy. Default is '0'
* **-R** - Force reloading of cargo registries. Default is '0'
* **-s** - Shallow cloning libindy git installations
