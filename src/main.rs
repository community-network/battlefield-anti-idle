use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::prelude::OsStrExt;
use std::ptr;
use std::{
    thread::sleep,
    time::Duration,
};
use winapi::shared::windef::HWND__;
use winapi::um::winuser::{FindWindowW, SetForegroundWindow, SendMessageW, GetForegroundWindow};

struct GameInfo {
    is_running: bool,
    game_process: *mut HWND__
}

fn main() {
    loop {
        let game_info = is_running();
        if game_info.is_running {
            unsafe {
                let current_forground_window = GetForegroundWindow();
                let l_param = make_l_param(20, 20);
                SendMessageW(game_info.game_process, 0x201, 0, l_param as isize);
                SendMessageW(game_info.game_process, 0x202, 0, l_param as isize);
                SetForegroundWindow(current_forground_window);
            }
        }
        sleep(Duration::from_secs(120));
    };
}

fn make_l_param(lo_word: i32, hi_word: i32) -> i32 {
    return (hi_word << 16) | (lo_word & 0xffff);
}

fn is_running() -> GameInfo {
    unsafe {
        let window: Vec<u16> = OsStr::new("Battlefield™ 1")
            .encode_wide()
            .chain(once(0))
            .collect();
        let window_handle = FindWindowW(std::ptr::null_mut(), window.as_ptr());
        let no_game: *mut HWND__ = ptr::null_mut();
        GameInfo{ is_running: window_handle != no_game, game_process: window_handle }
    }
}