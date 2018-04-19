extern crate num;
extern crate regex;

use num::{BigInt, One};
use regex::Regex;

use std::ops::Rem;
use std::str::FromStr;

/// Returns true if the supplied string represents a valid IBAN.
pub fn validate_iban(iban: String) -> bool {
    if !check_iban_pattern(iban.as_str()) {
        println!("IBAN {} has invalid format!", iban);
        return false
    }

    let remainder : BigInt = remainder(validation_num(&iban), BigInt::from_str("97").unwrap());
    let one : BigInt = One::one();
    one.eq(&remainder)
 }

fn check_iban_pattern(iban: &str) -> bool {
    return Regex::new(r"^[A-Z]{2}\d{2}\w{1,30}$").unwrap().is_match(iban);
}

fn remainder(dividend: BigInt, divisor: BigInt) -> BigInt {
    dividend.rem(divisor)
}

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

fn reorder_iban(iban: &String) -> String {
    let mut validation_string_raw = String::new();
    validation_string_raw.push_str(&iban[4..]);
    validation_string_raw.push_str(&iban[..2]);
    validation_string_raw.push_str(&iban[2..4]);
    return validation_string_raw
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_test() {
        assert!(validate_iban(String::from("DE44500105175407324931")));
        assert!(validate_iban(String::from("VG96VPVG0000012345678901")));
        assert!(validate_iban(String::from("MT84MALT011000012345MTLCAST001S")));
    }

    #[test]
    fn test_invalid_checksum() {
        assert_eq!(false, validate_iban(String::from("DE17500105175407324931")));
    }

    #[test]
    fn test_illegal_format() {
        assert_eq!(false, validate_iban(String::from("")));
        assert_eq!(false, validate_iban(String::from("12BC5001051754$7324931")));
        assert_eq!(false, validate_iban(String::from("DE445001051754073249317891237892378")));
    }
}
