use crate::actions;
use crate::structs;
use crate::structs::ChatType;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;
use std::{
    sync::{atomic, Arc},
    thread::sleep,
    time::Duration,
};

pub fn anti_afk_runner(game_name: &str) {
    let mut run_once_no_game = true;
    let message_timeout = Arc::new(atomic::AtomicU32::new(0));
    let current_message_id = Arc::new(atomic::AtomicU32::new(0));

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    log::info!("Script started.");

    let cfg: structs::SeederConfig = match confy::load_path("config.txt") {
        Ok(config) => config,
        Err(e) => {
            println!("error in config.txt: {}", e);
            println!("changing back to default..");
            structs::SeederConfig {
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
    };
    confy::store_path("config.txt", cfg.clone()).unwrap();

    log::info!("Config loaded.");

    loop {
        let timeout = message_timeout.load(atomic::Ordering::Relaxed);
        if (timeout >= (cfg.message_timeout_mins)) && cfg.send_messages {
            log::info!("sending message...");
            actions::send_message(&cfg, game_name, &current_message_id);
            message_timeout.store(0, atomic::Ordering::Relaxed);
        } else {
            run_once_no_game = actions::anti_afk(&cfg, game_name, run_once_no_game);
            if cfg.send_messages {
                message_timeout.store(timeout + 1, atomic::Ordering::Relaxed);
            }
        }
        sleep(Duration::from_secs(60));
    }
}
