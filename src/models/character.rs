// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Character` struct and its related types.

use serde::{Deserialize, Serialize};

use super::{Date, Gender, Image, Name, Person};
use crate::{Client, Result};

/// Represents a character.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Character {
    /// The ID of the character.
    pub id: i64,
    /// The name of the character.
    pub name: Name,
    /// The role of the character in the story.
    pub role: Option<CharacterRole>,
    /// The image of the character.
    pub image: Image,
    /// The description of the character.
    pub description: String,
    /// The gender of the character.
    pub gender: Option<Gender>,
    /// The date of birth of the character.
    pub date_of_birth: Option<Date>,
    /// The age of the character.
    pub age: Option<String>,
    /// The blood type of the character.
    pub blood_type: Option<String>,
    /// Whether the character is a favorite.
    pub is_favourite: Option<bool>,
    /// Whether the character is blocked from being a favorite.
    pub is_favourite_blocked: Option<bool>,
    /// The URL of the character's site.
    #[serde(rename = "siteUrl")]
    pub url: String,
    /// The number of favorites the character has.
    pub favourites: Option<i64>,
    /// The voice actors of the character.
    pub voice_actors: Option<Vec<Person>>,
    /// The moderator notes for the character.
    pub mod_notes: Option<String>,

    /// The client used to fetch additional data.
    #[serde(skip)]
    pub(crate) client: Client,
    /// Whether the person's data is fully loaded.
    #[serde(default)]
    pub(crate) is_full_loaded: bool,
}

impl Character {
    /// Loads the full details of the character.
    ///
    /// # Errors
    ///
    /// Returns an error if the character details cannot be loaded.
    ///
    /// # Panics
    ///
    /// Panics if the character is already fully loaded.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rust_anilist::{models::Character, Result};
    /// #
    /// # async fn f(character: Character) -> Result<()> {
    /// let character = character.load_full().await?;
    /// # Ok(())
    /// # }
    pub async fn load_full(self) -> Result<Self> {
        if !self.is_full_loaded {
            self.client.get_character(self.id).await
        } else {
            panic!("This character is already full loaded")
        }
    }

    /// Retrieves the media associated with the chcharacterr.
    ///
    /// # Errors
    ///
    /// Returns an error if the media cannot be retrieved.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the media to be returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rust_anilist::{models::{Manga, Character}, Result};
    /// #
    /// # async fn f(character: Character) -> Result<()> {
    /// let char_mangas = character.get_medias::<Manga>().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_medias<T>(&self) -> Result<T> {
        unimplemented!()
    }
}

/// Represents the role of a character in a story.
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum CharacterRole {
    /// A background character.
    #[default]
    Background,
    /// A main character.
    Main,
    /// A supporting character.
    Supporting,
}
