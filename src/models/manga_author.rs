use crate::schema::manga_authors;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = "manga_authors")]
pub struct MangaAuthor {
    pub manga_id: Uuid,
    pub author_id: Uuid,
    pub author_type: String,
}
