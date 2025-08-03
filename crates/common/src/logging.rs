use log::{info, Level};
use thiserror::Error;

#[derive(Debug)]
pub struct LoggingConfig {
    pub level: Level,
    pub json_format: bool,
}

#[derive(Debug, Error)]
pub enum LoggingError {
    #[error("logger initialization failed")]
    Init,
}

pub fn init_logging(config: &LoggingConfig) -> Result<(), LoggingError> {
    let level = config.level.to_level_filter();
    let format = if config.json_format {
        formatting::json_format
    } else {
        formatting::text_format
    };

    env_logger::Builder::new()
        .filter_level(level)
        .format(format)
        .target(env_logger::Target::Stdout)
        .try_init()
        .map_err(|_| LoggingError::Init)?;

    info!("Logging initialized with level: {}", level);
    Ok(())
}

pub mod formatting {
    use log::Record;
    use std::io::Write;

    pub fn json_format(
        buf: &mut env_logger::fmt::Formatter,
        record: &Record,
    ) -> std::io::Result<()> {
        writeln!(
            buf,
            "{{\"timestamp\":\"{}\",\"level\":\"{}\",\"target\":\"{}\",\"message\":\"{}\"}}",
            chrono::Utc::now().to_rfc3339(),
            record.level(),
            record.target(),
            record.args()
        )
    }

    pub fn text_format(
        buf: &mut env_logger::fmt::Formatter,
        record: &Record,
    ) -> std::io::Result<()> {
        writeln!(
            buf,
            "[{}] {}: {}",
            chrono::Utc::now().to_rfc3339(),
            record.level(),
            record.args()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initializes_logger() {
        let cfg = LoggingConfig {
            level: Level::Info,
            json_format: false,
        };
        let _ = init_logging(&cfg);
    }
}
