// SPDX-License-Identifier: MIT↴↴
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>↴↴

//! This module contains the `Season` enum.

use serde::{Deserialize, Serialize};

/// Represents the four seasons of the year.
///
/// The `Season` enum defines the four seasons: Winter, Spring, Summer,
/// and Fall. This can be used to categorize or filter data based on
/// the season.
///
/// # Variants
///
/// * `Winter` - Represents the winter season.
/// * `Spring` - Represents the spring season.
/// * `Summer` - Represents the summer season.
/// * `Fall` - Represents the fall season.
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
pub enum Season {
    /// Represents the winter season.
    #[default]
    Winter,
    /// Represents the spring season.
    Spring,
    /// Represents the summer season.
    Summer,
    /// Represents the fall season.
    Fall,
}

impl Season {
    /// Returns the name of the season.
    ///
    /// # Example
    ///
    /// ```
    /// # use rust_anilist::models::Season;
    /// let season = Season::Winter;
    /// assert_eq!(season.name(), "Winter");
    /// ```
    pub fn name(&self) -> &str {
        match self {
            Season::Winter => "Winter",
            Season::Spring => "Spring",
            Season::Summer => "Summer",
            Season::Fall => "Fall",
        }
    }
}

impl From<&str> for Season {
    fn from(value: &str) -> Self {
        match value.trim().to_uppercase().as_str() {
            "WINTER" => Season::Winter,
            "SPRING" => Season::Spring,
            "SUMMER" => Season::Summer,
            "FALL" => Season::Fall,
            _ => Season::default(),
        }
    }
}

impl From<String> for Season {
    fn from(value: String) -> Self {
        Season::from(value.as_str())
    }
}

impl std::fmt::Display for Season {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_season_name() {
        assert_eq!(Season::Winter.name(), "Winter");
        assert_eq!(Season::Spring.name(), "Spring");
        assert_eq!(Season::Summer.name(), "Summer");
        assert_eq!(Season::Fall.name(), "Fall");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Season::from("winter"), Season::Winter);
        assert_eq!(Season::from("SPRING"), Season::Spring);
        assert_eq!(Season::from("Summer"), Season::Summer);
        assert_eq!(Season::from("fall"), Season::Fall);
        assert_eq!(Season::from("unknown"), Season::Winter); // Default case
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Season::from("winter".to_string()), Season::Winter);
        assert_eq!(Season::from("SPRING".to_string()), Season::Spring);
        assert_eq!(Season::from("Summer".to_string()), Season::Summer);
        assert_eq!(Season::from("fall".to_string()), Season::Fall);
        assert_eq!(Season::from("unknown".to_string()), Season::Winter); // Default case
    }
}
