Steps to build libindy.a and libvcx.a for iOS
when you have NOT built them before on this machine
## Setup environment
---------------------------------------------------------------------------
1. Install Xcode.app
    * If you don't you will see this error:
```bash
Executing: "cargo" "read-manifest"
Executing: "cargo" "build" "--target" "aarch64-apple-ios" "--lib" "--features" "" "--color" "auto" "--release" "--verbose"
error: failed to run `rustc` to learn about target-specific information

Caused by:
  process didn't exit successfully: `rustc - --crate-name ___ --print=file-names --target aarch64-apple-ios --crate-type bin --crate-type rlib --crate-type dylib --crate-type cdylib --crate-type staticlib --crate-type proc-macro` (exit code: 101)
--- stderr
error: Error loading target specification: Could not find specification for target "aarch64-apple-ios"
  |
  = help: Use `--print target-list` for a list of built-in targets
```
2. From the command line run the following:
```bash
xcode-select --install
sudo xcode-select -s /Applications/Xcode.app/Contents/Developer
sudo xcodebuild -license
```


## Building
---------------------------------------------------------------------------
1. Checkout the libsovtoken project using https://github.com/evernym/libsovtoken.git or git@github.com:evernym/libsovtoken.git
1. Startup a terminal and cd into libsovtoken/libsovtoken/build_scripts/ios/mac
1. Run the script `./mac.01.libindy.setup.sh` (make sure the brew install commands are successful)
    * If it succeeded, run the command `rustc --print target-list|grep -i ios`. It should output:
```bash
aarch64-apple-ios
armv7-apple-ios
armv7s-apple-ios
i386-apple-ios
x86_64-apple-ios
```
4. Restart your terminal for environment variables to take effect and cd into libsovtoken/libsovtoken/build_scripts/ios/mac
1. Run the script `source ./mac.02.libindy.env.sh`
1.
    a) Run the script `./mac.03.libindy.build.sh`

    b) Run these scripts if needed if `./mac.03.libindy.build.sh` fails:
    - `./mac.08.libssl.libcrypto.build.sh`
    - `./mac.09.libzmq.libsodium.build.sh`
    - `./mac.10.libminiz.libsqlite3.combine.sh`
1. Run the script `./mac.14.libsovtoken.build.sh` (Test failures do not prevent the .a files from being correctly built)
If you get the error
error: failed to add native library /usr/local/lib/libindy.a: File too small to be an archive
then that means the build.rs file in the libsovtoken/libsovtoken folder is setup incorrectly.
You must comment out the lines that look like this, then rerun the script...
        let libindy_lib_path = match env::var("LIBINDY_DIR"){
            Ok(val) => val,
            Err(..) => panic!("Missing required environment variable LIBINDY_DIR")
        };

        let openssl = match env::var("OPENSSL_LIB_DIR") {
            Ok(val) => val,
            Err(..) => match env::var("OPENSSL_DIR") {
                Ok(dir) => Path::new(&dir[..]).join("/lib").to_string_lossy().into_owned(),
                Err(..) => panic!("Missing required environment variables OPENSSL_DIR or OPENSSL_LIB_DIR")
            }
        };

        println!("cargo:rustc-link-search=native={}",libindy_lib_path);
        println!("cargo:rustc-link-lib=static=indy");
        println!("cargo:rustc-link-search=native={}", openssl);
        println!("cargo:rustc-link-lib=static=crypto");
        println!("cargo:rustc-link-lib=static=ssl");
