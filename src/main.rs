use ::public_ip_address;
use dat::*;
use std::thread;
use tokio::sync::oneshot;

#[macro_use]
mod customs;
mod dat;
mod http_server;

#[tokio::main]
async fn main() -> Result<(), log::SetLoggerError> {
    customs::init("Logs")?;

    let (_tx, rx) = oneshot::channel::<()>();

    match read_encrypted_dat("settings.dat", &AES_IV) {
        Ok((header, events, settings)) => {
            datl!(info, "Header: {:?}", header);
            datl!(info, "Events: {:?}", events);
            datl!(info, "Settings: {:?}", settings);

            if let Some(global_settings) = get_settings() {
                let result = public_ip_address::perform_lookup(None).await;
                let host = match result {
                    Ok(lookup_response) => lookup_response.ip.to_string(),
                    Err(_) => global_settings.host,
                };

                datl!(info, "Host: {}", host);
                let ws_port = global_settings.websocket_port;
                let http_port = global_settings.http_port;
                datl!(info, "Websocket Port: {}", ws_port);
                datl!(info, "HTTP Port: {}", http_port);
            }
        }
        Err(e) => datl!(error, "{}", e),
    }

    let http_thread = thread::spawn(move || {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            http_server::start_http_server().await;
        });
    });

    rx.await.ok();

    http_thread.join().unwrap();

    Ok(())
}
