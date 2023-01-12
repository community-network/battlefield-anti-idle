use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::prelude::OsStrExt;
use std::ptr;
use std::thread::sleep;
use std::time::Duration;
use winapi::shared::windef::HWND__;
use winapi::um::winuser::{FindWindowW, SetForegroundWindow, ShowWindow, SendMessageW, GetForegroundWindow};

use crate::{send_keys, structs};
use crate::chars::{DXCode, char_to_dxcodes};

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

fn message_action(cfg: &structs::SeederConfig) {
    unsafe {
        send_keys::key_enter(0x24, 50); // J
        sleep(Duration::from_secs(3));
        let mut message: Vec<DXCode> = Vec::new();
        for char in cfg.message.chars() {
            if let Some(dx) = char_to_dxcodes(char) { message.push(dx) }
        }
        send_keys::send_string(message);
        sleep(Duration::from_secs(1));
        send_keys::key_enter(0x1C, 8); // ENTER
        sleep(Duration::from_secs(1));   
    }
}

fn bf2042_message_action(cfg: &structs::SeederConfig) {
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
        // println!("broadcast menu");
        send_keys::key_enter(0x39, 80); // SPACE
        sleep(Duration::from_secs(1));
        
        // println!("fill in");
        send_keys::spam_keys(0xC8, 8, 2); // UP
        sleep(Duration::from_secs(1));
        // println!("fill mode");
        send_keys::key_enter(0x39, 80); // SPACE
        sleep(Duration::from_secs(1));
        let mut message: Vec<DXCode> = Vec::new();
        for char in cfg.message.chars() {
            if let Some(dx) = char_to_dxcodes(char) { message.push(dx) }
        }
        send_keys::send_string(message);
        sleep(Duration::from_secs(1));
        // println!("done with message");
        send_keys::key_enter(0x1C, 80); // ENTER
        sleep(Duration::from_secs(1));
        // println!("done button");
        send_keys::key_enter(0xD0, 80); // DOWN
        sleep(Duration::from_secs(1));
        // println!("broadcast!");
        send_keys::key_enter(0x39, 80); // SPACE
        sleep(Duration::from_secs(1));
    
        // println!("Back to spawn screen");
        send_keys::spam_keys(0x01, 8, 2); // ESC
        sleep(Duration::from_secs(1));
    }
}

// https://gist.github.com/dretax/fe37b8baf55bc30e9d63
pub fn send_message(cfg: &structs::SeederConfig, game_name: &str) {
    let game_info = is_running(game_name);
    if game_info.is_running {
        unsafe {
            SetForegroundWindow(game_info.game_process);
            ShowWindow(game_info.game_process, 9);
            sleep(Duration::from_millis(1808));
            if game_name.is_empty() {
                bf2042_message_action(cfg);
            } else {
                message_action(cfg);
            }
            ShowWindow(game_info.game_process, 6);
        }
    }
}


pub fn is_running(game_name: &str) -> structs::GameInfo {
    unsafe {
        let window: Vec<u16> = OsStr::new(game_name)
            .encode_wide()
            .chain(once(0))
            .collect();
        let window_handle = FindWindowW(std::ptr::null_mut(), window.as_ptr());
        let no_game: *mut HWND__ = ptr::null_mut();
        structs::GameInfo {
            is_running: window_handle != no_game,
            game_process: window_handle,
        }
    }
}