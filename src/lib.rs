// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This crate provides a Rust library for interacting with the AniList API.

#![deny(missing_docs)]

mod client;
mod error;
pub mod models;

pub use client::Client;
pub use error::{Error, Result};
