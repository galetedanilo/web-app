use lazy_static::lazy_static;
use validator::ValidationError;
use regex::Regex;

pub fn helper_digit_validate(text: &str) -> Result<(), ValidationError> {

    lazy_static! {
        static ref DIGIT: Regex = Regex::new(r"[[:digit:]]").unwrap();
    }

    if DIGIT.is_match(text) {
        return Ok(());
    }

    Err(ValidationError::new("There are no numbers"))
}

pub fn helper_no_whitespace_validate(text: &str) -> Result<(), ValidationError> {

    lazy_static! {
        static ref WHITESPACE: Regex = Regex::new(r"[[:space:]]").unwrap();
    }

    if WHITESPACE.is_match(text) {
        return Err(ValidationError::new("There are blank spaces"));
    }

    Ok(())    
}

pub fn helper_lower_case_validate(text: &str) -> Result<(), ValidationError> {

    lazy_static! {
        static ref LOWER: Regex = Regex::new(r"[[:lower:]]").unwrap();
    }

    if LOWER.is_match(text) {
        return Ok(());
    }

    Err(ValidationError::new("There are no lower case"))
}

pub fn helper_upper_case_validate(text: &str) -> Result<(), ValidationError> {

    lazy_static! {
        static ref UPPER: Regex = Regex::new(r"[[:upper:]]").unwrap();
    }

    if UPPER.is_match(text) {
        return Ok(());
    }

    Err(ValidationError::new("There are no upper case"))
}