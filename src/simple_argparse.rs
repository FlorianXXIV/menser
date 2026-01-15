use crate::constants::{set_lower_threshold_int, set_upper_threshold_int, DEFAULT_PRICE_THRESHOLD};
use crate::util::list_weekdays;
use color_eyre::eyre::bail;
use std::env;
use std::str::FromStr;
use time::Weekday;

pub const WEEKDAYS: &[&'static str] = &[
    "monday",
    "tuesday",
    "wednesday",
    "thursday",
    "friday",
    "saturday",
    "sunday",
];

#[derive(Default)]
pub struct Args {
    pub tomorrow: bool,
    pub weekday: Option<Weekday>,
}

pub fn argparse() -> color_eyre::Result<(Args, Vec<Weekday>)> {
    let mut args = Args::default();
    let raw_args = env::args();

    for raw in raw_args {
        match raw.to_lowercase().as_str() {
            "tomorrow" => args.tomorrow = true,
            "sides" => set_lower_threshold_int(0),
            "sides_only" => {
                set_upper_threshold_int(DEFAULT_PRICE_THRESHOLD);
                set_lower_threshold_int(0);
            }
            _ => {
                if WEEKDAYS.contains(&raw.to_lowercase().as_str()) {
                    args.weekday = Some(Weekday::from_str(&raw)?);
                }
            }
        }
    }

    let today = time::OffsetDateTime::now_local()?.weekday();

    let current_day = if args.tomorrow {
        today.next()
    } else {
        args.weekday.unwrap_or(today)
    };

    if !WEEKDAYS.contains(&current_day.to_string().to_lowercase().as_str()) {
        bail!("Unknown weekday argument");
    }

    let week_days = list_weekdays(current_day);

    Ok((args, week_days))
}
