// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            delete_user,
            delete_song,
            delete_playlist,
            delete_playlist_entry
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn delete_playlist_entry(playlist_id: i64, song_id: i64) -> (String, bool) {
    let msg = format!("Song with id {} was deleted from playlist with id {}", song_id, playlist_id);
    (msg, true)
}

#[tauri::command]
fn delete_playlist(id: i64) -> (String, bool) {
    let msg = format!("Playlist with id {} was deleted", id);
    (msg, true)
}

#[tauri::command]
fn delete_song(id: i64) -> (String, bool) {
    let msg = format!("Song with id {} was deleted", id);
    (msg, true)
}

#[tauri::command]
fn delete_user(id: i64) -> (String, bool) {
    let msg = format!("User with id {} was deleted", id);
    (msg, true)
}
