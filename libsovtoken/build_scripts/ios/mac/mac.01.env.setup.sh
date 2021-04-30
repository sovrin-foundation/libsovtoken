#!/bin/sh

#1) Install Rust and rustup (https://www.rust-lang.org/install.html).
#To get into the if statement below execute the following command...
# mv /Users/norm/.cargo/bin/rustup /Users/norm/.cargo/bin/rustup.bak
RUSTUP_VERSION=`rustup --version`
echo "executing ${PWD}/mac.01.env.setup as $USER "

if [ "$?" != "0" ]; then
           if [ -f $HOME/.cargo/bin/rustup ]; then
        echo "You need to add ${HOME}/.cargo/bin to your PATH environment variable or simply restart your terminal"
        exit 1
    else
        if [ -f /usr/local/bin/rustc ]; then
            sudo mv /usr/local/bin/rustc /usr/local/bin/rustc.bak
            sudo mv /usr/local/bin/rustdoc /usr/local/bin/rustdoc.bak
            sudo mv /usr/local/bin/rust-lldb /usr/local/bin/rust-lldb.bak
            sudo mv /usr/local/bin/rust-gdb /usr/local/bin/rust-gdb.bak
        fi
        if [ -d /usr/local/lib/rustlib ]; then
            sudo mv /usr/local/lib/rustlib /usr/local/lib/rustlib.bak
            sudo mkdir /usr/local/lib/rustlib.bak/libs
            sudo mv /usr/local/lib/librustc* /usr/local/lib/rustlib.bak/libs
        fi
        curl https://sh.rustup.rs -sSf | sh
        source $HOME/.cargo/env
        rustup component add rust-src
        rustup component add rust-docs
        rustup update
        RUSTUP_VERSION=`rustup --version`
        if [ -f /usr/local/bin/rustc.bak ]; then
            sudo mv /usr/local/bin/rustc.bak /usr/local/bin/rustc
            sudo mv /usr/local/bin/rustdoc.bak /usr/local/bin/rustdoc
            sudo mv /usr/local/bin/rust-lldb.bak /usr/local/bin/rust-lldb
            sudo mv /usr/local/bin/rust-gdb.bak /usr/local/bin/rust-gdb
        fi
        if [ -d /usr/local/lib/rustlib.bak ]; then
            sudo mv /usr/local/lib/rustlib.bak /usr/local/lib/rustlib
            sudo mv /usr/local/lib/rustlib/libs/* /usr/local/lib
            sudo rm -rf /usr/local/lib/rustlib/libs
        fi
    fi
fi
# rustup self uninstall that the step 1) works again

if [[ $RUSTUP_VERSION =~ ^'rustup ' ]]; then
    rustup update
    rustup default 1.46.0
    rustup component add rls-preview rust-analysis rust-src

    rustup target remove aarch64-linux-android armv7-linux-androideabi arm-linux-androideabi i686-linux-android x86_64-linux-android i386-apple-ios armv7s-apple-ios armv7-apple-ios
    rustup target add aarch64-apple-ios x86_64-apple-ios

    RUST_TARGETS=$(rustc --print target-list|grep -i ios)
    if [ "RUST_TARGETS" = "" ]; then
        sudo xcodebuild -license
        echo "If you have successfully accepted the xcode build license then just re-run this script."
        echo "If you were not able to successfully accept the xcode build license then run this command in a terminal 'sudo xcodebuild -license' until it is successful before you attempt to re-run this script"
        exit 1
    fi

    cargo install cargo-lipo
    cargo install cargo-xcode

    BREW_VERSION=`brew --version`
    echo "brew version is $BREW_VERSION"
    if ! [[ $BREW_VERSION =~ ^'Homebrew ' ]]; then
        echo "installing brew via curl"
        /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"
        brew doctor
        brew update
    fi

    #2) Install required native libraries and utilities (libsodium is added with URL to homebrew since version<1.0.15 is required)
    INSTALL_LIBSODIUM=https://raw.githubusercontent.com/Homebrew/homebrew-core/65effd2b617bade68a8a2c5b39e1c3089cc0e945/Formula/libsodium.rb
    brew list pkg-config &>/dev/null || brew install pkg-config
    brew list libsodium &>/dev/null || brew install ${INSTALL_LIBSODIUM}
    brew list automake &>/dev/null || brew install automake
    brew list autoconf &>/dev/null || brew install autoconf
    brew list cmake &>/dev/null || brew install cmake
    brew list openssl &>/dev/null || brew install openssl
    brew list zmq &>/dev/null || brew install zmq
    brew list wget &>/dev/null || brew install wget
    brew list truncate &>/dev/null || brew install truncate
    brew list libzip &>/dev/null || brew install libzip
    brew list python3 &>/dev/null || brew install python3
fi
