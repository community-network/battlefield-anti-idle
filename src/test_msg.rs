mod actions;
mod chars;
mod send_keys;
mod structs;
use crate::structs::ChatType;
use std::sync::{atomic, Arc};

fn main() {
    let current_message_id = Arc::new(atomic::AtomicU32::new(0));
    let cfg: structs::SeederConfig = match confy::load_path("config.txt") {
        Ok(config) => config,
        Err(e) => {
            println!("error in config.txt: {}", e);
            println!("changing back to default..");
            structs::SeederConfig {
                send_messages: true,
                messages: vec!["testmessage1".into()],
                chat_type: ChatType::Public,
                message_start_time_utc: "12:00".into(),
                message_stop_time_utc: "23:00".into(),
                message_timeout_mins: 8,
            }
        }
    };
    confy::store_path("config.txt", cfg.clone()).unwrap();
    actions::send_message(&cfg, "Battlefield™ 2042", &current_message_id);
}
