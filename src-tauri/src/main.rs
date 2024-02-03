// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use aes_gcm::{
    aead::Aead,
    aead::{generic_array::GenericArray, OsRng},
    AeadCore, Aes256Gcm, Key, KeyInit,
};
use bcrypt::{hash, verify};
use ring::pbkdf2;
use rand::{thread_rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{prelude::*, SeekFrom, Write};
use std::num::NonZeroU32;
use std::path::Path;
use tauri::{
    CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, Size, LogicalSize,
};

const PBKDF2_ROUNDS: u32 = 100_000;
const SALT_LEN: usize = 16;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    data: Vec<u8>,
    nonce: Vec<u8>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Password {
    id: i32,
    name: String,
    username: String,
    password: Block,
    url: String,
    notes: String,
    decrypted_password: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct GeneratorOptions {
    min_lowercase: u32,
    min_uppercase: u32,
    min_numbers: u32,
    min_symbols: u32
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

    let mut folder_path = path.clone();
    for _ in "master_password.txt".chars() {
        folder_path.pop();
    }
    if !Path::new(&folder_path).exists() {
        std::fs::create_dir_all(&folder_path).unwrap();
    }
    path
}

fn derive_key(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA256,
        NonZeroU32::new(PBKDF2_ROUNDS).unwrap(),
        salt,
        password.as_bytes(),
        &mut key,
    );
    key
}

#[tauri::command]
fn get_passwords(master_password: String) -> Vec<Password> {
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

    let salt: [u8; SALT_LEN] = [0; SALT_LEN]; // constant salt should be enough for now
    let key = derive_key(&master_password, &salt);

    let mut contents_json: Vec<Password> = serde_json::from_str(&contents).unwrap();

    let mut passwords: Vec<Password> = Vec::new();
    for password in contents_json.iter_mut() {
        let encrypted_password = password.password.clone();
        let decrypted_password = decrypt(encrypted_password.clone(), &key);
        passwords.push(Password {
            id: password.id,
            name: password.name.to_string(),
            username: password.username.to_string(),
            password: encrypted_password,
            url: password.url.to_string(),
            notes: password.notes.to_string(),
            decrypted_password: Some(String::from_utf8(decrypted_password).unwrap()),
        });
    }

    passwords
}

#[tauri::command]
fn get_password(id: i32, master_password: String) -> Password {
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

    let salt: [u8; SALT_LEN] = [0; SALT_LEN]; // constant salt should be enough for now
    let key = derive_key(&master_password, &salt);

    let contents_json: Vec<Password> = serde_json::from_str(&contents).unwrap();

    let encrypted_password = contents_json[id as usize].password.clone();
    let password: Password = Password {
        id: contents_json[id as usize].id,
        name: contents_json[id as usize].name.to_string(),
        username: contents_json[id as usize].username.to_string(),
        password: encrypted_password.clone(),
        url: contents_json[id as usize].url.to_string(),
        notes: contents_json[id as usize].notes.to_string(),
        decrypted_password: Some(String::from_utf8(decrypt(encrypted_password, &key)).unwrap()),
    };

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
    master_password: String,
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

    let salt: [u8; SALT_LEN] = [0; SALT_LEN]; // constant salt should be enough for now
    let key = derive_key(&master_password, &salt);

    let mut contents_json: serde_json::Value = serde_json::from_str(&contents).unwrap();

    let mut password_to_edit = contents_json[id as usize].clone();
    let encrypted_password = encrypt(&password.trim().as_bytes().to_vec(), &key);

    password_to_edit["name"] = serde_json::Value::from(name);
    password_to_edit["username"] = serde_json::Value::from(username);
    password_to_edit["password"]["data"] = serde_json::Value::from(encrypted_password.data);
    password_to_edit["password"]["nonce"] = serde_json::Value::from(encrypted_password.nonce);
    password_to_edit["url"] = serde_json::Value::from(url);
    password_to_edit["notes"] = serde_json::Value::from(notes);

    contents_json[id as usize] = password_to_edit;

    let contents_string = serde_json::to_string(&contents_json).unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();

    file.write_all(contents_string.as_bytes()).unwrap();
    file.set_len(contents_string.len() as u64).unwrap();
}

#[tauri::command]
async fn open_add_password(app: tauri::AppHandle) {
    let _window = tauri::WindowBuilder::new(&app, "addpw", tauri::WindowUrl::App("addPw".into()))
        .build()
        .unwrap()
        .set_title("PassmanTRS - add password")
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
    .set_title("PassmanTRS - edit password")
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
    .set_title("PassmanTRS - Passworddetails")
    .map_err(|err| println!("{:?}", err)) // print error if the window fails to be created. Rust error handling 😍
    .ok();
}

#[tauri::command]
async fn open_generator(app: tauri::AppHandle) {
    let _window =
        tauri::WindowBuilder::new(&app, "generator", tauri::WindowUrl::App("generator".into()))
            .build()
            .unwrap()
            .set_title("PassmanTRS - Password Generator")
            .map_err(|err| println!("{:?}", err)) // print error if the window fails to be created. Rust error handling 😍
            .ok();
}

#[tauri::command]
fn add_password(
    name: String,
    username: String,
    password: String,
    url: String,
    notes: String,
    master_password: String,
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
    let mut passwords: Vec<Password> = if contents.is_empty() {
        vec![]
    } else {
        serde_json::from_str(&contents).unwrap()
    };

    let salt: [u8; SALT_LEN] = [0; SALT_LEN]; // constant salt should be enough for now
    let key = derive_key(&master_password, &salt);

    let password_block = encrypt(&password.trim().as_bytes().to_vec(), &key);

    passwords.push(Password {
        id: passwords.len() as i32,
        name,
        username,
        password: password_block,
        url,
        notes,
        decrypted_password: None,
    });

    let contents_string = serde_json::to_string(&passwords).unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();

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

    passwords.remove(id as usize);

    for (i, password) in passwords.iter_mut().enumerate() {
        password.id = i as i32;
    }

    let contents_string = serde_json::to_string(&passwords).unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();

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
}

#[tauri::command]
fn close_generator(app: tauri::AppHandle) {
    app.get_window("generator").unwrap().close().unwrap();
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

    file.seek(SeekFrom::Start(0)).unwrap();

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

fn encrypt(data: &[u8], key_byte: &[u8]) -> Block {
    let key: &Key<Aes256Gcm> = key_byte.into();
    let cipher = Aes256Gcm::new(&key);

    let nonce = Aes256Gcm::generate_nonce(OsRng);

    let encrypted_data = match cipher.encrypt(&nonce, data) {
        Ok(encrpted) => {
            let e = Block {
                data: encrpted,
                nonce: nonce.to_vec(),
            };

            e
        }
        Err(err) => {
            panic!("Could not encrypt data: {:?}", err);
        }
    };
    encrypted_data
}

fn decrypt(encrypted_data: Block, password_byte: &[u8]) -> Vec<u8> {
    let key: &Key<Aes256Gcm> = password_byte.into();
    let nonce = encrypted_data.nonce;
    let data = encrypted_data.data;

    let cipher = Aes256Gcm::new(&key);
    let op = cipher
        .decrypt(GenericArray::from_slice(&nonce), data.as_slice())
        .expect("decryption failure!");
    op
}

#[tauri::command]
fn change_master_password(old_pw: String, new_pw: String) -> bool {
    let mpw_path = get_master_password_file_path();
    let mut mpw_file = std::fs::OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(mpw_path.clone())
        .unwrap();
    let mut mpw_contents = String::new();
    std::io::Read::read_to_string(&mut mpw_file, &mut mpw_contents).unwrap();
    if mpw_contents.len() == 0 {
        return false;
    } else {
        if !verify(old_pw.as_bytes(), &mpw_contents).unwrap() {
            return false;
        }
    }

    let hashed_password = hash(new_pw.as_bytes(), 12).unwrap();
    mpw_file.seek(SeekFrom::Start(0)).unwrap();
    mpw_file.write_all(hashed_password.as_bytes()).unwrap();
    mpw_file.set_len(hashed_password.len() as u64).unwrap();

    migrate_passwords(old_pw, new_pw)
}

fn migrate_passwords(old_master_pw: String, new_master_pw: String) -> bool {
    let path = get_passwords_file_path();
    let mut file = std::fs::OpenOptions::new()
        .read(true)
        .create(true)
        .write(true)
        .open(path.clone())
        .unwrap();
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents).unwrap();
    if contents.len() == 0 {
        contents.push_str("[]");
        file.write_fmt(format_args!("{}", contents)).unwrap();
    }

    let salt: [u8; SALT_LEN] = [0; SALT_LEN]; // constant salt should be enough for now
    let old_key = derive_key(&old_master_pw, &salt);
    let new_key = derive_key(&new_master_pw, &salt);

    let mut contents_json: Vec<Password> = serde_json::from_str(&contents).unwrap();

    let mut passwords: Vec<Password> = Vec::new();
    for password in contents_json.iter_mut() {
        let encrypted_password = password.password.clone();
        let decrypted_password = decrypt(encrypted_password.clone(), &old_key);
        let encrypted_password = encrypt(&decrypted_password, &new_key);
        passwords.push(Password {
            id: password.id,
            name: password.name.to_string(),
            username: password.username.to_string(),
            password: encrypted_password,
            url: password.url.to_string(),
            notes: password.notes.to_string(),
            decrypted_password: None,
        });
    }

    let contents_string = serde_json::to_string(&passwords).unwrap();

    file.seek(SeekFrom::Start(0)).unwrap();

    file.write_all(contents_string.as_bytes()).unwrap();
    file.set_len(contents_string.len() as u64).unwrap();
    true
}

#[tauri::command]
fn generate_password(length: u32, options: GeneratorOptions) -> String {
    println!("Generating password with length {} and options {:?}", length, options);
    let lowercase_chars: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];

    let uppercase_chars: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ];

    let number_chars: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

    let symbol_chars: [char; 32] = [
        '!', '"', '#', '$', '%', '&', '\'', '(', ')', '*', '+', ',', '-', '.', '/', ':', ';', '<',
        '=', '>', '?', '@', '[', '\\', ']', '^', '_', '`', '{', '|', '}', '~'
    ];

    let mut password = String::with_capacity(length as usize);

    for _ in 0..options.min_lowercase {
        password.push(*lowercase_chars.choose(&mut thread_rng()).unwrap());
    }
    for _ in 0..options.min_uppercase {
        password.push(*uppercase_chars.choose(&mut thread_rng()).unwrap());
    }
    for _ in 0..options.min_numbers {
        password.push(*number_chars.choose(&mut thread_rng()).unwrap());
    }
    for _ in 0..options.min_symbols {
        password.push(*symbol_chars.choose(&mut thread_rng()).unwrap());
    }

    for _ in 0..(length - password.len() as u32) {
        let mut all_chars: Vec<char> = Vec::new();
        all_chars.extend_from_slice(&lowercase_chars);
        all_chars.extend_from_slice(&uppercase_chars);
        all_chars.extend_from_slice(&number_chars);
        all_chars.extend_from_slice(&symbol_chars);
        let all_chars = all_chars;
        password.push(*all_chars.choose(&mut thread_rng()).unwrap());
    }

    let mut chars = password.chars().collect::<Vec<_>>();
    chars.shuffle(&mut thread_rng());
    password = chars.into_iter().collect();
    println!("Generated password: {}", password);
    password
}

