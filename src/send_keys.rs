use std::{mem, thread::sleep, time::Duration};
use winapi::um::winuser::{INPUT, INPUT_KEYBOARD, KEYEVENTF_EXTENDEDKEY, KEYEVENTF_KEYUP, KEYEVENTF_SCANCODE, SendInput};

use crate::chars::DXCode;
// key codes: https://docs.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes

unsafe fn create_input(key_code: u16, wvk: u16, flags: u32) -> INPUT {
    let mut input = mem::zeroed::<INPUT>();
    input.type_ = INPUT_KEYBOARD;
    let mut ki = input.u.ki_mut();
    ki.wVk = wvk;
    ki.wScan = key_code;
    ki.dwExtraInfo = 0;
    ki.dwFlags = flags;
    ki.time = 0;
    input
}

unsafe fn key_down(key_code: u16) {
    let mut input = create_input(key_code, 0, KEYEVENTF_SCANCODE);
    SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
}

unsafe fn key_up(key_code: u16) {
    let mut input = create_input(key_code, 0, KEYEVENTF_KEYUP);
    SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
}

unsafe fn special_down(key_code: u16) {
    let mut input = create_input(0, key_code, KEYEVENTF_EXTENDEDKEY);
    SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
}

unsafe fn special_up(key_code: u16) {
    let mut input = create_input(0, key_code, KEYEVENTF_EXTENDEDKEY|KEYEVENTF_KEYUP);
    SendInput(1, &mut input, mem::size_of::<INPUT>() as i32);
}

pub unsafe fn spam_keys(key_code: u16, timeout: u64, amount: i16) {
    for _ in 0..amount {
        key_enter(key_code, timeout);
        sleep(Duration::from_millis(timeout));
    }
}

pub unsafe fn key_enter(key_code: u16, timeout: u64) {
    key_down(key_code);
    sleep(Duration::from_millis(timeout));
    key_up(key_code);
}

pub unsafe fn send_string(keys: Vec<DXCode>) {
    for key in keys {
        match key {
            DXCode::Shifted(code)=>{
                sleep(Duration::from_millis(10));
                special_down(0x10);
                sleep(Duration::from_millis(10));
                key_enter(code, 8);
                sleep(Duration::from_millis(10));
                special_up(0x10);
                sleep(Duration::from_millis(10));
            },
            DXCode::Symbol(code) => key_enter(code, 8),
        }
    }
}