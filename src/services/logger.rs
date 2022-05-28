use simplelog::*;
use std::fs::File;

pub struct Logger {}

impl Logger {
    pub fn init() {
        CombinedLogger::init(
            vec![
                TermLogger::new(
                    LevelFilter::Info,
                    Config::default(),
                    TerminalMode::Mixed,
                    ColorChoice::Auto,
                ),
                WriteLogger::new(
                    LevelFilter::Info,
                    Config::default(),
                    File::create("./logs/app.log").unwrap(),
                ),
                WriteLogger::new(
                    LevelFilter::Debug,
                    Config::default(),
                    File::create("./logs/debug.log").unwrap(),
                ),
            ]
        ).unwrap();
    }
}
