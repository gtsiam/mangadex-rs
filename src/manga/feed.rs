use serde::Serialize;
use uuid::Uuid;

use super::{ChapterList, MangaFeedQuery};

/// Get logged user followed manga feed (requires authentication)
///
/// Returns the feed (chapters list) for the logged user
///
/// Call to `GET /user/follows/manga/feed`
#[derive(Debug, Serialize, Clone)]
pub struct FollowedMangaFeed<'a> {
    #[serde(flatten)]
    pub query: &'a MangaFeedQuery,
}

impl_endpoint! {
    GET "/user/follows/manga/feed",
    #[body auth] FollowedMangaFeed<'_>, ChapterList
}

/// Manga feed
///
/// Returns the feed (chapters list) for the specified manga
///
/// Call to `GET /manga/{id}/feed`
#[derive(Debug, Serialize, Clone)]
pub struct MangaFeed<'a> {
    #[serde(skip)]
    pub manga_id: &'a Uuid,
    #[serde(flatten)]
    pub query: &'a MangaFeedQuery,
}

impl_endpoint! {
    GET ("/manga/{:x}/feed", manga_id),
    #[query] MangaFeed<'_>, ChapterList
}
