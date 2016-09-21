extern crate chrono;
use chrono::{DateTime,UTC,Duration};
// use std::time::Duration;

pub fn after(start_date: DateTime<UTC> ) -> DateTime<UTC> {
  let gigasecond = Duration::seconds(1_000_000_000);
  start_date + gigasecond
}