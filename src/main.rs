mod customs;

use log::{info, warn, error};

fn main() -> Result<(), log::SetLoggerError> {
    customs::init("Logs")?;

    info!("App started");
    warn!("This is a warning");
    error!("An error occurred");
    http!(error, "Server Started on Port: {}", 8080);

    Ok(())
}