use diesel::prelude::*;
use serde::{Deserialize, Serialize};

table! {
    links (playlist_id, song_id) {
        playlist_id -> Int4,
        song_id -> Int4,
    }
}

#[derive(Deserialize, Serialize, QueryableByName)]
#[diesel(table_name = links)]
pub(crate) struct Link {
    pub playlist_id: i32,
    pub song_id: i32,
}
