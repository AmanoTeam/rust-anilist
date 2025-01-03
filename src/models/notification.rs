// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Notification {}

/// Represents the options for a notification.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct NotificationOption {
    /// The type of the notification.
    notification_type: NotificationType,
    /// Whether the notification is enabled.
    enabled: bool,
}

/// Represents the type of a notification.
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum NotificationType {
    /// Notification for an activity message.
    #[default]
    ActivityMessage,
    /// Notification for an activity reply.
    ActivityReply,
    /// Notification for a new follower.
    Following,
    /// Notification for an activity mention.
    ActivityMention,
    /// Notification for a thread comment mention.
    ThreadCommentMention,
    /// Notification for an airing.
    Airing,
    /// Notification for an activity like.
    ActivityLike,
    /// Notification for an activity reply like.
    ActivityReplyLike,
    /// Notification for a thread like.
    ThreadLike,
    /// Notification for being subscribed to an activity reply.
    ActivityReplySubscribed,
    /// Notification for a related media addition.
    RelatedMediaAddition,
    /// Notification for a media data change.
    MediaDataChange,
    /// Notification for a media merge.
    MediaMerge,
    /// Notification for a media deletion.
    MediaDeletion,
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationType::ActivityMessage => write!(f, "Activity Message"),
            NotificationType::ActivityReply => write!(f, "Activity Reply"),
            NotificationType::Following => write!(f, "Following"),
            NotificationType::ActivityMention => write!(f, "Activity Mention"),
            NotificationType::ThreadCommentMention => write!(f, "Thread Comment Mention"),
            NotificationType::Airing => write!(f, "Airing"),
            NotificationType::ActivityLike => write!(f, "Activity Like"),
            NotificationType::ActivityReplyLike => write!(f, "Activity Reply Like"),
            NotificationType::ThreadLike => write!(f, "Thread Like"),
            NotificationType::ActivityReplySubscribed => write!(f, "Activity Reply Subscribed"),
            NotificationType::RelatedMediaAddition => write!(f, "Related Media Addition"),
            NotificationType::MediaDataChange => write!(f, "Media Data Change"),
            NotificationType::MediaMerge => write!(f, "Media Merge"),
            NotificationType::MediaDeletion => write!(f, "Media Deletion"),
        }
    }
}
