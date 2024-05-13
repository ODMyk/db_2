use diesel::prelude::*;
use serde::{Deserialize, Serialize};

table! {
    playlists (id) {
        id -> Serial,
        user_id -> Int4,
        is_private -> Bool,
        title -> Text,
    }
}

#[derive(Deserialize, Serialize, QueryableByName)]
#[table_name = "playlists"]
pub(crate) struct Playlist {
    pub id: i32,
    pub user_id: i32,
    pub is_private: bool,
    pub title: String,
}
