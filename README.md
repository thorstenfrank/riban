# riban
Simple IBAN validator written in Rust

Takes an IBAN as an argument and tells you whether it's valid. And if not, why.

This is an application, not a proper crate to use as a lib - if you're looking for that,
check out https://crates.io/crates/iban_validate.

## Building

`cargo build`

## Running

Either

`cargo run <iban>`

or using the built binary:

`riban <iban>`

Where <iban> can be an IBAN in string format either with or without whitespace

Such as

`riban DE44 5001 0517 5407 3249 31`

or

`riban DE44500105175407324931`
