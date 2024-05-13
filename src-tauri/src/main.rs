// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;

extern crate diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_query;
use models::link::Link;
use models::playlist::Playlist;
use models::song::Song;
use models::tier::Tier;
use models::user::User;
use std::fs::read_to_string;
use std::fs::write;
use std::sync::Mutex;
use std::sync::OnceLock;

struct Database {
    pub url: String,
}

impl Database {
    fn new() -> Database {
        Self {
            url: String::from(
                "host=127.0.0.1 port=5432 dbname=sahodb user=postgres password=postgres",
            ),
        }
    }

    fn change(&mut self, url: String) {
        self.url = url;
    }
}

fn url_wrapper() -> Mutex<&'static mut Database> {
    static mut DATABASE: OnceLock<Database> = OnceLock::new();
    unsafe {
        DATABASE.get_or_init(|| Database::new());
        Mutex::new(DATABASE.get_mut().unwrap())
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_users,
            get_songs,
            get_tiers,
            get_playlists,
            get_links,
            delete_tier,
            delete_user,
            delete_song,
            delete_playlist,
            delete_link,
            edit_user,
            edit_tier,
            edit_song,
            edit_playlist,
            edit_link,
            create_tier,
            create_user,
            create_song,
            create_playlist,
            create_link,
            connect_to_db,
            init_db,
            query1,
            query2,
            query3,
            query4,
            query5,
            query6,
            query7,
            query8
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn query1(user_id: i32, count: i32, private: bool, public: bool) -> Result<Vec<Song>, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let mut playlist_type_condition = "";
    if private && !public {
        playlist_type_condition = " AND playlists.is_private";
    } else if !private && public {
        playlist_type_condition = " AND NOT playlists.is_private";
    }

    let results =
        sql_query(format!("SELECT * FROM songs WHERE user_id={} AND times_played >= {} AND EXISTS (SELECT id FROM playlists INNER JOIN playlistSongs on playlists.id=playlistSongs.playlist_id WHERE songs.id=playlistSongs.song_id{}) ORDER BY id;", user_id, count, playlist_type_condition)).load::<Song>(&mut connection.unwrap());
    if results.is_err() {
        return Err(results.err().unwrap().to_string());
    }
    Ok(results.unwrap())
}

#[tauri::command]
fn query2(user_id: i32, playlist_id: i32) -> Result<Vec<Song>, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let results =
        sql_query(format!("SELECT * FROM songs WHERE user_id={} AND EXISTS(SELECT * FROM playlistSongs WHERE song_id=songs.id AND playlist_id={}) ORDER BY id;", user_id, playlist_id)).load::<Song>(&mut connection.unwrap());
    if results.is_err() {
        return Err(results.err().unwrap().to_string());
    }
    Ok(results.unwrap())
}

#[tauri::command]
fn query3(count: i32, limited: bool) -> Result<Vec<User>, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let mut prefix = "NOT ";
    if limited {
        prefix = "";
    }

    let results =
        sql_query(format!("SELECT * FROM users WHERE EXISTS (SELECT id FROM tiers WHERE id=users.tier_id AND {}is_limited) AND (SELECT COUNT(id) FROM songs WHERE user_id=users.id LIMIT {}) >= {} ORDER BY id;", prefix, count + 1, count)).load::<User>(&mut connection.unwrap());
    if results.is_err() {
        return Err(results.err().unwrap().to_string());
    }
    Ok(results.unwrap())
}

#[tauri::command]
fn query4(title: &str, count: i32) -> Result<Vec<User>, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let results =
        sql_query(format!("SELECT * FROM users WHERE EXISTS (SELECT id FROM tiers WHERE id=users.tier_id AND title='{}') AND EXISTS (SELECT id FROM playlists WHERE user_id=users.id AND (SELECT COUNT(playlist_id) FROM playlistSongs WHERE playlist_id=playlists.id LIMIT {}) >= {}) ORDER BY id;", title, count + 1, count)).load::<User>(&mut connection.unwrap());
    if results.is_err() {
        return Err(results.err().unwrap().to_string());
    }
    Ok(results.unwrap())
}

#[tauri::command]
fn query5(
    users_count: i32,
    playlists_count: i32,
    private: bool,
    public: bool,
) -> Result<Vec<Tier>, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let mut playlist_type_condition = "";
    if private && !public {
        playlist_type_condition = " AND playlists.is_private";
    } else if !private && public {
        playlist_type_condition = " AND NOT playlists.is_private";
    }

    let results = sql_query(format!("SELECT * FROM tiers WHERE (SELECT COUNT(id) FROM users WHERE (SELECT COUNT(id) FROM playlists WHERE user_id=users.id{}) >= {} AND tier_id=tiers.id) >= {} ORDER BY id;", playlist_type_condition, playlists_count, users_count)).load::<Tier>(&mut connection.unwrap());
    if results.is_err() {
        return Err(results.err().unwrap().to_string());
    }
    Ok(results.unwrap())
}

