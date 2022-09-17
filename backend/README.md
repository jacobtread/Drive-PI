![Logo](../assets/exported/svg/logo-side.svg)

# Drive-PI Backend

![License](https://img.shields.io/github/license/jacobtread/Drive-PI?style=for-the-badge)

This project is the backend code for the **Drive-PI** application this is written in Rust as is
intended to be run on a **Linux** based target that has NetworkManager installed (For the hotspot to work).

> **Drive-PI** cannot be compiled to run on Windows as it requires Unix apis and uses linux
> commands in order to function. It must be cross compiled for linux or ARM

## Compiling For Raspberry PI

### Windows

You will need to have rustup installed and add the `armv7-unknown-linux-gnueabihf` target using the 
following command:
```shell
rustup target add armv7-unknown-linux-gnueabihf
```

To compile the executable that runs on a **Raspberry PI** you will need to have an ARM toolchain that
contains `arm-linux-gnueabihf-gcc` I personally use the prebuilt toolchain from 
[https://gnutoolchains.com/raspberry/](https://gnutoolchains.com/raspberry/) this contains everything you 
need 

After you have the toolchain you can run the batch script named "build-arm.bat" or
execute the following command:
```shell
cargo build --target armv7-unknown-linux-gnueabihf --release
```