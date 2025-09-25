#[macro_use]
mod customs;
mod dat;

use dat::*;

fn main() -> Result<(), log::SetLoggerError> {
    customs::init("Logs")?;

    match read_encrypted_dat("settings.dat", &AES_IV) {
        Ok((header, events, settings)) => {
            datl!(info, "Header: {:?}", header);
            datl!(info, "Events: {:?}", events);
            datl!(info, "Settings: {:?}", settings);
            
            if let Some(global_settings) = get_settings() {
                let host = global_settings.host;
                let ws_port = global_settings.websocket_port;
                let http_port = global_settings.http_port;
                datl!(info, "Host: {}", host);
                datl!(info, "Websocket Port: {}", ws_port);
                datl!(info, "HTTP Port: {}", http_port);
            }
        }
        Err(e) => datl!(error, "{}", e),
    }

    Ok(())
}