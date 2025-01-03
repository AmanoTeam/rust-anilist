// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

/// Represents a color with various predefined options and a custom hex value.
///
/// The `Color` enum defines a list of supported colors, each with an
/// associated variant. Additionally, it supports custom colors defined
/// by a hex string.
///
/// # Variants
///
/// * `Blue` - The blue color.
/// * `Purple` - The purple color. This is the default color.
/// * `Pink` - The pink color.
/// * `Orange` - The orange color.
/// * `Red` - The red color.
/// * `Green` - The green color.
/// * `Gray` - The gray color.
/// * `Hex(String)` - A custom color defined by a hex string.
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
