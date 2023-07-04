pub mod data;
pub mod proxy;

use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::Services;

use self::{data::data_routes, proxy::proxy_routes};
use crate::{
    models::{error::*, proxy::Proxy, success::*},
    request::{proxy::{add::*, delete::*}, data::{set::*, delete::*}},
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Sync Module API", 
        version = "0.0.1"
    ),
    components(schemas(
        // Proxy models
        Proxy,
        AddProxyRequest,
        DeleteProxyRequest,

        // Data sync models
        SetDataRequest,
        SetMultiDataRequest,
        DeleteDataRequest,
        
        // General Reponses
        SuccessResponse,
        ErrorResponse
    )),
    paths(
        // Sync paths
        data::health::health,
        data::set::set_data,
        data::set::set_multi_data,
        data::delete::delete_data,

        // Proxy paths
        proxy::get::get_proxies,
        proxy::add::add_proxy,
        proxy::delete::delete_proxy
    ),
    tags(
        (name = "Proxy", description = "API routes for managing proxies"),
        (name = "Sync", description = "API routes for syncing data between proxies")
    )
)]
struct ApiDoc;

pub fn routes(service: Services) -> Router {
    Router::new()
        .merge(data_routes())
        .merge(proxy_routes())
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .with_state(service)
}
