use crate::schema::mangas;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = "mangas")]
pub struct Manga {
    pub id: Uuid,
    pub title: String,
    pub title_english: Option<String>,
    pub title_alternative: Option<String>,
    pub synonyms: Option<String>,
    pub status: Option<String>,
    pub chapters: Option<i32>,
    pub volumes: Option<i32>,
    pub score: Option<f32>,
    pub volumes: Option<i32>,
    pub serialization: Option<String>,
    pub synopsis: String,
    pub image_url: Option<String>,
    pub image_background_url: Option<String>,
    pub mal_url: Option<String>,
    pub manga_type: MangaType,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = diesel::sql_types::Text)]
pub enum MangaType {
    Manga,
    Manhwa,
    Manhua,
}

impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for MangaType
where
    DB: diesel::backend::Backend,
    String: diesel::deserialize::FromSql<diesel::sql_types::Text, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        match String::from_sql(bytes)?.as_str {
            "Manga" => Ok(MangaType::Manga),
            "Manhwa" => Ok(MangaType::Manhwa),
            "Manhua" => Ok(MangaType::Manhua),
            _ => Err("Invalid manga type".into()),
        }
    }
}

impl<DB> diesel::serialize::ToSql<diesel::sql_types::Text, DB> for &MangaType
where
    DB: diesel::backend::Backend,
    str: diesel::serialize::ToSql<diesel::sql_types::Text, DB>,
{
    fn to_sql(&self, out: &mut diesel::serialize::Output<DB>) -> diesel::serialize::Result {
        match *self {
            MangaType::Manga => out.write_all(b"Manga")?,
            MangaType::Manhwa => out.write_all(b"Manhwa")?,
            MangaType::Manhua => out.write_all(b"Manhua")?,
        }
        Ok(diesel::serialize::IsNull::No)
    }
}