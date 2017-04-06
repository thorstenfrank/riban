# riban
Simple IBAN validator written in Rust

Just some dabbling around with Rust for the first time. 

Takes an IBAN as an argument and tells you whether it's valid or not. Woohoo!

Corrections, suggestions and tips highly welcome!

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
