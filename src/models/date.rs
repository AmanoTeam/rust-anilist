// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Date` struct.

use chrono::{Datelike, Local, NaiveDate};
use serde::{Deserialize, Serialize};

/// Represents a date.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Date {
    /// The year of the date.
    pub year: Option<i32>,
    /// The month of the date.
    pub month: Option<u32>,
    /// The day of the date.
    pub day: Option<u32>,
}

impl Date {
    /// Creates a new date.
    pub fn new(year: Option<i32>, month: Option<u32>, day: Option<u32>) -> Self {
        Self { year, month, day }
    }

    /// Creates a new date from the current date.
    pub fn now() -> Self {
        let now = Local::now().naive_local().date();

        Self {
            year: Some(now.year()),
            month: Some(now.month()),
            day: Some(now.day()),
        }
    }

    /// Returns the year of the date.
    pub fn year(&self) -> Option<i32> {
        self.year
    }

    /// Returns the month of the date.
    pub fn month(&self) -> Option<u32> {
        self.month
    }

    /// Returns the day of the date.
    pub fn day(&self) -> Option<u32> {
        self.day
    }

    /// Formats the date according to the given format string.
    ///
    /// The format string can contain the following placeholders:
    /// - `{year}`, `{yyyy}`, `{yy}`, `{y}`, `{YEAR}`, `{YYYY}`, `{YY}`, `{Y}`: Year
    /// - `{month}`, `{mon}`, `{mm}`, `{m}`, `{MONTH}`, `{MON}`, `{MM}`, `{M}`: Month
    /// - `{day}`, `{dd}`, `{d}`, `{DAY}`, `{DD}`, `{D}`: Day
    ///
    /// # Arguments
    ///
    /// * `format` - A string slice that holds the format pattern.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rust_anilist::models::Date;
    /// let date = Date { year: Some(2023), month: Some(10), day: Some(5) };
    /// let formatted = date.format("{yyyy}-{mm}-{dd}");
    /// assert_eq!(formatted, "2023-10-05");
    /// ```
    pub fn format(&self, format: &str) -> String {
        let mut formatted = format.to_string();

        if let Some(year) = self.year {
            formatted = formatted.replace("{year}", &year.to_string());
            formatted = formatted.replace("{yyyy}", &year.to_string());
            formatted = formatted.replace("{yy}", &format!("{:02}", year % 100));
            formatted = formatted.replace("{y}", &year.to_string());
            formatted = formatted.replace("{YEAR}", &year.to_string());
            formatted = formatted.replace("{YYYY}", &year.to_string());
            formatted = formatted.replace("{YY}", &format!("{:02}", year % 100));
            formatted = formatted.replace("{Y}", &year.to_string());
        }

        if let Some(month) = self.month {
            formatted = formatted.replace("{month}", &format!("{:02}", month));
            formatted = formatted.replace("{mon}", &format!("{:02}", month));
            formatted = formatted.replace("{mm}", &format!("{:02}", month));
            formatted = formatted.replace("{m}", &month.to_string());
            formatted = formatted.replace("{MONTH}", &format!("{:02}", month));
            formatted = formatted.replace("{MON}", &format!("{:02}", month));
            formatted = formatted.replace("{MM}", &format!("{:02}", month));
            formatted = formatted.replace("{M}", &month.to_string());
        }

        if let Some(day) = self.day {
            formatted = formatted.replace("{day}", &format!("{:02}", day));
            formatted = formatted.replace("{dd}", &format!("{:02}", day));
            formatted = formatted.replace("{d}", &day.to_string());
            formatted = formatted.replace("{DAY}", &format!("{:02}", day));
            formatted = formatted.replace("{DD}", &format!("{:02}", day));
            formatted = formatted.replace("{D}", &day.to_string());
        }

        formatted
    }

    /// Returns the date as a `NaiveDate`.
    pub fn as_date(&self) -> NaiveDate {
        NaiveDate::from_ymd_opt(
            self.year.unwrap_or(0),
            self.month.unwrap_or(0),
            self.day.unwrap_or(0),
        )
        .unwrap()
    }

    /// Returns the date as a string.
    pub fn as_string(&self) -> String {
        let year = self.year.map_or(String::new(), |y| y.to_string());
        let month = self.month.map_or(String::new(), |m| format!("{:02}", m));
        let day = self.day.map_or(String::new(), |d| format!("{:02}", d));

        format!("{}-{}-{}", year, month, day)
    }

    /// Returns whether the date is valid.
    pub fn is_valid(&self) -> bool {
        self.year.is_some() && self.month.is_some() && self.day.is_some()
    }
}

impl From<NaiveDate> for Date {
    fn from(date: NaiveDate) -> Self {
        Self {
            year: Some(date.year()),
            month: Some(date.month()),
            day: Some(date.day()),
        }
    }
}

impl From<Date> for NaiveDate {
    fn from(date: Date) -> Self {
        date.as_date()
    }
}

impl std::fmt::Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Local;

    #[test]
    fn test_new() {
        let date = Date::new(Some(2023), Some(10), Some(5));

        assert_eq!(date.year(), Some(2023));
        assert_eq!(date.month(), Some(10));
        assert_eq!(date.day(), Some(5));
    }

    #[test]
    fn test_now() {
        let date = Date::now();
        let now = Local::now().naive_local().date();

        assert_eq!(date.year(), Some(now.year()));
        assert_eq!(date.month(), Some(now.month()));
        assert_eq!(date.day(), Some(now.day()));
    }

    #[test]
    fn test_year() {
        let date = Date::new(Some(2023), None, None);

        assert_eq!(date.year(), Some(2023));
    }

    #[test]
    fn test_month() {
        let date = Date::new(None, Some(10), None);

        assert_eq!(date.month(), Some(10));
    }

    #[test]
    fn test_day() {
        let date = Date::new(None, None, Some(5));

        assert_eq!(date.day(), Some(5));
    }

    #[test]
    fn test_format() {
        let date = Date::new(Some(2023), Some(10), Some(5));
        let formatted = date.format("{yyyy}-{mm}-{dd}");

        assert_eq!(formatted, "2023-10-05");
    }

    #[test]
    fn test_as_date() {
        let date = Date::new(Some(2023), Some(10), Some(5));
        let naive_date = date.as_date();

        assert_eq!(naive_date.year(), 2023);
        assert_eq!(naive_date.month(), 10);
        assert_eq!(naive_date.day(), 5);
    }

    #[test]
    fn test_as_string() {
        let date = Date::new(Some(2023), Some(10), Some(5));
        let date_string = date.as_string();

        assert_eq!(date_string, "2023-10-05");
    }

    #[test]
    fn test_is_valid() {
        let valid_date = Date::new(Some(2023), Some(10), Some(5));
        let invalid_date = Date::new(Some(2023), None, Some(5));

        assert!(valid_date.is_valid());
        assert!(!invalid_date.is_valid());
    }
}
