#[allow(unused_imports)]
use anyhow::{Context, Result};
use log::LevelFilter;

#[cfg(debug_assertions)]
pub fn init_log(log_level: LevelFilter) -> Result<()> {
    let mut clog = colog::default_builder();
    clog.filter(None, log_level);
    clog.init();

    log::set_max_level(log_level);

    Ok(())
}

#[cfg(not(debug_assertions))]
use syslog::{Facility, Formatter3164, BasicLogger};

#[cfg(not(debug_assertions))]
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
