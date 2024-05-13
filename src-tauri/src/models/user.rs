use diesel::prelude::*;
use serde::{Deserialize, Serialize};

table! {
    users (id) {
        id -> Serial,
        nickname -> Text,
        email -> Text,
        password -> Text,
        tier_id -> Int4,
    }
}

#[derive(Deserialize, Serialize, QueryableByName)]
#[table_name = "users"]
pub(crate) struct User {
    pub id: i32,
    pub nickname: String,
    pub email: String,
    pub password: String,
    pub tier_id: i32,
}
