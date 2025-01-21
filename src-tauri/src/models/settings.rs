use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = settings)]
pub struct Setting {
    pub id: Option<i32>,
    pub key: String,
    pub value: String,
}

// Define the schema
diesel::table! {
    settings (id) {
        id -> Integer,
        key -> Text,
        value -> Text,
    }
}
