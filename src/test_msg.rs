mod actions;
mod chars;
mod send_keys;
mod shared_main;
mod structs;

fn main() {
    let cfg: structs::SeederConfig = match confy::load_path("config.txt") {
        Ok(config) => config,
        Err(e) => {
            println!("error in config.txt: {}", e);
            println!("changing back to default..");
            structs::SeederConfig {
                send_messages: true,
                message: "testmessage1".into(),
                message_start_time_utc: "12:00".into(),
                message_stop_time_utc: "23:00".into(),
                message_timeout_mins: 8,
            }
        }
    };
    confy::store_path("config.txt", cfg.clone()).unwrap();
    actions::send_message(&cfg, "Battlefield™ 2042");
}
