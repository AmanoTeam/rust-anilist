// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Title` struct.

use serde::{Deserialize, Serialize};

/// Represents a title with various language options.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub struct Title {
    /// The title in Romaji (Latin script).
    romaji: Option<String>,
    /// The title in English.
    english: Option<String>,
    /// The title in the native language.
    native: String,
    /// The title preferred by the user.
    user_preferred: Option<String>,
}

impl Title {
    /// Returns the title in Romaji (Latin script).
    pub fn romaji(&self) -> &str {
        self.romaji.as_deref().unwrap_or(&self.native)
    }

    /// Returns the title in English.
    pub fn english(&self) -> &str {
        self.english.as_deref().unwrap_or(&self.native)
    }

    /// Returns the title in the native language.
    pub fn native(&self) -> &str {
        &self.native
    }

    /// Returns the title preferred by the user.
    pub fn user_preferred(&self) -> &str {
        self.user_preferred.as_deref().unwrap_or(&self.native)
    }

    /// Checks if the title is empty.
    ///
    /// A title is considered empty if all of its fields are either `None` or empty.
    ///
    /// # Returns
    ///
    /// * `true` if the `romaji`, `english`, and `user_preferred` fields are `None`
    ///   and the `native` field is an empty string.
    /// * `false` otherwise.
    pub fn is_empty(&self) -> bool {
        self.romaji.is_none()
            && self.english.is_none()
            && self.native.is_empty()
            && self.user_preferred.is_none()
    }
}

impl From<Title> for String {
    fn from(title: Title) -> Self {
        title.native().to_string()
    }
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.native())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_romaji_with_romaji() {
        let title = Title {
            romaji: Some("Romaji Title".to_string()),
            english: None,
            native: "Native Title".to_string(),
            user_preferred: None,
        };

        assert_eq!(title.romaji(), "Romaji Title");
    }

    #[test]
    fn test_romaji_without_romaji() {
        let title = Title {
            romaji: None,
            english: None,
            native: "Native Title".to_string(),
            user_preferred: None,
        };

        assert_eq!(title.romaji(), "Native Title");
    }

    #[test]
    fn test_english_with_english() {
        let title = Title {
            romaji: None,
            english: Some("English Title".to_string()),
            native: "Native Title".to_string(),
            user_preferred: None,
        };

        assert_eq!(title.english(), "English Title");
    }

    #[test]
    fn test_english_without_english() {
        let title = Title {
            romaji: None,
            english: None,
            native: "Native Title".to_string(),
            user_preferred: None,
        };

        assert_eq!(title.english(), "Native Title");
    }

    #[test]
    fn test_native() {
        let title = Title {
            romaji: None,
            english: None,
            native: "Native Title".to_string(),
            user_preferred: None,
        };

        assert_eq!(title.native(), "Native Title");
    }

    #[test]
    fn test_user_preferred_with_user_preferred() {
        let title = Title {
            romaji: None,
            english: None,
            native: "Native Title".to_string(),
            user_preferred: Some("User Preferred Title".to_string()),
        };

        assert_eq!(title.user_preferred(), "User Preferred Title");
    }

    #[test]
    fn test_user_preferred_without_user_preferred() {
        let title = Title {
            romaji: None,
            english: None,
            native: "Native Title".to_string(),
            user_preferred: None,
        };

        assert_eq!(title.user_preferred(), "Native Title");
    }

    #[test]
    fn test_from_title_to_string() {
        let title = Title {
            romaji: None,
            english: None,
            native: "Native Title".to_string(),
            user_preferred: None,
        };
        let title_string: String = title.into();

        assert_eq!(title_string, "Native Title");
    }
}