#[tauri::command]
fn query6(user_id: i32, private: bool, public: bool) -> Result<Vec<Playlist>, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let mut playlist_type_condition = "";
    if private && !public {
        playlist_type_condition = "playlists.is_private AND ";
    } else if !private && public {
        playlist_type_condition = "NOT playlists.is_private AND ";
    }

    let results = sql_query(format!("SELECT * FROM playlists WHERE {}(SELECT COUNT(id) FROM songs WHERE user_id={}) = (SELECT COUNT(id) FROM songs WHERE user_id={} AND EXISTS(SELECT playlist_id FROM playlistSongs WHERE song_id=songs.id AND playlist_id=playlists.id)) ORDER BY id;", playlist_type_condition, user_id, user_id)).load::<Playlist>(&mut connection.unwrap());
    if results.is_err() {
        return Err(results.err().unwrap().to_string());
    }
    Ok(results.unwrap())
}

#[tauri::command]
fn query7(playlist_id: i32, songs_count: i32) -> Result<Vec<Playlist>, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let results = sql_query(format!("SELECT * FROM playlists WHERE (SELECT COUNT(id) FROM songs WHERE user_id=playlists.user_id) >= {} AND (SELECT DISTINCT COUNT(song_id) FROM playlistSongs WHERE playlist_id={}) <= (SELECT COUNT(playlist_id) FROM playlistSongs WHERE playlist_id=playlists.id AND EXISTS(SELECT song_id FROM playlistSongs as X WHERE X.playlist_id={} AND X.song_id=playlistSongs.song_id)) ORDER BY id;", songs_count, playlist_id, playlist_id)).load::<Playlist>(&mut connection.unwrap());
    if results.is_err() {
        return Err(results.err().unwrap().to_string());
    }
    Ok(results.unwrap())
}

#[tauri::command]
fn query8() -> Result<Vec<Song>, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let results = sql_query("SELECT * FROM songs WHERE (SELECT COUNT(id) FROM playlists) = (SELECT DISTINCT COUNT(DISTINCT playlist_id) FROM playlistSongs WHERE song_id=songs.id) ORDER BY id;").load::<Song>(&mut connection.unwrap());
    if results.is_err() {
        return Err(results.err().unwrap().to_string());
    }
    Ok(results.unwrap())
}

#[tauri::command]
fn get_users() -> Result<Vec<User>, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let results =
        sql_query("SELECT * FROM users ORDER BY id;").load::<User>(&mut connection.unwrap());
    if results.is_err() {
        return Err(results.err().unwrap().to_string());
    }
    Ok(results.unwrap())
}

#[tauri::command]
fn get_tiers() -> Result<Vec<Tier>, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let results =
        sql_query("SELECT * FROM tiers ORDER BY id;").load::<Tier>(&mut connection.unwrap());
    if results.is_err() {
        return Err(results.err().unwrap().to_string());
    }
    Ok(results.unwrap())
}

#[tauri::command]
fn get_songs() -> Result<Vec<Song>, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let results =
        sql_query("SELECT * FROM songs ORDER BY id;").load::<Song>(&mut connection.unwrap());
    if results.is_err() {
        return Err(results.err().unwrap().to_string());
    }
    Ok(results.unwrap())
}

#[tauri::command]
fn get_playlists() -> Result<Vec<Playlist>, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let results = sql_query("SELECT * FROM playlists ORDER BY id;")
        .load::<Playlist>(&mut connection.unwrap());
    if results.is_err() {
        return Err(results.err().unwrap().to_string());
    }
    Ok(results.unwrap())
}

#[tauri::command]
fn get_links() -> Result<Vec<Link>, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let results = sql_query("SELECT * FROM playlistSongs ORDER BY playlist_id;")
        .load::<Link>(&mut connection.unwrap());
    if results.is_err() {
        return Err(results.err().unwrap().to_string());
    }
    Ok(results.unwrap())
}

#[tauri::command]
fn delete_link(playlist_id: i64, song_id: i64) -> Result<bool, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!(
        "DELETE FROM playlistSongs WHERE playlist_id={} AND song_id={}",
        playlist_id, song_id
    ))
    .load::<Playlist>(&mut connection.unwrap());
    Ok(result.is_ok())
}

#[tauri::command]
fn delete_tier(id: i64) -> Result<bool, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!("DELETE FROM tiers WHERE id={}", id))
        .load::<Tier>(&mut connection.unwrap());
    Ok(result.is_ok())
}

#[tauri::command]
fn delete_user(id: i64) -> Result<bool, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!("DELETE FROM users WHERE id={}", id))
        .load::<User>(&mut connection.unwrap());
    Ok(result.is_ok())
}

#[tauri::command]
fn delete_song(id: i64) -> Result<bool, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!("DELETE FROM songs WHERE id={}", id))
        .load::<Song>(&mut connection.unwrap());
    Ok(result.is_ok())
}

#[tauri::command]
fn delete_playlist(id: i64) -> Result<bool, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!("DELETE FROM playlists WHERE id={}", id))
        .load::<Playlist>(&mut connection.unwrap());
    Ok(result.is_ok())
}

