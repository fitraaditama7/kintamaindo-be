use crate::schema::authors;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = "authors")]
pub struct Author {
    pub id: Uuid,
    pub name: String,
}
