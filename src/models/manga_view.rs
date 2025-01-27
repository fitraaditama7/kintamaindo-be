use crate::schema::manga_views;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = "manga_views")]
pub struct MangaView {
    pub id: Uuid,
    pub manga_id: Uuid,
    pub viewed_at: chrono::NaiveDateTime,
}
