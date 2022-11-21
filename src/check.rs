use fancy_regex::Regex;
use validator::ValidationError;

pub fn validation_message(msg: &'static str) -> ValidationError {
    let mut error = ValidationError::new("");
    error.message = Some(std::borrow::Cow::Borrowed(msg));
    error
}

pub fn check_with(
    test_str: &str,
    regex_str: &str,
    fail_message: &'static str,
) -> Result<(), ValidationError> {
    let regex = Regex::new(regex_str).map_err(|_| validation_message("Invalid Regex"))?;
    let result = regex
        .is_match(test_str)
        .map_err(|_| validation_message("Matching process failed"))?;

    match result {
        true => Ok(()),
        false => Err(validation_message(fail_message)),
    }
}

pub fn check_date(date: &str) -> Result<(), ValidationError> {
    check_with(
        date,
        r#"^(0[1-9]|1[0-9]|2[0-9]|(3[0-1]))[/](0[1-9]|1[0-2])[/]([12][0-9][0-9][0-9])$"#,
        "The date is in wrong format",
    )
}

pub fn check_field(field: &str) -> Result<(), ValidationError> {
    check_with(
        field,
        r#"^[a-zA-Z0-9-_ ]{3,50}$"#,
        "The field should contain a-z A-Z 0-9 - _ and from 3 to 20 characters in length",
    )
}
