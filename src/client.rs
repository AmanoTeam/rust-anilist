// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use std::time::Duration;

use crate::models::{Anime, Character, Manga, Person};
use crate::Result;

#[derive(Clone)]
pub struct Client {
    api_token: Option<String>,
    timeout: u64,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            api_token: None,
            timeout: 20,
        }
    }
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
    /// ```no_run
    /// use anilist::Client;
    ///
    /// let client = Client::default();
    /// let anime = client.get_anime(Some(1), None).await.unwrap();
    /// ```
    pub async fn get_anime(
        &self,
        id: Option<i64>,
        mal_id: Option<i64>,
    ) -> Result<crate::models::Anime> {
        let variables = match id {
            Some(id) => serde_json::json!({ "id": id }),
            None => serde_json::json!({ "mal_id": mal_id.unwrap_or(0) }),
        };
        let data = self.request("anime", "get", variables).await.unwrap();

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
    /// ```no_run
    /// use anilist::Client;
    ///
    /// let client = Client::default();
    /// let manga = client.get_manga(Some(1), None).await.unwrap();
    /// ```
    pub async fn get_manga(
        &self,
        id: Option<i64>,
        mal_id: Option<i64>,
    ) -> Result<crate::models::Manga> {
        let variables = match id {
            Some(id) => serde_json::json!({ "id": id }),
            None => serde_json::json!({ "mal_id": mal_id.unwrap_or(0) }),
        };
        let data = self.request("manga", "get", variables).await.unwrap();

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
    /// ```no_run
    /// use anilist::Client;
    ///
    /// let client = Client::default();
    /// let character = client.get_character(1).await.unwrap();
    /// ```
    pub async fn get_character(&self, id: i64) -> Result<crate::models::Character> {
        let data = self
            .request("character", "get", serde_json::json!({ "id": id }))
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
    /// ```no_run
    /// use anilist::Client;
    ///
    /// let client = Client::default();
    /// let character = client.get_char(1).await.unwrap();
    /// ```
    pub async fn get_char(&self, id: i64) -> Result<crate::models::Character> {
        self.get_character(id).await
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
    /// ```no_run
    /// use anilist::Client;
    ///
    /// let client = Client::default();
    /// let person = client.get_person(1).await.unwrap();
    /// ```
    pub async fn get_person(&self, id: i64) -> Result<crate::models::Person> {
        let data = self
            .request("person", "get", serde_json::json!({ "id": id }))
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

    pub async fn search_anime(
        &self,
        variables: serde_json::Value,
    ) -> Option<Vec<crate::models::Anime>> {
        let result = self.request("anime", "search", variables).await.unwrap();
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
    pub(crate) async fn request(
        &self,
        media_type: &str,
        action: &str,
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
    pub(crate) fn get_query(media_type: &str, _action: &str) -> Result<String> {
        let media_type = media_type.to_lowercase();
        let media_types = ["anime", "manga", "character", "user", "person", "studio"];
        if !media_types.contains(&media_type.as_str()) {
            panic!("The media type '{}' does not exits", { media_type });
        }

        let graphql_query = match media_type.as_str() {
            "anime" => include_str!("../queries/get_anime.graphql").to_string(),
            "manga" => include_str!("../queries/get_manga.graphql").to_string(),
            "character" => include_str!("../queries/get_character.graphql").to_string(),
            // "user" => include_str!("../queries/get_user.graphql").to_string(),
            "person" => include_str!("../queries/get_person.graphql").to_string(),
            // "studio" => include_str!("../queries/get_studio.graphql").to_string(),
            _ => unimplemented!(),
        };

        Ok(graphql_query)
    }
}