#[tauri::command]
fn edit_tier(tier: Tier) -> Result<bool, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!(
        "UPDATE tiers SET title='{}', is_limited={}, price={} WHERE id={}",
        tier.title, tier.is_limited, tier.price, tier.id
    ))
    .load::<Playlist>(&mut connection.unwrap());
    Ok(result.is_ok())
}

#[tauri::command]
fn edit_user(user: User) -> Result<bool, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!(
        "UPDATE users SET nickname='{}', email='{}', password='{}', tier_id={} WHERE id={}",
        user.nickname, user.email, user.password, user.tier_id, user.id
    ))
    .load::<User>(&mut connection.unwrap());
    Ok(result.is_ok())
}

#[tauri::command]
fn edit_song(song: Song) -> Result<bool, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!(
        "UPDATE songs SET title='{}', times_played={}, user_id={} WHERE id={}",
        song.title, song.times_played, song.user_id, song.id
    ))
    .load::<Song>(&mut connection.unwrap());
    Ok(result.is_ok())
}

#[tauri::command]
fn edit_playlist(playlist: Playlist) -> Result<bool, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!(
        "UPDATE playlists SET title='{}', is_private={}, user_id={} WHERE id={}",
        playlist.title, playlist.is_private, playlist.user_id, playlist.id
    ))
    .load::<Playlist>(&mut connection.unwrap());
    Ok(result.is_ok())
}

#[tauri::command]
fn edit_link(old_link: Link, new_link: Link) -> Result<bool, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!(
        "UPDATE playlistSongs SET playlist_id='{}', song_id={} WHERE playlist_id={} AND song_id={}",
        new_link.playlist_id, new_link.song_id, old_link.playlist_id, old_link.song_id
    ))
    .load::<Link>(&mut connection.unwrap());
    Ok(result.is_ok())
}

#[tauri::command]
fn create_tier(tier: Tier) -> Result<i32, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!(
        "INSERT INTO tiers (title, price, is_limited) VALUES ('{}', {}, {}) RETURNING *;",
        tier.title, tier.price, tier.is_limited
    ))
    .get_result::<Tier>(&mut connection.unwrap());
    if result.is_err() {
        return Err(result.err().unwrap().to_string());
    }
    Ok(result.unwrap().id)
}

#[tauri::command]
fn create_user(user: User) -> Result<i32, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!(
        "INSERT INTO users (nickname, email, password, tier_id) VALUES ('{}', '{}', '{}', {}) RETURNING *;",
        user.nickname, user.email, user.password, user.tier_id
    ))
    .get_result::<User>(&mut connection.unwrap());
    if result.is_err() {
        return Err(result.err().unwrap().to_string());
    }
    Ok(result.unwrap().id)
}

#[tauri::command]
fn create_song(song: Song) -> Result<i32, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!(
        "INSERT INTO songs (title, times_played, user_id) VALUES ('{}', {}, {}) RETURNING *;",
        song.title, song.times_played, song.user_id
    ))
    .get_result::<Song>(&mut connection.unwrap());
    if result.is_err() {
        return Err(result.err().unwrap().to_string());
    }
    Ok(result.unwrap().id)
}

#[tauri::command]
fn create_playlist(playlist: Playlist) -> Result<i32, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!(
        "INSERT INTO playlists (title, is_private, user_id) VALUES ('{}', {}, {}) RETURNING *;",
        playlist.title, playlist.is_private, playlist.user_id
    ))
    .get_result::<Playlist>(&mut connection.unwrap());
    if result.is_err() {
        return Err(result.err().unwrap().to_string());
    }

    Ok(result.unwrap().id)
}

#[tauri::command]
fn create_link(link: Link) -> Result<bool, String> {
    let connection = PgConnection::establish(url_wrapper().get_mut().unwrap().url.as_str());

    if connection.is_err() {
        return Err(connection.err().unwrap().to_string());
    }

    let result = sql_query(format!(
        "INSERT INTO playlistSongs (playlist_id, song_id) VALUES ({}, {}) RETURNING *;",
        link.playlist_id, link.song_id
    ))
    .load::<Link>(&mut connection.unwrap());

    Ok(result.is_ok())
}

#[tauri::command]
fn init_db() -> Result<bool, String> {
    let read_result = read_to_string("data");
    if read_result.is_ok() {
        let string = read_result.unwrap();
        return connect_to_db(&string, false);
    }
    Ok(false)
}

#[tauri::command]
fn connect_to_db(connection_string: &str, rewrite: bool) -> Result<bool, String> {
    let db: ConnectionResult<PgConnection> = PgConnection::establish(connection_string);
    let return_flag = db.is_ok();
    if return_flag {
        let con = String::from(connection_string);
        let mut binding = url_wrapper();
        binding.get_mut().unwrap().change(con);
        if rewrite {
            let write_result = write(
                "data",
                binding.into_inner().unwrap().url.as_str().as_bytes(),
            );
            if write_result.is_err() {}
        }
    }
    Ok(return_flag)
}
