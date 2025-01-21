use chrono::NaiveDateTime;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = screenshots)]
pub struct Screenshot {
    pub id: Option<i32>,
    pub name: String,
    pub image: String,
    pub recognized: bool,
    pub ocr: bool,
    pub created_at: NaiveDateTime,
}

// Define the schema
diesel::table! {
    screenshots (id) {
        id -> Integer,
        name -> Text,
        image -> Text,
        recognized -> Bool,
        ocr -> Bool,
        created_at -> Timestamp,
    }
}
