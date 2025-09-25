pub mod customer;
use crate::http_server::endpoints::customer::customer;

pub fn register_routes() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    customer()
}