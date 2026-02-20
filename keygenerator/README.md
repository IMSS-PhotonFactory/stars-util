# KEYGENERATOR
A .key file generator for the STARS system

### How to Build
To build the keygenerator you need the Rust compiler. Please refer to https://rust-lang.org/ and follow the instructions.

Then execute the following command in the project directory.

Debug
```
cargo build
```
Release (Optimizations enabled)
```
cargo build --release
```

### How to use
The keygenerator accepts the filname as an input parameter.

For example:
```
>keygenerator term1.key
```
will generate a term1.key file.

The .key extension is optional. The key file will be created in the same directory as the executable. An existing key file will not be overwritten.

If started without a parameter, the keygenerator will ask for a filename.
