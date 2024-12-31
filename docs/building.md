# Building CrabCraft

CrabCraft can be build using stable Rust with its package manager Cargo.

## Production

To build CrabCraft for production (this is what you usually want) run:

```shell
cargo build --release
```

CrabCraft binary named `crab_craft` will be found in target/release directory.

> [!TIP] You can prefix the command with `RUSTFLAGS='-C target-cpu=native'` to
> build CrabCraft specifically for your CPU, which will enable extra performance
> optimizations.

## Debug

To build CrabCraft with debug symbols run:

```shell
cargo build
```

> [!NOTE] In debug mode optimizations will be disabled, which means CrabCraft
> will have a much worse performance.

## Optional features

CrabCraft has optional features that can be enabled during compilation.

### Tokio console

Support for [tokio-console](https://github.com/tokio-rs/console) can be enabled
by compiling CrabCraft with the `tokio-console` feature flag.

```shell
cargo build --features tokio-console
```
