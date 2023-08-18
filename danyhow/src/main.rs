use std::fs::{
    self,
    File,
};

use anyhow::{
    Context,
    Error,
};
use log;
use simplelog::{
    CombinedLogger,
    Config,
    SimpleLogger,
    TermLogger,
    WriteLogger,
};

fn main() {
    if let Err(e) = CombinedLogger::init(vec![
        SimpleLogger::new(
            log::LevelFilter::Trace,
            Config::default(),
        ),
        TermLogger::new(
            log::LevelFilter::Trace,
            Config::default(),
            simplelog::TerminalMode::Mixed,
            simplelog::ColorChoice::Auto,
        ),
        WriteLogger::new(
            log::LevelFilter::Info,
            Config::default(),
            File::create("my.log").expect("cant't crate log file"),
        ),
    ]) {
        println!("Failed to init logger: {:?}", e);
        return;
    };

    if let Err(e) = get_cluster_info() {
        log::error!("{:?}", e);
    }
}

fn get_cluster_info() -> Result<(), Error> {
    let config = fs::read_to_string("cluster.json")
        .with_context(|| "cant't open file")?;

    return Ok(());
}
