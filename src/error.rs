// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use thiserror::Error as TError;

/// A specialized `Result` type for operations that can return an `Error`.
///
/// This is defined as a convenience to avoid writing out `std::result::Result`
/// with the `Error` type repeatedly.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents the various errors that can occur in the application.
///
/// This enum defines different types of errors that can be encountered,
/// such as invalid IDs and API errors.
#[derive(TError, Debug)]
pub enum Error {
    #[error("invalid ID")]
    InvalidId,
    #[error("api error: `{0}`")]
    ApiError(String),
}
