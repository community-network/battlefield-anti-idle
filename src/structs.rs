use serde_derive::{Deserialize, Serialize};
use winapi::shared::windef::HWND__;

#[derive(Serialize, Deserialize, Clone)]
pub enum ChatType {
    Announce,
    Public,
    Team,
    Squad,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SeederConfig {
    pub send_messages: bool,
    pub minimize_after_action: bool,
    pub messages: Vec<String>,
    pub chat_type: ChatType,
    pub message_start_time_utc: String,
    pub message_stop_time_utc: String,
    pub message_timeout_mins: u32,
    pub keypress_mode: bool,
    pub key: String,
    pub key_hold_time: u64,
}

pub struct GameInfo {
    pub is_running: bool,
    pub game_process: *mut HWND__,
}

/// `SeederConfig` implements `Default`
impl ::std::default::Default for SeederConfig {
    fn default() -> Self {
        Self {
            send_messages: false,
            minimize_after_action: true,
            messages: vec!["Join our discord, we are always recruiting: discord.gg/BoB".into()],
            chat_type: ChatType::Public,
            message_start_time_utc: "12:00".into(),
            message_stop_time_utc: "23:00".into(),
            message_timeout_mins: 8,
            keypress_mode: false,
            key: "tab".into(),
            key_hold_time: 80,
        }
    }
}
