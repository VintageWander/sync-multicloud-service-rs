use axum::Router;

use crate::Services;

use self::{
    delete::delete_data,
    health::health,
    set::{set_data, set_multi_data},
};

pub mod delete;
pub mod health;
pub mod set;

pub fn data_routes() -> Router<Services> {
    Router::new().nest(
        "/sync",
        Router::new()
            .merge(health())
            .merge(set_data())
            .merge(set_multi_data())
            .merge(delete_data()),
    )
}
