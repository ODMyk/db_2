use diesel::prelude::*;
use serde::{Deserialize, Serialize};

table! {
    songs (id) {
        id -> Serial,
        user_id -> Int4,
        times_played -> Int4,
        title -> Text,
    }
}

#[derive(Deserialize, Serialize, QueryableByName)]
#[table_name = "songs"]
pub(crate) struct Song {
    pub id: i32,
    pub user_id: i32,
    pub times_played: i32,
    pub title: String,
}
