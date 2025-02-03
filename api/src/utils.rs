use chrono::{DateTime, Utc};
use rand::distr::{Alphanumeric, SampleString};
use sonyflake::{decompose, Builder, Sonyflake};
use tonic::Status;

/// Macro for initializing a regex struct only once and reusing a referance to it on future calls using the standard library's `OnceLock``.
macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: std::sync::OnceLock<regex::Regex> = std::sync::OnceLock::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

pub fn next_id(sf: &Sonyflake) -> Result<(i64, i64, u16), Status> {
    match sf.next_id() {
        Ok(id) => {
            let decomposed_id = decompose(id);
            Ok((
                id as i64,
                // time is in a non standard 1/100 second unix epoch time format used by sonyflake.
                decomposed_id.time as i64,
                decomposed_id.machine_id as u16,
            ))
        }
        Err(e) => Err(Status::internal(e.to_string())),
    }
}

pub fn new_sonyflake<T: Iterator<Item = u16>>(
    machine_ids: &mut T,
) -> Result<Sonyflake, Box<dyn std::error::Error>> {
    let machine_id = machine_ids
        .next()
        .ok_or("Not enough machine IDs in provoded range.")?;
    Ok(Builder::new()
        .start_time(DateTime::UNIX_EPOCH)
        .machine_id(&|| Ok(machine_id))
        .finalize()?)
}

pub fn validate_and_format_name(name: String) -> Option<String> {
    let bad_char_regex = regex!("[\t\n\r_]");
    let formatted = name.trim();
    let length = formatted.len();

    if length < 2 || length > 30 || bad_char_regex.is_match(formatted) {
        return None;
    }
    let mut space_count = 0;
    let mut last_char_is_space = false;
    for c in formatted.chars() {
        if c == ' ' {
            if space_count == 3 || last_char_is_space == true {
                return None;
            }
            space_count = space_count + 1;
            last_char_is_space = true;
        } else {
            last_char_is_space = false;
        }
    }
    Some(formatted.to_owned())
}

pub fn generate_random_name() -> String {
    Alphanumeric.sample_string(&mut rand::rng(), 13)
}

#[allow(dead_code)]
/// Gets integer unix timestamp in the non standard 1/100 second unix epoch time format used in our IDs.
pub fn get_currentrtf_timestamp() -> i64 {
    Utc::now().timestamp_millis() / 10
}

#[allow(dead_code)]
/// Gets chronos DateTime from a 1/100 second unix epoch timestamp.
pub fn new_date_time_from_timestamp(timestamp: i64) -> Option<DateTime<Utc>> {
    DateTime::from_timestamp_millis(timestamp * 10)
}
