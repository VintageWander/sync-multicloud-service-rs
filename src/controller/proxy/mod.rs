pub mod add;
pub mod delete;
pub mod get;

use axum::Router;

use crate::Services;

use self::{add::add_proxy, delete::delete_proxy, get::get_proxies};

pub fn proxy_routes() -> Router<Services> {
    Router::new().nest(
        "/proxy",
        Router::new()
            .merge(get_proxies())
            .merge(add_proxy())
            .merge(delete_proxy()),
    )
}
