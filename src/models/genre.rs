use crate::schema::genres;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = "genres")]
pub struct Genre {
    pub id: Uuid,
    pub name: String,
}
