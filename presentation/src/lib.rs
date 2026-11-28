pub mod navigation;
pub mod templates;

use std::time::{SystemTime, UNIX_EPOCH};

pub const ASSET_VERSION: &str = env!("ASSET_VERSION");

const SECONDS_PER_DAY: u64 = 86_400;

#[must_use]
pub fn current_year() -> u64 {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("the system clock is before 1970")
        .as_secs();

    year_of(seconds)
}

fn year_of(unix_seconds: u64) -> u64 {
    let days = unix_seconds / SECONDS_PER_DAY + 719_468;
    let era = days / 146_097;
    let day_of_era = days - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let starts_in_january_or_february = day_of_year >= 306;

    era * 400 + year_of_era + u64::from(starts_in_january_or_february)
}

#[cfg(test)]
mod tests {
    use super::year_of;

    #[test]
    fn gives_1970_at_the_unix_epoch() {
        assert_eq!(year_of(0), 1970);
    }

    #[test]
    fn gives_the_old_year_at_the_last_second_of_december() {
        assert_eq!(year_of(1_767_225_599), 2025);
    }

    #[test]
    fn gives_the_new_year_at_the_first_second_of_january() {
        assert_eq!(year_of(1_767_225_600), 2026);
    }

    #[test]
    fn gives_the_year_of_a_leap_day() {
        assert_eq!(year_of(1_709_164_800), 2024);
    }

    #[test]
    fn gives_the_year_of_the_last_day_of_a_leap_year() {
        assert_eq!(year_of(1_735_603_200), 2024);
    }
}
