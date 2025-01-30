// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Color` struct.

use serde::{Deserialize, Serialize};

/// Represents a color with various predefined options and a custom hex value.
///
/// The `Color` enum defines a list of supported colors, each with an
/// associated variant. Additionally, it supports custom colors defined
/// by a hex string.
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
pub enum Color {
    /// The blue color.
    Blue,
    /// The purple color.
    #[default]
    Purple,
    /// The pink color.
    Pink,
    /// The orange color.
    Orange,
    /// The red color.
    Red,
    /// The green color.
    Green,
    /// The gray color.
    Gray,
    /// Others colors as hex.
    #[serde(untagged)]
    Hex(String),
}

impl Color {
    /// Returns the hex value of the color.
    pub fn hex(&self) -> Option<&str> {
        match self {
            Color::Hex(hex) => Some(hex),
            _ => None,
        }
    }
}

impl From<&str> for Color {
    fn from(value: &str) -> Self {
        match value.trim().to_uppercase().as_str() {
            "BLUE" => Color::Blue,
            "PURPLE" => Color::Purple,
            "PINK" => Color::Pink,
            "ORANGE" => Color::Orange,
            "RED" => Color::Red,
            "GREEN" => Color::Green,
            "GRAY" => Color::Gray,
            _ => Color::Hex(value.to_string()),
        }
    }
}

impl From<String> for Color {
    fn from(value: String) -> Self {
        Color::from(value.as_str())
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Color::Blue => write!(f, "Blue"),
            Color::Purple => write!(f, "Purple"),
            Color::Pink => write!(f, "Pink"),
            Color::Orange => write!(f, "Orange"),
            Color::Red => write!(f, "Red"),
            Color::Green => write!(f, "Green"),
            Color::Gray => write!(f, "Gray"),
            Color::Hex(hex) => write!(f, "{}", hex),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_with_hex_color() {
        let color = Color::Hex("#FF5733".to_string());

        assert_eq!(color.hex(), Some("#FF5733"));
    }

    #[test]
    fn test_hex_with_predefined_color() {
        let color = Color::Blue;

        assert_eq!(color.hex(), None);
    }

    #[test]
    fn test_from_str_predefined_colors() {
        assert_eq!(Color::from("blue"), Color::Blue);
        assert_eq!(Color::from("PURPLE"), Color::Purple);
        assert_eq!(Color::from("Pink"), Color::Pink);
        assert_eq!(Color::from("orange"), Color::Orange);
        assert_eq!(Color::from("RED"), Color::Red);
        assert_eq!(Color::from("green"), Color::Green);
        assert_eq!(Color::from("gray"), Color::Gray);
    }

    #[test]
    fn test_from_str_hex_color() {
        assert_eq!(Color::from("#FF5733"), Color::Hex("#FF5733".to_string()));
    }

    #[test]
    fn test_from_string_predefined_colors() {
        assert_eq!(Color::from("blue".to_string()), Color::Blue);
        assert_eq!(Color::from("PURPLE".to_string()), Color::Purple);
        assert_eq!(Color::from("Pink".to_string()), Color::Pink);
        assert_eq!(Color::from("orange".to_string()), Color::Orange);
        assert_eq!(Color::from("RED".to_string()), Color::Red);
        assert_eq!(Color::from("green".to_string()), Color::Green);
        assert_eq!(Color::from("gray".to_string()), Color::Gray);
    }

    #[test]
    fn test_from_string_hex_color() {
        assert_eq!(
            Color::from("#FF5733".to_string()),
            Color::Hex("#FF5733".to_string())
        );
    }
}
