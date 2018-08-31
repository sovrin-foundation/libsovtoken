## Libsovtoken Android Building
-------------------------------
Steps to build libsovtoken for Android

Prebuilt binaries exist for each of the dependencies:
- OpenSSL
- Libsodium
- ZMQ
- Libindy

Libsovtoken can be built without docker by running `build.nondocker.sh`
It can be built in docker by running `build.sh`.

Artifacts for each android ABI will be built and zipped together.

| ABI | Arch-Triplet |
| --- | ------------ |
| arm   | arm-linux-androideabi |
| armv7 | armv7-linux-androideabi |
| arm64 | aarch64-linux-android |
| x86   | i686-linux-android |
| x86_64 | x86_64-linux-android |

The zip file format will be available in `_build` directory and will look as follows:

**libsovtoken_<Cargo.toml-version>-yyyymmddHHMM-<git_short_rev>_<target_arch>.zip**

where `<Cargo.toml-version>` is what is in the *libsovtoken/Cargo.toml* under \[package]
version = "..."

where YYYY is the 4 digit year, mm is the two digit month, dd is the two digit day of the month, HH is the two digit UTC hour, MM is the two digit minute.

where `<git_short_rev>` is what is returned by `git rev-parse --short HEAD`

where `<target_arch>` is the target built. It will always be all for now, but is noted here in case a single ABI is needed.

The file can be uploaded to **Kraken** using the following command
```bash
curl -u <USERNAME> -X POST -F file=@./<LOCAL_PATH_TO_ZIP> https://kraken.corp.evernym.com/repo/<repo>/upload
```

where `<repo>` is one of `ios`, `android`, `npm`.


If you need credentials for **Kraken** you will need to give a Unix style password hash to technical enablement.
A simple tool *mkpwhash.py* from https://gitlab.corp.evernym.com/te/Ops-tools does the trick.

### Android Build Settings

*libsovtoken/build_scripts/android/android_settings.txt* contains the settings for building with android.

### Dependency Settings

1. Libindy
    1. To set the version of libindy to build against, set the file name in *libsovtoken/build_scripts/android/libsovtoken/libsovtoken.dependencies.txt*.
    1. A list of files can be found at https://repo.corp.evernym.com/filely/android
1. Openssl
1. Sodium
1. ZeroMQ
    1. To set the version o build against, set the file name in *libindy.dependencies.txt*
    1. A list of files can be found at https://repo.corp.evernym.com/filely/android


## Libindy Android Building
---------------------------

Libindy building uses all the same settings as libsovtoken minus those in the libsovtoken directory.
It also uses *libsovtoken/build_scripts/libindy.commit.sha1.hash.txt* which tells git which
version to use.

Libsovtoken can be built without docker by running `build.nondocker.sh`. Use `-d` flag e.g `build.nondocker.sh -d` if you want the dependencies should be downloaded and used automatically.

It can be built in docker by running `build.sh`.

Artifacts for each android ABI will be built and zipped together.
