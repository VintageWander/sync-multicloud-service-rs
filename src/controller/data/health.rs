use axum::{extract::State, routing::get, Router};
use futures_util::{future::try_join_all, TryStreamExt};

use crate::{models::proxy::Proxy, web::Web, Services, WebResult};

#[utoipa::path(
    get,
    tag = "Sync",
    path = "/sync/health",
    responses(
        (
            status = 200,
            description = "All proxies functional",
            body = SuccessResponse,
            example = json!(
                {
                    "code": "200 OK",
                    "message": "All proxies functional",
                    "data": null,
                    "error": "",
                }
            )
        ),
        (
            status = 500,
            description = "Request to proxies error",
            body = SuccessResponse,
            example = json!(
                {
                    "code": "500 Internal Server Error",
                    "message": "Request to proxies error",
                    "data": null,
                    "error": "A connection to one of the proxies could not be made",
                }
            )
        )
    )
)]
pub fn health() -> Router<Services> {
    async fn health_handler(
        State(Services {
            client,
            proxy_service,
        }): State<Services>,
    ) -> WebResult {
        let mut proxies = proxy_service.get_proxies().await?;

        let mut tasks = vec![];

        while let Some(Proxy { url }) = proxies.try_next().await? {
            tasks.push(client.get(format!("{url}/health")).send())
        }

        // The difference between join_all and try_join_all is that
        // join_all launches all tasks in parallel and doesn't care about
        // the task's output
        // try_join_all does the same, but if one of the tasks returns an error,
        // the error will be returned, regardless of the rest
        try_join_all(tasks).await?;
        Ok(Web::ok("All proxies functional", ()))
    }
    Router::new().route("/health", get(health_handler))
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
    async fn get_health_should_success() {
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

        let response = test_client.get("/sync/health").send().await;

        assert_eq!(response.status(), StatusCode::OK);

        let Web {
            code,
            message,
            data,
            error,
        } = response.json().await;

        assert_eq!(code, StatusCode::OK.to_string());
        assert_eq!(message, "All proxies functional");
        assert_eq!(data, json!(null));
        assert_eq!(error, "");

        test_client
            .delete("/proxy/delete")
            .json(&json!(
                { "url": "http://localhost:3000" }
            ))
            .send()
            .await;
    }
}
