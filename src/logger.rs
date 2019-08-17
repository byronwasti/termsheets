use chrono;
use fern;
use log::debug;
use std;

pub fn init() -> Result<(), fern::InitError> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            if record.level() == log::LevelFilter::Debug {
                out.finish(format_args!(
                    "[{}][{}][{}] {}",
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    record.level(),
                    record.target(),
                    message
                ))
            } else {
                out.finish(format_args!("{}", message))
            }
        })
        .level(log::LevelFilter::Debug)
        .chain(fern::log_file("logs/output.log")?)
        .apply()?;

    debug!("Logger initialized");
    Ok(())
}
