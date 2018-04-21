extern crate num;
extern crate regex;

use num::{BigInt, One};
use regex::Regex;
use std::ops::Rem;
use std::str::FromStr;

/// Returns true if the supplied string represents a valid IBAN.
pub fn validate_iban(iban: &String) -> Result<(), &'static str> {
    validate_iban_pattern(iban.as_str())?;
    validate_checksum(&iban)?;
    Ok(())
}

/// Validates the basic syntax
fn validate_iban_pattern(iban: &str) -> Result<(), &'static str> {
    let len = iban.len();
    if len < 5 {
        return Err("Minimum length is 5");
    } else if len > 34 {
        return Err("Maximum length is 34")
    }

    let country_code = &iban[0..2];
    if !Regex::new("[A-Z]{2}").unwrap().is_match(country_code) {
        return Err("Country code must be upper-case characters");
    }

    let checksum = &iban[2..4];
    if !Regex::new(r"\d{2}").unwrap().is_match(checksum) {
        return Err("Checksum is not a number");
    }

    let bban = &iban[4..];
    if Regex::new(r"\W").unwrap().is_match(bban) {
        return Err("BBAN must only contain alphanumeric characters");
    }

    Ok(())
}

fn validate_checksum(iban: &String) -> Result<(), &'static str> {
    let numeric = convert_to_numeric(&iban);
    let remainder = numeric.rem(BigInt::from_str("97").unwrap());
    let one : BigInt = One::one();
    if !one.eq(&remainder) {
        return Err("Checksum calculation failed");
    }

    Ok(())
}

fn convert_to_numeric(iban: &String) -> BigInt {
    let mut validation_string = String::new();
    for c in reorder_iban(&iban).chars() {
        if c.is_alphabetic() {
            push_digit(&mut validation_string, &c);
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
    validation_string_raw
}

fn push_digit(string: &mut String, c: &char) {
    if let Some(digit) = c.to_digit(36) {
        string.push_str(digit.to_string().as_str());
    }
}

// ############################################################################
// Tests
// ############################################################################
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_test() {
        assert!(validate_iban(&String::from("DE44500105175407324931")).is_ok());
    }

    #[test]
    fn empty_string() {
        let input = String::from("");
        let result = validate_iban(&input);
        assert!(result.is_err());
    }

    #[test]
    fn illegal_character() {
        assert!(validate_iban(&String::from("BC5001051754$7324931")).is_err());
    }

    #[test]
    fn no_country_code() {
        assert!(validate_iban(&String::from("12BC5001051754$7324931")).is_err());
    }

    #[test]
    fn checksum_format() {
        assert!(validate_iban(&String::from("DEZX500105175407324931")).is_err());
    }

    #[test]
    fn too_long() {
        assert!(validate_iban(&String::from("DE445001051754073249317891237892378")).is_err());
    }

    #[test]
    fn checksum_error() {
        assert!(validate_iban(&String::from("DE17500105175407324931")).is_err());
    }
}
