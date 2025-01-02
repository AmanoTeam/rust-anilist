// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use std::io::Write;
use std::time::Duration;

use crate::models::{Anime, Character, Manga, Person, User};
use crate::Result;

#[derive(Clone)]
pub struct Client {
    /// The API token to use for requests.
    api_token: Option<String>,
    /// The timeout for requests (in seconds).
    timeout: u64,
}

impl Client {
    /// Set the API token.
    pub fn api_token(mut self, token: &str) -> Self {
        self.api_token = Some(token.to_string());
        self
    }

    /// Set the timeout for the requests (in seconds).
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout = seconds;
        self
    }

    /// Get an anime by its ID or MAL ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the anime.
    /// * `mal_id` - The MAL ID of the anime.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let anime = client.get_anime(1).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_anime(&self, id: i64) -> Result<crate::models::Anime> {
        let data = self
            .request(
                MediaType::Anime,
                Action::Get,
                serde_json::json!({ "id": id }),
            )
            .await
            .unwrap();

        match serde_json::from_str::<Anime>(&data["data"]["Media"].to_string()) {
            Ok(mut anime) => {
                anime.is_full_loaded = true;

                Ok(anime)
            }
            Err(e) => Err(crate::Error::ApiError(e.to_string())),
        }
    }

    /// Get a manga by its ID or MAL ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the manga.
    /// * `mal_id` - The MAL ID of the manga.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let manga = client.get_manga(1).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_manga(&self, id: i64) -> Result<crate::models::Manga> {
        let data = self
            .request(
                MediaType::Manga,
                Action::Get,
                serde_json::json!({ "id": id }),
            )
            .await
            .unwrap();

        match serde_json::from_str::<Manga>(&data["data"]["Media"].to_string()) {
            Ok(mut manga) => {
                manga.is_full_loaded = true;

                Ok(manga)
            }
            Err(e) => Err(crate::Error::ApiError(e.to_string())),
        }
    }

    /// Get a character by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the character.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let character = client.get_character(1).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_character(&self, id: i64) -> Result<crate::models::Character> {
        let data = self
            .request(
                MediaType::Character,
                Action::Get,
                serde_json::json!({ "id": id }),
            )
            .await
            .unwrap();

        match serde_json::from_str::<Character>(&data["data"]["Character"].to_string()) {
            Ok(mut character) => {
                character.is_full_loaded = true;

                Ok(character)
            }
            Err(e) => Err(crate::Error::ApiError(e.to_string())),
        }
    }

    /// Get a character by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the character.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let character = client.get_char(1).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_char(&self, id: i64) -> Result<crate::models::Character> {
        self.get_character(id).await
    }

    /// Get a user by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let user = client.get_user(1).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user(&self, id: i64) -> Result<crate::models::User> {
        let data = self
            .request(
                MediaType::User,
                Action::Get,
                serde_json::json!({ "id": id }),
            )
            .await
            .unwrap();

        match serde_json::from_str::<User>(&data["data"]["User"].to_string()) {
            Ok(user) => Ok(user),
            Err(e) => Err(crate::Error::ApiError(e.to_string())),
        }
    }

    /// Get a user by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the user.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let user = client.get_user_by_name("andrielfr").await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_by_name<N: ToString>(&self, name: N) -> Result<crate::models::User> {
        let name = name.to_string();

        let data = self
            .request(
                MediaType::User,
                Action::Get,
                serde_json::json!({ "name": name }),
            )
            .await
            .unwrap();

        match serde_json::from_str::<User>(&data["data"]["User"].to_string()) {
            Ok(user) => Ok(user),
            Err(e) => Err(crate::Error::ApiError(e.to_string())),
        }
    }

    /// Get a person by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the person.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let person = client.get_person(1).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_person(&self, id: i64) -> Result<crate::models::Person> {
        let data = self
            .request(
                MediaType::Person,
                Action::Get,
                serde_json::json!({ "id": id }),
            )
            .await
            .unwrap();

        match serde_json::from_str::<Person>(&data["data"]["Staff"].to_string()) {
            Ok(mut person) => {
                person.is_full_loaded = true;

                Ok(person)
            }
            Err(e) => Err(crate::Error::ApiError(e.to_string())),
        }
    }

    /// Search for anime.
    ///
    /// # Arguments
    ///
    pub async fn search_anime(
        &self,
        variables: serde_json::Value,
    ) -> Option<Vec<crate::models::Anime>> {
        let result = self
            .request(MediaType::Anime, Action::Search, variables)
            .await
            .unwrap();
        let mut _models = Vec::<crate::models::Anime>::new();
        unimplemented!()
    }

    /// Send a request to the AniList API.
    ///
    /// # Arguments
    ///
    /// * `media_type` - The type of media to request.
    /// * `action` - The action to perform.
    /// * `variables` - The variables to send with the request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    async fn request(
        &self,
        media_type: MediaType,
        action: Action,
        variables: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, reqwest::Error> {
        let query = Client::get_query(media_type, action).unwrap();
        let json = serde_json::json!({"query": query, "variables": variables});
        let mut body = reqwest::Client::new()
            .post("https://graphql.anilist.co/")
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .timeout(Duration::from_secs(self.timeout))
            .body(json.to_string());

        if let Some(token) = &self.api_token {
            body = body.bearer_auth(token);
        }

        let response = body.send().await?.text().await?;
        let result: serde_json::Value = serde_json::from_str(&response).unwrap();

        Ok(result)
    }

    /// Get the GraphQL query for a specific media type.
    ///
    /// # Arguments
    ///
    /// * `media_type` - The type of media to get the query for.
    ///
    /// # Errors
    ///
    /// Returns an error if the media type is not valid.
    fn get_query(media_type: MediaType, action: Action) -> Result<String> {
        let graphql_query = match action {
            Action::Get => {
                match media_type {
                    MediaType::Anime => include_str!("../queries/get_anime.graphql").to_string(),
                    MediaType::Manga => include_str!("../queries/get_manga.graphql").to_string(),
                    MediaType::Character => {
                        include_str!("../queries/get_character.graphql").to_string()
                    }
                    MediaType::User => include_str!("../queries/get_user.graphql").to_string(),
                    MediaType::Person => include_str!("../queries/get_person.graphql").to_string(),
                    // MediaType::Studio => include_str!("../queries/get_studio.graphql").to_string(),
                    _ => unimplemented!(),
                }
            }
            Action::Search => {
                match media_type {
                    // MediaType::Anime => include_str!("../queries/search_anime.graphql").to_string(),
                    // MediaType::Manga => include_str!("../queries/search_manga.graphql").to_string(),
                    // MediaType::Character => {
                    //     include_str!("../queries/search_character.graphql").to_string()
                    // }
                    // MediaType::User => include_str!("../queries/search_user.graphql").to_string(),
                    // MediaType::Person => {
                    //     include_str!("../queries/search_person.graphql").to_string()
                    // }
                    // MediaType::Studio => include_str!("../queries/search_studio.graphql").to_string(),
                    _ => unimplemented!(),
                }
            }
        };

        Ok(graphql_query)
    }
}

impl Default for Client {
    fn default() -> Self {
        Client {
            api_token: None,
            timeout: 20,
        }
    }
}

/// The action to perform.
enum Action {
    /// Get media by ID.
    Get,
    /// Search for media.
    Search,
}

/// The type of media to request.
enum MediaType {
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
}
