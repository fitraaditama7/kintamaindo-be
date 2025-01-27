use crate::schema::chapter_images;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = "chapter_images")]
pub struct ChapterImage {
    pub id: Uuid,
    pub chapter_id: Uuid,
    pub image_url: String,
    pub position: i32,
}
