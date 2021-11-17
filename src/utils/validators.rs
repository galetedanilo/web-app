use lazy_static::lazy_static;
use validator::{ValidationError, ValidationErrors};
use regex::Regex;

pub fn helper_is_number_validate(text: &str) -> Result<(), ValidationError> {

    lazy_static! {
        static ref DIGIT: Regex = Regex::new(r"[[:digit:]]").unwrap();
    }

    match DIGIT.is_match(text) {
        true => Ok(()),
        false => Err(ValidationError::new("There are no numbers"))
    }  
}

pub fn helper_no_whitespace_validate(text: &str) -> Result<(), ValidationError> {

    lazy_static! {
        static ref WHITESPACE: Regex = Regex::new(r"[[:space:]]").unwrap();
    }

    match WHITESPACE.is_match(text) {
        true => Err(ValidationError::new("There are blank spaces")),
        false => Ok(()),
    }      
}

pub fn helper_lower_case_validate(text: &str) -> Result<(), ValidationError> {

    lazy_static! {
        static ref LOWER: Regex = Regex::new(r"[[:lower:]]").unwrap();
    }

    match LOWER.is_match(text) {
        true => Ok(()),
        false => Err(ValidationError::new("There are no lower case")),
    }    
}

pub fn helper_upper_case_validate(text: &str) -> Result<(), ValidationError> {

    lazy_static! {
        static ref UPPER: Regex = Regex::new(r"[[:upper:]]").unwrap();
    }

    match UPPER.is_match(text) {
        true => Ok(()),
        false => Err(ValidationError::new("There are no upper case")),
    }
}

pub fn helper_get_messages(err: ValidationErrors) -> Vec<String> {

    let mut err_resp: Vec<String> = Vec::new();

    for (_key, value) in &err.field_errors() {
        for ex in value.into_iter() {
            err_resp.push(ex.to_string());
        }
    }

    err_resp
}