use diesel::prelude::*;
use serde::{Deserialize, Serialize};

table! {
    tiers (id) {
        id -> Serial,
        price -> Double,
        is_limited -> Bool,
        title -> Text,
    }
}

#[derive(Deserialize, Serialize, QueryableByName)]
#[table_name = "tiers"]
pub(crate) struct Tier {
    pub id: i32,
    pub price: f64,
    pub is_limited: bool,
    pub title: String,
}
