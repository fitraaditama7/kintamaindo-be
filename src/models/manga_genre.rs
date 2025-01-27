use crate::schema::manga_genres;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = "manga_genres")]
pub struct MangaGenre {
    pub manga_id: Uuid,
    pub genre_id: Uuid,
}
