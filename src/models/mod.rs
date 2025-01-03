// SPDX-License-Identifier: MIT↴
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>↴

//! This module contains various models and structures used in the library.

mod anime;
mod character;
mod color;
mod cover;
mod date;
mod format;
mod gender;
mod image;
mod language;
mod link;
mod manga;
mod name;
mod notification;
mod person;
mod relation;
mod season;
mod source;
mod status;
mod studio;
mod tag;
mod title;
mod user;

pub use anime::Anime;
pub use character::{Character, CharacterRole};
pub use color::Color;
pub use cover::Cover;
pub use date::Date;
pub use format::Format;
pub use gender::Gender;
pub use image::Image;
pub use language::Language;
pub use link::{Link, LinkType};
pub use manga::Manga;
pub use name::Name;
pub use notification::{Notification, NotificationOption, NotificationType};
pub use person::Person;
pub use relation::{Relation, RelationType};
pub use season::Season;
pub use source::Source;
pub use status::Status;
pub use studio::Studio;
pub use tag::Tag;
pub use title::Title;
pub use user::User;

use serde::{Deserialize, Serialize};

/// Represents different types of media.
///
/// The `MediaType` enum defines various types of media, such as anime,
/// manga, character, user, person, studio, and an unknown type.
///
/// # Variants
///
/// * `Anime` - Represents an anime.
/// * `Manga` - Represents a manga.
/// * `Character` - Represents a character.
/// * `User` - Represents a user.
/// * `Person` - Represents a person.
/// * `Studio` - Represents a studio.
/// * `Unknown` - Represents an unknown type of media.
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum MediaType {
    /// An anime.
    Anime,
    /// A manga.
    Manga,
    /// A character.
    Character,
    /// An user.
    User,
    /// A person.
    Person,
    /// A studio.
    Studio,
    /// Unknown type.
    #[default]
    Unknown,
}
