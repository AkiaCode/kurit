# Kurit
static website generator

## Clone
```sh
~/$ git clone --recursive https://github.com/AkiaCode/kurit && cd ./kurit
```

## Build
```sh
~/kurit$ cargo run --release -- build ./tests/profile.md
   Compiling kurit-ops v0.1.0 (/Volumes/Seritka/github/kurit/crates/kurit-ops)
   Compiling kurit-runtime v0.1.0 (/Volumes/Seritka/github/kurit/crates/kurit-runtime)
   Compiling kurit v0.1.0 (/Volumes/Seritka/github/kurit/crates/kurit)
    Finished release [optimized] target(s) in 9.78s
     Running `target/release/kurit build ./tests/profile.md`
Fin.
```

## Run
```sh
~/kurit$ cargo run --release
```
## Features (not yet)

### Single executable
```sh
~/kurit$ ldd ./target/debug/kurit
	linux-vdso.so.1 (0x00007ffddadfe000)
	libgcc_s.so.1 => /lib/x86_64-linux-gnu/libgcc_s.so.1 (0x00007fe2123e0000)
	libm.so.6 => /lib/x86_64-linux-gnu/libm.so.6 (0x00007fe2122f9000)
	libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6 (0x00007fe212000000)
	/lib64/ld-linux-x86-64.so.2 (0x00007fe2146df000)
```

### Zero-config
