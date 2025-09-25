use std::net::{IpAddr, SocketAddr};

mod endpoints;
use crate::dat::get_settings;
use crate::http_server::endpoints::register_routes;

pub async fn start_http_server() {
    let routes = register_routes();

    if let Some(settings) = get_settings() {
        let result = public_ip_address::perform_lookup(None).await;
        
        let host = match result {
            Ok(lookup_response) => lookup_response.ip.to_string(),
            Err(_) => settings.host.clone(),
        };

        let port = settings.http_port;

        let ip_addr: IpAddr = host.parse().unwrap_or_else(|_| {
            IpAddr::from([127, 0, 0, 1])
        });

        let socket_addr = SocketAddr::new(ip_addr, port);

        http!(info, "server started on https://{}", socket_addr);

        warp::serve(routes).run(socket_addr).await;
    } else {
        http!(error, "No settings found for HTTP server!");
    }
}
