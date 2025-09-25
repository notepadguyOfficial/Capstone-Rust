use serde::{Deserialize, Serialize};
use warp::{Filter, Rejection, Reply};

#[derive(Serialize)]
pub struct RegisterResponse {
    message: String,
    status: String,
}

#[derive(Serialize)]
pub struct AddressResponse {
    message: String,
    status: String,
    payload: Option<Payload>,
}

#[allow(dead_code)]
#[derive(Serialize)]
pub struct Payload {
    uid: u32,
    address: String,
    longitude: f64,
    latitude: f64,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct RegisterRequest {
    first_name: String,
    last_name: String,
    phone: u64,
    gender: String,
    username: String,
    password: String,
    birth: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct AddressRequest {
    uid: u32,
    latitude: f64,
    longitude: f64,
}

pub fn customer() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("customer").and(register().or(address()))
}

fn register() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .map(|_body: RegisterRequest| {
            // ToDo: add more logic and handling later

            warp::reply::json(&RegisterResponse {
                message: "Successfully registered".to_string(),
                status: "success".to_string(),
            })
        })
}

fn address() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path("address").and(
        warp::path("save")
            .and(warp::post())
            .and(warp::body::json())
            .map(|_body: AddressRequest| {
                // ToDo: add more logic and handling later

                warp::reply::json(&AddressResponse {
                    message: "Address Successfully Saved.".to_string(),
                    status: "success".to_string(),
                    payload: None,
                })
            })
            .or(warp::path("retrieve").and(warp::get()).map(|| {
                // ToDo: add more logic and handling later

                // e.g
                let id = 1;
                let address = "123 Main St".to_string();
                let longitude = -122.4194;
                let latitude = 37.7749;

                let payload = Payload {
                    uid: id,
                    address: address,
                    longitude: longitude,
                    latitude: latitude,
                };

                warp::reply::json(&AddressResponse {
                    message: "Address Successfully Retrieved.".to_string(),
                    status: "success".to_string(),
                    payload: Some(payload),
                })

                // how to log using my own logger http!(level, "", args)
            })),
    )
}
