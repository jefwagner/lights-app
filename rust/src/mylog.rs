use anyhow::{Context, Result};
use syslog::{Facility, Formatter3164, BasicLogger};
use log::LevelFilter;

pub fn init_log(log_level: LevelFilter) -> Result<()> {
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "lights".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter)
    .context("Failed to connect to syslog")?;

    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
    .context("Failed to initialize logging")?;

    log::set_max_level(log_level);

    Ok(())
}