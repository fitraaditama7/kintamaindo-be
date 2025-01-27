use crate::schema::chapters;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = "chapters")]
pub struct Chapter {
    pub id: Uuid,
    pub manga_id: Uuid,
    pub chapter_number: i32,
    pub sub_chapter_number: Option<i32>,
    pub title: String,
    pub release_date: Option<chrono::NaiveDate>,
}
