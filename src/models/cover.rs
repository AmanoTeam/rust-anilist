// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Cover` struct.

use serde::{Deserialize, Serialize};

use crate::models::Color;

/// Represents the cover images of various sizes and the color of the cover.
///
/// The `Cover` struct contains URLs for the cover images in different sizes
/// (extra large, large, and medium) and an optional color.
///
/// # Fields
///
/// * `extra_large` - The URL of the cover image in extra large size.
/// * `large` - The URL of the cover image in large size.
/// * `medium` - The URL of the cover image in medium size.
/// * `color` - The color of the cover image.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Cover {
    /// The URL of the cover image in extra large size.
    pub extra_large: Option<String>,
    /// The URL of the cover image in large size.
    pub large: Option<String>,
    /// The URL of the cover image in medium size.
    pub medium: Option<String>,
    /// The color of the cover image.
    pub color: Option<Color>,
}

impl Cover {
    /// Returns the URL of the largest version of the cover image.
    pub fn largest(&self) -> Option<&str> {
        if let Some(extra_large) = self.extra_large.as_deref() {
            Some(extra_large)
        } else if let Some(large) = self.large.as_deref() {
            Some(large)
        } else if let Some(medium) = self.medium.as_deref() {
            Some(medium)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_largest_with_extra_large() {
        let cover = Cover {
            extra_large: Some("https://example.com/extra_large.jpg".to_string()),
            large: Some("https://example.com/large.jpg".to_string()),
            medium: Some("https://example.com/medium.jpg".to_string()),
            color: None,
        };

        assert_eq!(cover.largest(), Some("https://example.com/extra_large.jpg"));
    }

    #[test]
    fn test_largest_with_large() {
        let cover = Cover {
            extra_large: None,
            large: Some("https://example.com/large.jpg".to_string()),
            medium: Some("https://example.com/medium.jpg".to_string()),
            color: None,
        };

        assert_eq!(cover.largest(), Some("https://example.com/large.jpg"));
    }

    #[test]
    fn test_largest_with_medium() {
        let cover = Cover {
            extra_large: None,
            large: None,
            medium: Some("https://example.com/medium.jpg".to_string()),
            color: None,
        };

        assert_eq!(cover.largest(), Some("https://example.com/medium.jpg"));
    }

    #[test]
    fn test_largest_with_none() {
        let cover = Cover {
            extra_large: None,
            large: None,
            medium: None,
            color: None,
        };

        assert_eq!(cover.largest(), None);
    }
}
