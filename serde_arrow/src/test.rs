use chrono::{DateTime, FixedOffset, NaiveDate, TimeZone, Utc};
use serde::Serialize;

use crate::{util::string_extractor::StringExtractor, Result};

#[test]
fn test_chrono_api_naive_datetime() -> Result<()> {
    let dt = NaiveDate::from_ymd(2021, 8, 3).and_hms(12, 0, 0);
    let dt_str = dt.serialize(StringExtractor)?;

    assert_eq!(dt_str, "2021-08-03T12:00:00");
    Ok(())
}

#[test]
fn test_chrono_api_datetime() -> Result<()> {
    let dt = Utc.ymd(730, 12, 1).and_hms(2, 3, 50);
    let dt_str = dt.serialize(StringExtractor)?;

    assert_eq!(dt_str, "0730-12-01T02:03:50Z");
    Ok(())
}

#[test]
fn test_chrono_fixed_offset() -> Result<()> {
    let dt = FixedOffset::east(5 * 3600)
        .ymd(2020, 12, 24)
        .and_hms(13, 30, 00);
    let dt_str = dt.serialize(StringExtractor)?;

    assert_eq!(dt_str, "2020-12-24T13:30:00+05:00");
    Ok(())
}

#[test]
fn test_parse_utc() -> Result<()> {
    let dt = "0730-12-01T02:03:50Z".parse::<DateTime<Utc>>()?;
    assert_eq!(dt, Utc.ymd(730, 12, 1).and_hms(2, 3, 50));

    let dt = "2020-12-24T13:30:00+05:00".parse::<DateTime<Utc>>()?;
    assert_eq!(dt, Utc.ymd(2020, 12, 24).and_hms(8, 30, 0));
    Ok(())
}
