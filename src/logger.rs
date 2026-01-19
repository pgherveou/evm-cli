use std::fs;
use std::path::PathBuf;

/// Get the log file path: ~/.evm-cli/output.log
fn log_path() -> Option<PathBuf> {
    let home = std::env::var_os("HOME")?;
    Some(PathBuf::from(home).join(".evm-cli/output.log"))
}

/// Initialize the logger to write to ~/.evm-cli/output.log
pub fn init() -> Result<(), fern::InitError> {
    let Some(log_file_path) = log_path() else {
        // If we can't determine home directory, just use env_logger as fallback
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn"))
            .format_timestamp(None)
            .format_target(false)
            .init();
        return Ok(());
    };

    // Ensure the directory exists
    if let Some(parent) = log_file_path.parent() {
        let _ = fs::create_dir_all(parent);
    }

    // Configure fern to write to the log file
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{}] {}: {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .chain(fern::log_file(log_file_path)?)
        .apply()?;

    Ok(())
}
