use std::{
    sync::{atomic, Arc},
    thread::{sleep},
    time::Duration,
};
use std::io::Write;
use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use crate::actions;
use crate::structs;

pub fn anti_afk_runner(game_name: &str) {
    let mut run_once_no_game = true;
    let message_timeout = Arc::new(atomic::AtomicU32::new(0));

    Builder::new()
    .format(|buf, record| {
        writeln!(buf,
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
                message: "Join our discord, we are always recruiting: discord.gg/BoB".into(),
                message_start_time_utc: "12:00".into(),
                message_stop_time_utc: "23:00".into(),
                message_timeout_mins: 8,
            }
        }
    };
    confy::store_path("config.txt", cfg.clone()).unwrap();

    log::info!("Config loaded.");

    loop {
        let timeout = message_timeout.load(atomic::Ordering::Relaxed);
        if (timeout >= (cfg.message_timeout_mins)) && cfg.send_messages {
            log::info!("sending message...");
            actions::send_message(&cfg, game_name);
            message_timeout.store(0, atomic::Ordering::Relaxed);
        } else {
            run_once_no_game = actions::anti_afk(game_name, run_once_no_game);
            if cfg.send_messages {
                message_timeout.store(timeout + 1, atomic::Ordering::Relaxed);
            }
        }
        sleep(Duration::from_secs(60));
    };
}