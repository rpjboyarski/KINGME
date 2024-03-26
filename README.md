# KINGME

Some King of the Hill utilities I wrote in an afternoon for things that are difficult to do quickly with just bash scripting. 

## "Features"
- Simple persistence mechanisms through cron jobs, services, and bash profiles
- Update and continuously write king.txt with IOCTLs to make it much harder to tamper with
- Path-Bomb command to move commmon binaries to a hidden folder, then replace them with another program, or /bin/true if none is specified
- Binary is statically linked with musl, so no need for the target to have a matching LIBC

## Installation
Don't forget to run **rustup toolchain add x86_64-unknown-linux-musl**

Other than that, it should be as simple as cloning the repository and building for release
