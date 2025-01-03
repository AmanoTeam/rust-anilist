// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

use super::{
    Anime, Character, Color, Format, Image, Manga, NotificationOption, Person, Status, Studio,
};

/// A user.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct User {
    /// The ID of the user.
    pub id: i32,
    /// The name of the user.
    pub name: String,
    /// The about of the user.
    pub about: Option<String>,
    /// The avatar of the user.
    pub avatar: Option<Image>,
    /// The banner of the user.
    #[serde(rename = "bannerImage")]
    pub banner: Option<String>,
    /// The donator badge of the user.
    pub donator_badge: String,
    /// The donator tier of the user.
    pub donator_tier: i32,
    /// The favourites of the user.
    #[serde(skip)]
    pub favourites: Favourites,
    /// Whether the user is blocked or not.
    pub is_blocked: Option<bool>,
    /// Whether the user is a follower or not.
    pub is_follower: Option<bool>,
    /// Whether the user is following or not.
    pub is_following: Option<bool>,
    /// The media list options of the user.
    pub media_list_options: Option<MediaListOptions>,
    /// The options of the user.
    pub options: Option<Options>,
    #[serde(rename = "siteUrl")]
    pub url: String,
    /// The statistics of the user.
    pub statistics: UserStatisticTypes,
    /// The unread notification count of the user.
    pub unread_notification_count: Option<i32>,
    /// The created date of the user.
    pub created_at: i64,
    /// The updated date of the user.
    pub updated_at: i64,
}

/// The options of a user.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Options {
    /// The title language of the user.
    pub title_language: Option<UserTitleLanguage>,
    #[serde(default)]
    /// Whether the user wants to display adult content or not.
    pub display_adult_content: bool,
    /// Whether the user wants to receive airing notifications or not.
    #[serde(default)]
    pub airing_notifications: bool,
    /// The profile color of the user.
    pub profile_color: Color,
    /// The notifications options of the user.
    pub notifications_options: Option<Vec<NotificationOption>>,
    /// The timezone of the user.
    pub timezone: Option<String>,
    /// The activity merge time of the user.
    #[serde(default)]
    pub activity_merge_time: i32,
    /// The staff name language of the user.
    #[serde(default)]
    pub staff_name_language: UserStaffNameLanguage,
    /// Whether the user wants to restrict messages to following or not.
    #[serde(default)]
    pub restrict_messages_to_following: bool,
    /// The disabled list activity of the user.
    pub disabled_list_activity: Option<Vec<ListActivityOption>>,
}

/// The title language of a user.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum UserTitleLanguage {
    /// The Romaji title language.
    #[default]
    Romaji,
    /// The English title language.
    English,
    /// The native title language.
    Native,
    /// The Romaji stylised title language.
    RomajiStylised,
    /// The English stylised title language.
    EnglishStylised,
    /// The native stylised title language.
    NativeStylised,
}

/// The staff name language of a user.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum UserStaffNameLanguage {
    /// The Romaji Western staff name language.
    RomajiWestern,
    /// The Romaji staff name language.
    #[default]
    Romaji,
    /// The native staff name language.
    Native,
}

/// The list activity option of a user.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct ListActivityOption {
    /// The status of the list activity.
    status: Status,
    /// Whether the list activity is disabled or
    disabled: bool,
}

/// The media list options of a user.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MediaListOptions {
    /// The row order of the media list options.
    row_order: String,
    /// The anime list of the media list options.
    anime_list: MediaListTypeOptions,
    /// The manga list of the media list options.
    manga_list: MediaListTypeOptions,
}

/// The media list type options of a user.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MediaListTypeOptions {
    /// The section order of the media list type options.
    section_order: Vec<String>,
    /// Whether the completed section is split by format or not.
    split_completed_section_by_format: bool,
    /// The custom lists of the media list type options.
    custom_lists: Vec<String>,
    /// The advanced scoring of the media list type options.
    advanced_scoring: Vec<String>,
    /// Whether the advanced scoring is enabled or not.
    advanced_scoring_enabled: bool,
}

/// The favourites of a user.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Favourites {
    /// The favourited animes.
    anime: Vec<Anime>,
    /// The favourited mangas.
    manga: Vec<Manga>,
    /// The favourited characters.
    characters: Vec<Character>,
    /// The favourited staff.
    staff: Vec<Person>,
    /// The favourited studios.
    studios: Vec<Studio>,
}

/// The statistics of a user.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UserStatisticTypes {
    /// The anime statistics of the user.
    anime: UserStatistics,
    /// The manga statistics of the user.
    manga: UserStatistics,
}

/// The statistics of a user.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UserStatistics {
    /// The count of the statistics.
    count: i32,
    /// The standard deviation of the statistics.
    standard_deviation: Option<f32>,
    /// The minutes watched of the statistics.
    minutes_watched: Option<i32>,
    /// The episodes watched of the statistics.
    episodes_watched: Option<i32>,
    /// The chapters read of the statistics.
    chapters_read: Option<i32>,
    /// The volumes read of the statistics.
    volumes_read: Option<i32>,
    /// The formats of the statistics.
    formats: Option<Vec<UserFormatStatistic>>,
    /// The statuses of the statistics.
    statuses: Vec<UserStatusStatistic>,
}

/// The format statistics of a user.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UserFormatStatistic {
    /// The count of the format statistics.
    count: i32,
    /// The minutes watched of the format statistics.
    minutes_watched: Option<i32>,
    /// The chapters read of the format statistics.
    chapters_read: Option<i32>,
    /// The media IDs of the format statistics.
    #[serde(default)]
    media_ids: Vec<i32>,
    /// The format of the format statistics.
    format: Format,
}

/// The status statistics of a user.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UserStatusStatistic {
    /// The count of the status statistics.
    count: i32,
    /// The minutes watched of the status statistics.
    minutes_watched: Option<i32>,
    /// The episodes watched of the status statistics.
    chapters_read: Option<i32>,
    /// The media IDs of the status statistics.
    #[serde(default)]
    /// The status of the status statistics.
    media_ids: Vec<i32>,
    /// The status of the status statistics.
    status: Status,
}
