// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use serde::{Deserialize, Serialize};
use serde_json;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::{io::Write, path::Path};
use tauri::Manager;
use bcrypt::{hash, verify};

// Password struct
#[derive(Debug, Serialize, Deserialize)]
struct Password {
    id: i32,
    name: String,
    username: String,
    password: String,
    url: String,
    notes: String,
}

fn get_os() -> String {
    let os = std::env::consts::OS;
    os.to_string()
}

fn get_passwords_file_path() -> String {
    let is_windows = get_os() == "windows";
    let path = if is_windows {
        let mut path = Path::new(&std::env::var("APPDATA").unwrap())
            .to_str()
            .unwrap()
            .to_string();
        path.push_str("\\passmantrs\\passwords.json");
        path
    } else {
        let mut path = String::new();
        path.push_str(&std::env::var("HOME").unwrap());
        path.push_str("/.config/passmantrs/passwords.json");
        path
    };
    // check if folder exists and create if it doesn't
    let mut folder_path = path.clone();
    for _ in "passwords.json".chars() {
        folder_path.pop();
    }
    if !Path::new(&folder_path).exists() {
        std::fs::create_dir_all(&folder_path).unwrap();
    }
    path
}

fn get_master_password_file_path() -> String {
    let is_windows = get_os() == "windows";
    let path = if is_windows {
        let mut path = Path::new(&std::env::var("APPDATA").unwrap())
            .to_str()
            .unwrap()
            .to_string();
        path.push_str("\\passmantrs\\master_password.txt");
        path
    } else {
        let mut path = String::new();
        path.push_str(&std::env::var("HOME").unwrap());
        path.push_str("/.config/passmantrs/master_password.txt");
        path
    };
    // check if folder exists and create if it doesn't
    let mut folder_path = path.clone();
    for _ in "master_password.txt".chars() {
        folder_path.pop();
    }
    if !Path::new(&folder_path).exists() {
        std::fs::create_dir_all(&folder_path).unwrap();
    }
    path
}

// returns a list of passwords as a string array after being decrypted
#[tauri::command]
fn get_passwords() -> Vec<Password> {
    let path = get_passwords_file_path();
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(path)
        .unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    if contents.len() == 0 {
        contents.push_str("[]");
        file.write_fmt(format_args!("{}", contents)).unwrap();
    }
    // create contentsJson variable to store contents as json
    let mut contents_json: serde_json::Value = serde_json::from_str(&contents).unwrap();
    // decrypt passwords
    let mut passwords: Vec<Password> = Vec::new();
    for password in contents_json.as_array_mut().unwrap() {
        let decrypted_password = decrypt(password["password"].to_string());
        passwords.push(Password {
            id: password["id"].to_string().parse::<i32>().unwrap(),
            name: password["name"].to_string(),
            username: password["username"].to_string(),
            password: decrypted_password,
            url: password["url"].to_string(),
            notes: password["notes"].to_string(),
        });
    }
    // return passwords
    passwords
}

#[tauri::command]
fn get_password(id: i32) -> Password {
    let path = get_passwords_file_path();
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(path)
        .unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    if contents.len() == 0 {
        contents.push_str("[]");
        file.write_fmt(format_args!("{}", contents)).unwrap();
    }
    // create contentsJson variable to store contents as json
    let contents_json: serde_json::Value = serde_json::from_str(&contents).unwrap();
    // decrypt password
    let password: Password = Password {
        id: contents_json[id as usize]["id"]
            .to_string()
            .parse::<i32>()
            .unwrap(),
        name: contents_json[id as usize]["name"].to_string(),
        username: contents_json[id as usize]["username"].to_string(),
        password: decrypt(contents_json[id as usize]["password"].to_string()),
        url: contents_json[id as usize]["url"].to_string(),
        notes: contents_json[id as usize]["notes"].to_string(),
    };
    // return password
    password
}

#[tauri::command]
fn edit_password(
    id: i32,
    name: String,
    username: String,
    password: String,
    url: String,
    notes: String,
) {
    let path = get_passwords_file_path();
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(path)
        .unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    if contents.len() == 0 {
        contents.push_str("[]");
        file.write_fmt(format_args!("{}", contents)).unwrap();
    }
    // create contentsJson variable to store contents as json
    let mut contents_json: serde_json::Value = serde_json::from_str(&contents).unwrap();
    // find password to edit by id
    let mut password_to_edit = contents_json[id as usize].clone();
    // edit password
    password_to_edit["name"] = serde_json::Value::from(name);
    password_to_edit["username"] = serde_json::Value::from(username);
    password_to_edit["password"] = serde_json::Value::from(encrypt(password));
    password_to_edit["url"] = serde_json::Value::from(url);
    password_to_edit["notes"] = serde_json::Value::from(notes);
    // replace password in contentsJson
    contents_json[id as usize] = password_to_edit;
    // convert contentsJson back to string
    let contents_string = serde_json::to_string(&contents_json).unwrap();
    // set cursor to 0
    file.seek(SeekFrom::Start(0)).unwrap();
    // write contents to file
    file.write_all(contents_string.as_bytes()).unwrap();
    file.set_len(contents_string.len() as u64).unwrap();
}

