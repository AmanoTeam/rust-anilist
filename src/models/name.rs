// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Name` struct.

use serde::{Deserialize, Serialize};

/// Represents a name.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Name {
    /// The first name.
    pub first: String,
    /// The middle name, if any.
    pub middle: Option<String>,
    /// The last name, if any.
    pub last: Option<String>,
    /// The full name.
    pub full: String,
    /// The native name, if any.
    pub native: Option<String>,
    /// Alternative names.
    pub alternative: Vec<String>,
    /// Alternative names that may contain spoilers.
    pub alternative_spoiler: Option<Vec<String>>,
    /// The name preferred by the user, if any.
    pub user_preferred: Option<String>,
}

impl Name {
    /// Returns the full name.
    pub fn full(&self) -> String {
        self.full.clone()
    }

    /// Returns the native name, if any.
    pub fn native(&self) -> Option<String> {
        self.native.clone()
    }

    /// Returns the alternative names.
    pub fn alternative(&self) -> Vec<String> {
        self.alternative.clone()
    }

    /// Returns the alternative names that may contain spoilers.
    pub fn spoiler(&self) -> Option<Vec<String>> {
        self.alternative_spoiler.clone()
    }

    /// Returns the name preferred by the user, if any.
    pub fn user_preferred(&self) -> Option<String> {
        self.user_preferred.clone()
    }
}
