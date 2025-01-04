// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Image` struct.

use serde::{Deserialize, Serialize};

/// Represents an image with different sizes.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub struct Image {
    /// URL of the large version of the image.
    pub large: String,
    /// URL of the medium version of the image.
    pub medium: String,
}

impl Image {
    /// Returns the URL of the largest version of the image.
    pub fn largest(&self) -> &str {
        if self.large.is_empty() {
            &self.medium
        } else {
            &self.large
        }
    }
}