#[tauri::command]
async fn open_add_password(app: tauri::AppHandle) {
    let _window = tauri::WindowBuilder::new(&app, "addpw", tauri::WindowUrl::App("addPw".into()))
        .build()
        .unwrap()
        .set_title("Passwort hinzufügen")
        .map_err(|err| println!("{:?}", err)) // print error if the window fails to be created. Rust error handling 😍
        .ok();
}

#[tauri::command]
async fn open_edit_password(app: tauri::AppHandle, id: i32) {
    let _window = tauri::WindowBuilder::new(
        &app,
        "editpw",
        tauri::WindowUrl::App(format!("editPw?id={}", id).into()),
    )
    .build()
    .unwrap()
    .set_title("Passwort bearbeiten")
    .map_err(|err| println!("{:?}", err)) // print error if the window fails to be created. Rust error handling 😍
    .ok();
}

#[tauri::command]
async fn open_view_password(app: tauri::AppHandle, id: i32) {
    let _window = tauri::WindowBuilder::new(
        &app,
        "viewpw",
        tauri::WindowUrl::App(format!("viewPw?id={}", id).into()),
    )
    .build()
    .unwrap()
    .set_title("Passwortdetails")
    .map_err(|err| println!("{:?}", err)) // print error if the window fails to be created. Rust error handling 😍
    .ok();
}

#[tauri::command]
fn add_password(name: String, username: String, mut password: String, url: String, notes: String) {
    let path = get_passwords_file_path();
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(path)
        .unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    let mut passwords: Vec<Password> = if contents.is_empty() {
        vec![]
    } else {
        serde_json::from_str(&contents).unwrap()
    };
    // encrypt password
    password = encrypt(password);
    // add new password to the vector
    passwords.push(Password {
        id: passwords.len() as i32,
        name,
        username,
        password,
        url,
        notes,
    });
    // serialize the vector to json and write it back to the file
    let contents_string = serde_json::to_string(&passwords).unwrap();
    // set cursor to 0
    file.seek(SeekFrom::Start(0)).unwrap();
    // write contents to file
    file.write_all(contents_string.as_bytes()).unwrap();
    file.set_len(contents_string.len() as u64).unwrap();
}

#[tauri::command]
fn delete_password(id: i32) {
    let path = get_passwords_file_path();
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(path)
        .unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    let mut passwords: Vec<Password> = if contents.is_empty() {
        vec![]
    } else {
        serde_json::from_str(&contents).unwrap()
    };
    // remove password from vector
    passwords.remove(id as usize);
    //reset ids
    for (i, password) in passwords.iter_mut().enumerate() {
        password.id = i as i32;
    }
    // serialize the vector to json and write it back to the file
    let contents_string = serde_json::to_string(&passwords).unwrap();
    // set cursor to 0
    file.seek(SeekFrom::Start(0)).unwrap();
    // write contents to file
    file.write_all(contents_string.as_bytes()).unwrap();
    file.set_len(contents_string.len() as u64).unwrap();
}

#[tauri::command]
fn close_add_password(app: tauri::AppHandle) {
    app.get_window("addpw").unwrap().close().unwrap();
    app.emit_all("refresh_passwords", ()).unwrap();
}

#[tauri::command]
fn close_edit_password(app: tauri::AppHandle) {
    app.get_window("editpw").unwrap().close().unwrap();
    app.emit_all("refresh_passwords", ()).unwrap();
}

#[tauri::command]
fn close_view_password(app: tauri::AppHandle) {
    app.get_window("viewpw").unwrap().close().unwrap();
    // app.emit_all("refresh_passwords", ()).unwrap();
}

#[tauri::command]
fn delete_passwords() {
    let path = get_passwords_file_path();
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(path)
        .unwrap();
    let passwords: Vec<Password> = vec![];
    let contents_string = serde_json::to_string(&passwords).unwrap();
    // set cursor to 0
    file.seek(SeekFrom::Start(0)).unwrap();
    // write contents to file
    file.write_all(contents_string.as_bytes()).unwrap();
    file.set_len(contents_string.len() as u64).unwrap();
}

#[tauri::command]
fn validate_master_password(password: String) -> bool {
    let path = get_master_password_file_path();
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(path)
        .unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    if contents.is_empty() {
        delete_passwords();
        let hashed_password = hash(password.as_bytes(), 12).unwrap();
        file.write_all(hashed_password.as_bytes()).unwrap();
        file.set_len(hashed_password.len() as u64).unwrap();
        return true;
    }
    if verify(password.as_bytes(), &contents).unwrap() {
        return true;
    } else {
        return false;
    }
}

fn encrypt(password: String) -> String {
    password
}

fn decrypt(password: String) -> String {
    password
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_passwords,
            get_password,
            edit_password,
            open_add_password,
            open_edit_password,
            open_view_password,
            add_password,
            delete_password,
            close_add_password,
            close_edit_password,
            close_view_password,
            validate_master_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
