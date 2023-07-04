use axum::{extract::State, routing::delete, Router};

use crate::{request::proxy::delete::DeleteProxyRequest, web::Web, Services, WebResult};

#[utoipa::path(
    delete,
    tag = "Proxy",
    path = "/proxy/delete",
    request_body(
        content = DeleteProxyRequest,
        description = "Delete proxy request",
        example = json!(
            { "url": "http://proxy3:3000" }
        ),
    ),
    responses(
        (
            status = 200,
            description = "Delete proxy success",
            body = SuccessResponse,
            example = json!(
                {
                    "code": "200 OK",
                    "message": "Deleted proxy successfully",
                    "data": null,
                    "error": "",
                }
            )
        ),
        (
            status = 404,
            description = "Proxy not found",
            body = ErrorResponse,
            example = json!(
                {
                    "code": "404 Not Found",
                    "message": "Not found",
                    "data": null,
                    "error": "The url provided cannot be found in the database",
                }
            )
        ),
    )
)]
pub fn delete_proxy() -> Router<Services> {
    async fn delete_proxy_handler(
        State(Services { proxy_service, .. }): State<Services>,
        DeleteProxyRequest { url }: DeleteProxyRequest,
    ) -> WebResult {
        proxy_service.delete_proxy(&url).await?;
        Ok(Web::ok("Deleted proxy successfully", ()))
    }
    Router::new().route("/delete", delete(delete_proxy_handler))
}

#[cfg(test)]
mod tests {
    use axum_test_helper::TestClient;
    use mongodb::Collection;
    use reqwest::{Client, StatusCode};
    use serde_json::json;

    use crate::{
        controller::routes, models::proxy::Proxy, mongo::connect_mongo,
        service::proxy::ProxyService, Services,
    };

    #[tokio::test]
    async fn delete_proxy_should_success_test() {
        let proxy_collection: Collection<Proxy> = connect_mongo().await.collection("Proxy");

        let proxy_service = ProxyService::init(&proxy_collection);

        let service = Services {
            client: Client::new(),
            proxy_service,
        };

        let router = routes(service);

        let test_client = TestClient::new(router);

        test_client
            .post("/proxy/create")
            .json(&json!(
                { "url": "http://localhost:3000" }
            ))
            .send()
            .await;

        let response = test_client
            .delete("/proxy/delete")
            .json(&json!(
                { "url": "http://localhost:3000" }
            ))
            .send()
            .await;

        assert_eq!(response.status(), StatusCode::OK)
    }

    #[tokio::test]
    async fn delete_proxy_should_fail_test() {
        let proxy_collection: Collection<Proxy> = connect_mongo().await.collection("Proxy");

        let proxy_service = ProxyService::init(&proxy_collection);

        let service = Services {
            client: Client::new(),
            proxy_service,
        };

        let router = routes(service);

        let test_client = TestClient::new(router);

        let response = test_client
            .delete("/proxy/delete")
            .json(&json!(
                { "url": "http://invalid" }
            ))
            .send()
            .await;

        assert_eq!(response.status(), StatusCode::OK)
    }
}
