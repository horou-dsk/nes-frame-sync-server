use chrono::Local;

pub mod router;
pub mod server;

pub fn setup_logger() -> Result<(), fern::InitError> {
    // let colors = ColoredLevelConfig::new().debug(Color::Magenta);
    let log_level = std::env::var("RUST_LOG");
    let log_level = match log_level {
        Ok(level) => {
            match level.as_str() {
                "info" => log::LevelFilter::Info,
                "error" => log::LevelFilter::Error,
                _ => log::LevelFilter::Debug,
            }
        },
        Err(_) => log::LevelFilter::Debug
    };
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}:{}] [{}] {}",
                // colors.color(record.level()),
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.target(),
                record.line().unwrap_or(0),
                record.level(),
                message
            ))
        })
        .level(log_level)
        .chain(std::io::stdout())
        // .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