fn main() {
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let generator = CustomMenuItem::new("generator".to_string(), "Password Generator");
    let about = CustomMenuItem::new("about".to_string(), "About");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let tray_menu = SystemTrayMenu::new()
        .add_item(hide)
        .add_item(generator)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(about)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);
    let tray = SystemTray::new().with_menu(tray_menu);
    tauri::Builder::default()
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => {
                let item_handle = app.tray_handle().get_item(&id);
                if id == "quit" {
                    app.exit(0);
                } else if id == "hide" {
                    if app.get_window("main").unwrap().is_visible().unwrap() {
                        app.get_window("main").unwrap().hide().unwrap();
                        item_handle.set_title("Show").unwrap();
                    } else {
                        app.get_window("main").unwrap().show().unwrap();
                        item_handle.set_title("Hide").unwrap();
                    }
                } else if id == "generator" {
                    let _window = tauri::WindowBuilder::new(
                        app,
                        "generator",
                        tauri::WindowUrl::App("generator".into()),
                    )
                    .build()
                    .unwrap()
                    .set_title("PassmanTRS - Password Generator")
                    .map_err(|err| println!("{:?}", err))
                    .ok();
                } else if id == "about" {
                    let window = tauri::WindowBuilder::new(
                        app,
                        "about",
                        tauri::WindowUrl::App("about".into()),
                    )
                    .build()
                    .unwrap();
                    window
                        .set_size(Size::Logical(LogicalSize {
                            width: 600.0,
                            height: 250.0,
                        }))
                        .unwrap();
                    window
                        .set_title("PassmanTRS - About")
                        .map_err(|err| println!("{:?}", err))
                        .ok();
                    window.set_resizable(false).unwrap();
                    window.show().unwrap();
                } else {
                    println!("Unknown menu item clicked: {}", id);
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            get_passwords,
            get_password,
            edit_password,
            open_add_password,
            open_edit_password,
            open_view_password,
            open_generator,
            add_password,
            delete_password,
            close_add_password,
            close_edit_password,
            close_view_password,
            close_generator,
            validate_master_password,
            change_master_password,
            generate_password
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
