# num

Arbitrary sized numeric types for Rust.

## Changes by dbalchev
new Trait ToBytes
BigUint impl of ToBytes
mod_pow for T:Num + ToBytes

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies.num]

git = "https://github.com/dbalchev/num"
```

and this to your crate root:

```rust
extern crate num;
```
