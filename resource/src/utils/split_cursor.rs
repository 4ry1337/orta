use chrono::{DateTime, Utc};
use std::error::Error;

pub fn parse_cursor(
    cursor_str: &str,
) -> Result<(Option<&str>, Option<DateTime<Utc>>), Box<dyn Error>> {
    let mut id: Option<&str> = None;
    let mut created_at: Option<DateTime<Utc>> = None;

    let parts: Vec<&str> = cursor_str.split('_').collect();

    if let Some(&id_part) = parts.get(0) {
        id = Some(id_part);
    }

    if let Some(&datetime_part) = parts.get(1) {
        created_at = Some(DateTime::parse_from_rfc3339(datetime_part)?.into());
    }

    Ok((id, created_at))
}
