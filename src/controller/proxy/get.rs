use axum::{extract::State, routing::get, Router};
use futures_util::TryStreamExt;

use crate::{web::Web, Services, WebResult};

#[utoipa::path(
    get,
    tag = "Proxy",
    path = "/proxy",
    responses(
        (
            status = 200,
            description = "List of all proxies in the sync module",
            body = [Proxy],
            example = json!(
                {
                    "code": "200 OK",
                    "message": "Get all proxies successfully",
                    "data": [
                        { "url": "http://proxy1:1000" },
                        { "url": "http://proxy2:2000" }
                    ],
                    "error": ""
                }
            )
        )
    )
)]
pub fn get_proxies() -> Router<Services> {
    async fn get_proxies_handler(
        State(Services { proxy_service, .. }): State<Services>,
    ) -> WebResult {
        let proxies = proxy_service
            .get_proxies()
            .await?
            .try_collect::<Vec<_>>()
            .await?;

        Ok(Web::ok("Get all proxies successfully", proxies))
    }
    Router::new().route("/", get(get_proxies_handler))
}

#[cfg(test)]
mod tests {
    use axum_test_helper::TestClient;
    use mongodb::Collection;
    use reqwest::{Client, StatusCode};
    use serde_json::json;

    use crate::{
        controller::routes, models::proxy::Proxy, mongo::connect_mongo,
        service::proxy::ProxyService, web::Web, Services,
    };

    #[tokio::test]
    async fn get_proxies_should_success_test() {
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

        let response = test_client.get("/proxy").send().await;

        assert_eq!(response.status(), StatusCode::OK);

        let Web {
            code,
            message,
            data,
            ..
        } = response.json().await;
        assert_eq!(code, StatusCode::OK.to_string());
        assert_eq!(message, "Get all proxies successfully");

        test_client
            .delete("/proxy/delete")
            .json(&json!(
                { "url": "http://localhost:3000" }
            ))
            .send()
            .await;
    }
}
