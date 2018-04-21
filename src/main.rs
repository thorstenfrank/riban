//  Copyright 2017 Thorsten Frank
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//       http://www.apache.org/licenses/LICENSE-2.0
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//
extern crate riban;

use std::env;
use std::process;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        process::exit(1);
    } else {
        //let split = &args[1..];
        //println!("{:?}", split);

        // concatenate all the remaining arguments - this way, formatted IBANs can be used as args
        let to_validate = String::from(args.split_off(1).join(""));
        let iban = riban::validate_iban(&to_validate);

        match iban {
            Ok(()) => println!("IBAN is valid"),
            Err(e) => println!("Invalid IBAN: {}", e),
        }
    }
}

fn help() {
    println!("Usage: riban <iban>");
    println!("");
    println!("Examples:");
    println!("riban DE12345678910");
    println!("riban DE12 3456 7891");
}
