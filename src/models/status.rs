// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Status` enum.

use serde::{Deserialize, Serialize};

/// Represents the status of a media.
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum Status {
    /// The media is finished.
    Finished,
    /// The media is currently releasing.
    Releasing,
    /// The media is not yet released.
    #[default]
    NotYetReleased,
    /// The media has been cancelled.
    Cancelled,
    /// The media is on hiatus.
    Hiatus,
    /// The media is currently ongoing.
    Current,
    /// The media is planned for future release.
    Planning,
    /// The media is completed.
    Completed,
    /// The media has been dropped.
    Dropped,
    /// The media is paused.
    Paused,
    /// The media is repeating.
    Repeating,
}

impl Status {
    /// Returns a summary of the status.
    pub fn summary(&self) -> &str {
        match self {
            Status::Finished => "Has completed and is no longer being updated.",
            Status::Releasing => "Currently releasing.",
            Status::NotYetReleased => "To be released in the future.",
            Status::Cancelled => "Ended before the work could be completed.",
            Status::Hiatus => "Currently paused with the intention of resuming in the future.",
            Status::Current => "Currently being updated.",
            Status::Planning => "Planned for future release.",
            Status::Completed => "Has completed and is no longer being updated.",
            Status::Dropped => {
                "No longer being updated due to a lack of interest or other reasons."
            }
            Status::Paused => "Currently paused.",
            Status::Repeating => "Repeating the same content.",
        }
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Finished => write!(f, "Finished"),
            Status::Releasing => write!(f, "Releasing"),
            Status::NotYetReleased => write!(f, "Not Yet Released"),
            Status::Cancelled => write!(f, "Cancelled"),
            Status::Hiatus => write!(f, "Hiatus"),
            Status::Current => write!(f, "Current"),
            Status::Planning => write!(f, "Planning"),
            Status::Completed => write!(f, "Completed"),
            Status::Dropped => write!(f, "Dropped"),
            Status::Paused => write!(f, "Paused"),
            Status::Repeating => write!(f, "Repeating"),
        }
    }
}
