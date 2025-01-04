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
