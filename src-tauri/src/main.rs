#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
//Database requirements
pub mod data;
//application setups
pub mod settings;

extern crate walkdir;
use std::path::Path;
extern crate photon_rs;
use data::{
    count_synced, get_child, get_child_count, get_child_count_psfilter, get_child_ids,
    get_children_psfilter, get_children_unsynced, get_selected_id, push_children, update_accept,
    update_reject, update_synced, RecordsResponse, decrypt
};
use photon_rs::native::open_image;
use settings::{get_to_server, save_to_server};
use walkdir::WalkDir;

fn is_path_exist(path: &str) -> bool {
    Path::new(path).exists()
}
#[tauri::command]
fn get_photo_src(src_path: &str) -> String {
    let img = open_image(src_path).expect("File should open");

    format!("{}", img.get_base64())
}
#[tauri::command]
fn get_file_list(folder_path: &str) -> Vec<String> {
    println!("Rust receive req: {}", folder_path);
    let mut v: Vec<String> = Vec::new();
    if is_path_exist(folder_path) {
        for file in WalkDir::new(folder_path)
            .into_iter()
            .filter_map(|file| file.ok())
        {
            if file.metadata().unwrap().is_file() {
                let path_str = file.path().display().to_string();
                if let Some(ext) = Path::new(&path_str).extension() {
                    if ext.to_str().unwrap() == "JPG" {
                        //println!("extension: {:?}",ext);
                        v.push(path_str);
                    }
                }
            }
        }
        println!("Done searching {} file", v.len());
    }

    v
}
#[tauri::command]
fn get_photo_paths(child_id: i32, folder_path: &str) -> Vec<String> {
    let directory = Path::new(folder_path);
    let mut paths: Vec<String> = Vec::new();
    for entry in WalkDir::new(directory).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() {
            continue;
        }

        let file_name = entry.file_name().to_string_lossy();

        if file_name.starts_with(&format!("{}_", child_id)) {
            paths.push(entry.path().to_string_lossy().into_owned());
        }
    }
    paths
}
#[tauri::command]
fn retrieve_user_records(api_path: &str) -> usize {
    println!("Retrive data called. API Path: {}", api_path);
    if let Ok(body) = reqwest::blocking::get(api_path) {
        if let Ok(recs) = body.json::<RecordsResponse>() {
            println!("Num row: {:?}", recs.rows.len());
            return push_children(recs.rows).unwrap();
        }
    }
    println!("Retrieve Done!");
    0
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_file_list,
            get_photo_src,
            get_photo_paths,
            retrieve_user_records,
            get_child,
            get_child_ids,
            get_selected_id,
            update_accept,
            update_reject,
            get_child_count,
            get_child_count_psfilter,
            get_to_server,
            save_to_server,
            get_children_psfilter,
            get_children_unsynced,
            update_synced,
            count_synced,
            decrypt
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
