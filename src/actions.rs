use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::once;
use std::os::windows::prelude::OsStrExt;
use std::ptr;
use std::sync::atomic::{self, AtomicU32};
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use winapi::shared::windef::HWND__;
use winapi::um::winuser::{
    FindWindowW, GetForegroundWindow, SendMessageW, SetForegroundWindow, ShowWindow,
};

use crate::chars::{char_to_dxcodes, DXCode};
use crate::{send_keys, structs};

fn make_l_param(lo_word: i32, hi_word: i32) -> i32 {
    (hi_word << 16) | (lo_word & 0xffff)
}

pub fn anti_afk(game_name: &str, mut run_once_no_game: bool) -> bool {
    let game_info = is_running(game_name);
    if game_info.is_running {
        unsafe {
            let current_forground_window = GetForegroundWindow();
            let l_param = make_l_param(20, 20);
            SendMessageW(game_info.game_process, 0x201, 0, l_param as isize);
            SendMessageW(game_info.game_process, 0x202, 0, l_param as isize);
            SetForegroundWindow(current_forground_window);
            // reset no game check
            run_once_no_game = true;
        }
        log::info!("Running anti-idle for {}.", game_name);
    } else if run_once_no_game {
        log::info!("No game found, idleing...");
        run_once_no_game = false;
    }
    run_once_no_game
}

fn message_action(message_to_send: &str, open_chat_key: u16) {
    unsafe {
        send_keys::key_enter(open_chat_key, 50); // J
        sleep(Duration::from_secs(3));
        let mut current_message: Vec<DXCode> = Vec::new();
        for char in message_to_send.chars() {
            if let Some(dx) = char_to_dxcodes(char) {
                current_message.push(dx)
            }
        }
        send_keys::send_string(current_message);
        sleep(Duration::from_secs(1));
        send_keys::key_enter(0x1C, 8); // ENTER
        sleep(Duration::from_secs(1));
    }
}

fn bf2042_message_action(message_to_send: &str) {
    unsafe {
        // println!("Open pause menu");
        send_keys::key_enter(0x01, 80); // ESC
        sleep(Duration::from_secs(1));
        // println!("Most left menu");
        send_keys::spam_keys(0x10, 80, 3); // Q
        sleep(Duration::from_secs(1));
        // println!("Top of list");
        send_keys::spam_keys(0xD0, 80, 5); // DOWN
        sleep(Duration::from_secs(1));
        // println!("from bottom second item");
        send_keys::key_enter(0xC8, 80); // UP
        sleep(Duration::from_secs(1));
        // println!("Click!");
        send_keys::key_enter(0x39, 80); // SPACE
        sleep(Duration::from_secs(1));
        // println!("Move to top item for broadcast menu!");
        send_keys::spam_keys(0xC8, 80, 5); // UP
        sleep(Duration::from_secs(1));
        // println!("broadcast menu");
        send_keys::key_enter(0x39, 80); // SPACE
        sleep(Duration::from_secs(1));
        // println!("fill in");
        send_keys::spam_keys(0xC8, 8, 2); // UP
        sleep(Duration::from_secs(1));
        // println!("fill mode");
        send_keys::key_enter(0x39, 80); // SPACE
        sleep(Duration::from_secs(1));
        let mut current_message: Vec<DXCode> = Vec::new();
        for char in message_to_send.chars() {
            if let Some(dx) = char_to_dxcodes(char) {
                current_message.push(dx)
            }
        }
        send_keys::send_string(current_message);
        sleep(Duration::from_secs(1));
        // println!("done with message");
        send_keys::key_enter(0x1C, 80); // ENTER
        sleep(Duration::from_secs(1));
        // println!("Back to spawn screen");
        send_keys::spam_keys(0x01, 8, 3); // ESC
        sleep(Duration::from_secs(1));
    }
}

fn get_config_key(chat: &str, defaults: u16) -> u16 {
    let docs = dirs_next::document_dir();
    match docs {
        Some(docs) => {
            match File::open(format!(
                "{}\\Battlefield 2042\\settings\\PROFSAVE_profile",
                docs.to_str().unwrap()
            )) {
                Ok(f) => {
                    let f = BufReader::new(f);
                    for line in f.lines() {
                        let line = line.unwrap_or_default();
                        if line.starts_with(&format!("GstKeyBinding.default.{}.0.button", chat)) {
                            return match line.split(' ').last().unwrap_or("").parse() {
                                Ok(key_code) => key_code,
                                Err(_) => {
                                    log::error!("Keycode in config invalid, using defaults...");
                                    defaults
                                }
                            };
                        }
                    }
                    log::error!("Keybind not found in config of game, using defaults...");
                    defaults
                }
                Err(_) => {
                    log::error!("Cant open profsave of bf2042 at \"{}\\Battlefield 2042\\settings\\PROFSAVE_profile\", using defaults...", docs.to_str().unwrap());
                    defaults
                }
            }
        }
        None => {
            log::error!("Can't find documents folder, using defaults...");
            defaults
        }
    }
}

// https://gist.github.com/dretax/fe37b8baf55bc30e9d63
pub fn send_message(
    cfg: &structs::SeederConfig,
    game_name: &str,
    current_message_id: &Arc<AtomicU32>,
) {
    let game_info = is_running(game_name);
    if game_info.is_running {
        let mut message_id = current_message_id.load(atomic::Ordering::Relaxed);
        let current_message: &String = &cfg.messages[message_id as usize];

        unsafe {
            SetForegroundWindow(game_info.game_process);
            ShowWindow(game_info.game_process, 9);
            sleep(Duration::from_millis(1808));
            let squad_key = match game_name == "Battlefield™ 2042" {
                true => get_config_key("GstKeyBinding.default.ConceptSquadChat.0.button", 0x26),
                false => 0x26,
            };
            let team_key = match game_name == "Battlefield™ 2042" {
                true => get_config_key("GstKeyBinding.default.ConceptTeamChat.0.button", 0x25),
                false => 0x25,
            };

            match cfg.chat_type {
                structs::ChatType::Announce => {
                    if game_name == "Battlefield™ 2042" {
                        bf2042_message_action(current_message);
                    } else {
                        message_action(current_message, 0x24);
                    }
                }
                structs::ChatType::Public => {
                    if game_name == "Battlefield™ 2042" {
                        send_keys::key_enter(squad_key, 50);
                        sleep(Duration::from_secs(3));
                        let tab_key =
                            get_config_key("GstKeyBinding.default.ConceptChat.1.button", 0x0F);
                        message_action(current_message, tab_key);
                    } else {
                        message_action(current_message, squad_key);
                    }
                }
                structs::ChatType::Team => message_action(current_message, team_key),
                structs::ChatType::Squad => message_action(current_message, squad_key),
            }

            if cfg.minimize_after_message {
                ShowWindow(game_info.game_process, 6);
            }
        }

        if message_id + 1 >= cfg.messages.len() as u32 {
            message_id = 0;
        } else {
            message_id += 1;
        }

        // save
        current_message_id.store(message_id, atomic::Ordering::Relaxed);
    }
}

pub fn is_running(game_name: &str) -> structs::GameInfo {
    unsafe {
        let window: Vec<u16> = OsStr::new(game_name).encode_wide().chain(once(0)).collect();
        let window_handle = FindWindowW(std::ptr::null_mut(), window.as_ptr());
        let no_game: *mut HWND__ = ptr::null_mut();
        structs::GameInfo {
            is_running: window_handle != no_game,
            game_process: window_handle,
        }
    }
}
