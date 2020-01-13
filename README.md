# Falco Rust Client

[![crates](https://img.shields.io/badge/crates.io-v0.0.0-orange.svg?style=flat-square&longCache=true)](https://crates.io/crates/falco)

> The rust language implementation of the Falco client

## Install

To include as a dependency:

```toml
[dependencies]
falco = "0.0.0"
```

## Full Examples

- [Output events example](examples/outputs.rs)

To execute any example:

```console
cargo run --example <name>
```

Where `<name>` needs to be changed with the file name of the example.

## Update protos

Do you want/need to update the proto files?

We provided a make command - ie., `make protos` - to update them.

```console
make clean
make
```
