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
extern crate num;
extern crate regex;

use num::{BigInt, One};
use regex::Regex;
use std::env;
use std::ops::Rem;
use std::str::FromStr;

/**
 *
 */
fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please supply an IBAN!");
    } else {
        let iban = String::from(args.split_off(1).join(""));
        println!("Validating IBAN {}", iban);

        if validate_iban(iban) {
            println!("IBAN is valid");
        } else {
            println!("IBAN is NOT valid");
        }
    }
}

/**
 *
 */
fn validate_iban(iban: String) -> bool {
    if !check_iban_pattern(iban.as_str()) {
        println!("IBAN {} has invalid format!", iban);
        return false
    }

    let remainder : BigInt = remainder(validation_num(&iban), BigInt::from_str("97").unwrap());
    let one : BigInt = One::one();
    one.eq(&remainder)
 }

 /**
  *
  */
fn check_iban_pattern(iban: &str) -> bool {
    return Regex::new(r"^[A-Z]{2}\d{2}\w{1,30}$").unwrap().is_match(iban);
}

/**
 *
 */
fn remainder(dividend: BigInt, divisor: BigInt) -> BigInt {
    dividend.rem(divisor)
}

/**
 *
 */
fn validation_num(iban: &String) -> BigInt {
    let mut validation_string = String::new();
    for c in reorder_iban(&iban).chars() {
        if c.is_alphabetic() {
            validation_string.push_str(numeric_representation(c).as_str());
        } else if c.is_numeric() {
            validation_string.push(c);
        }
    }
    BigInt::from_str(validation_string.as_str()).unwrap()
}

/**
 *
 */
fn reorder_iban(iban: &String) -> String {
    let mut validation_string_raw = String::new();
    validation_string_raw.push_str(&iban[4..]);
    validation_string_raw.push_str(&iban[..2]);
    validation_string_raw.push_str(&iban[2..4]);
    return validation_string_raw
}

/**
 *
 */
fn numeric_representation(c: char) -> String {
    match c {
        'A' => String::from("10"),
        'B' => String::from("11"),
        'C' => String::from("12"),
        'D' => String::from("13"),
        'E' => String::from("14"),
        'F' => String::from("15"),
        'G' => String::from("16"),
        'H' => String::from("17"),
        'I' => String::from("18"),
        'J' => String::from("19"),
        'K' => String::from("20"),
        'L' => String::from("21"),
        'M' => String::from("22"),
        'N' => String::from("23"),
        'O' => String::from("24"),
        'P' => String::from("25"),
        'Q' => String::from("26"),
        'R' => String::from("27"),
        'S' => String::from("28"),
        'T' => String::from("29"),
        'U' => String::from("30"),
        'V' => String::from("31"),
        'W' => String::from("32"),
        'X' => String::from("33"),
        'Y' => String::from("34"),
        'Z' => String::from("35"),
        _ => String::from("0")
    }
}
