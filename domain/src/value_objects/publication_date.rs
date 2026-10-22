use std::fmt;
use std::str::FromStr;

const MONTHS: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

const WEEKDAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PublicationDate {
    year: u32,
    month: u32,
    day: u32,
}

impl PublicationDate {
    pub fn iso8601(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    pub fn rfc2822(&self) -> String {
        format!(
            "{}, {:02} {} {} 00:00:00 +0000",
            WEEKDAYS[self.weekday()],
            self.day,
            &self.month_name()[..3],
            self.year
        )
    }

    fn month_name(&self) -> &'static str {
        MONTHS[self.month as usize - 1]
    }

    fn weekday(&self) -> usize {
        const OFFSETS: [u32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let year = if self.month < 3 { self.year - 1 } else { self.year };

        ((year + year / 4 - year / 100 + year / 400 + OFFSETS[self.month as usize - 1] + self.day) % 7)
            as usize
    }
}

impl FromStr for PublicationDate {
    type Err = String;

    fn from_str(date: &str) -> Result<Self, Self::Err> {
        let invalid = || format!("the date `{date}` is not in the form `January 8, 2026`");

        let (month_and_day, year) = date.split_once(", ").ok_or_else(invalid)?;
        let (month, day) = month_and_day.split_once(' ').ok_or_else(invalid)?;
        let month = MONTHS.iter().position(|name| *name == month).ok_or_else(invalid)? as u32 + 1;
        let day: u32 = day.parse().map_err(|_| invalid())?;
        let year: u32 = year.parse().map_err(|_| invalid())?;

        if !(1..=31).contains(&day) {
            return Err(invalid());
        }

        Ok(Self { year, month, day })
    }
}

impl fmt::Display for PublicationDate {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}, {}", self.month_name(), self.day, self.year)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_date_in_the_form_of_the_front_matter() {
        let date: PublicationDate = "January 4, 2026".parse().unwrap();

        assert_eq!(date.to_string(), "January 4, 2026");
    }

    #[test]
    fn refuses_a_date_in_a_different_form() {
        for date in ["2026-01-04", "Jan 4, 2026", "January 32, 2026", "January 4 2026", ""] {
            assert!(date.parse::<PublicationDate>().is_err(), "{date}");
        }
    }

    #[test]
    fn writes_the_date_in_iso_8601_for_the_structured_data_and_the_sitemap() {
        let date: PublicationDate = "March 3, 2026".parse().unwrap();

        assert_eq!(date.iso8601(), "2026-03-03");
    }

    #[test]
    fn writes_the_date_in_rfc_2822_for_the_feed() {
        let cases = [
            ("January 4, 2026", "Sun, 04 Jan 2026 00:00:00 +0000"),
            ("February 29, 2024", "Thu, 29 Feb 2024 00:00:00 +0000"),
            ("December 31, 2025", "Wed, 31 Dec 2025 00:00:00 +0000"),
            ("March 1, 2026", "Sun, 01 Mar 2026 00:00:00 +0000"),
        ];

        for (date, expected) in cases {
            assert_eq!(date.parse::<PublicationDate>().unwrap().rfc2822(), expected);
        }
    }

    #[test]
    fn orders_the_dates_from_the_oldest_to_the_newest() {
        let older: PublicationDate = "December 31, 2025".parse().unwrap();
        let newer: PublicationDate = "January 4, 2026".parse().unwrap();

        assert!(older < newer);
    }
}
