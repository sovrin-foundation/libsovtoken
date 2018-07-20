## Libsovtoken Android Building
-------------------------------
Steps to build libindy and libsovtoken for Android

With Docker
Prebuilt binaries exist for each of the _C_ dependencies:
- OpenSSL
- Libsodium
- ZMQ

These don't need to be built and the build folders for these exist in case its necessary.
These are built using the same method as indy and libsovtoken using the *build.sh* and its arguments.
Libindy is a necessary dependency that must be built until a prebuilt artifact exists.
Choose an architecture to build for <arm|arm64|x86>, the triplet will be
<arm-linux-androideabi|aarch64-linux-android|i686-linux-android> and run these commands:
```bash
cd indy
./build.sh <target-arch>
cd ../libsovtoken
mkdir libindy_<target-arch>
cp ../indy/indy-sdk/libindy/target/<target-arch-triplet>/release/libindy.so libindy_<target-arch>/
cp ../indy/indy-sdk/libindy/target/<target-arch-triplet>/release/libindy.a libindy_<target-arch>/
./build.sh <target-arch>
```

The binaries will be in **libsovtoken/libsovtoken/target/<target-arch-triplet/release**
- libsovtoken.so
- libsovtoken.a
