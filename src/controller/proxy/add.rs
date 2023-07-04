use axum::{extract::State, routing::post, Router};

use crate::{error::Error, request::proxy::add::AddProxyRequest, web::Web, Services, WebResult};

#[utoipa::path(
    post,
    tag = "Proxy",
    path = "/proxy/create",
    request_body(
        content = AddProxyRequest,
        description = "Add proxy request",
        example = json!(
            { "url": "http://proxy3:3000" }
        )
    ),
    responses(
        (
            status = 200,
            description = "Created new proxy",
            body = Proxy,
            example = json!(
                {
                    "code": "200 OK",
                    "message": "New proxy created",
                    "data": {
                        "url": "http://proxy3:3000"
                    },
                    "error": ""
                }
            )
        ),
        (
            status = 400,
            description = "Duplicate proxies",
            body = ErrorResponse,
            example = json!(
                {
                    "code": "400 Bad Request",
                    "message": "Proxy already exists",
                    "data": null,
                    "error": "This proxy is already exists, please try another"
                }
            )
        )
    )
)]
pub fn add_proxy() -> Router<Services> {
    async fn add_proxy_handler(
        State(Services {
            client,
            proxy_service,
        }): State<Services>,
        AddProxyRequest { url }: AddProxyRequest,
    ) -> WebResult {
        // Test connection before adding to the proxies list
        match client.get(format!("{url}/health")).send().await {
            // Connection successful
            Ok(_) => {
                let new_proxy = proxy_service.add_proxy(&url).await?;
                Ok(Web::created("New proxy created", new_proxy))
            }
            // Connection failed
            Err(_) => Err(Error::CannotReachProxy),
        }
    }
    Router::new().route("/create", post(add_proxy_handler))
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
    async fn add_proxy_should_success_test() {
        let proxy_collection: Collection<Proxy> = connect_mongo().await.collection("Proxy");

        let proxy_service = ProxyService::init(&proxy_collection);

        let service = Services {
            client: Client::new(),
            proxy_service,
        };

        let router = routes(service);

        let test_client = TestClient::new(router);

        let response = test_client
            .post("/proxy/create")
            .json(&json!(
                { "url": "http://localhost:3000" }
            ))
            .send()
            .await;
        assert_eq!(response.status(), StatusCode::CREATED);

        let Web {
            code,
            message,
            error,
            ..
        } = response.json().await;
        assert_eq!(code, StatusCode::CREATED.to_string());
        assert_eq!(message, "New proxy created");
        assert_eq!(error, "");

        test_client
            .delete("/proxy/delete")
            .json(&json!(
                { "url": "http://localhost:3000" }
            ))
            .send()
            .await;
    }

    #[tokio::test]
    async fn add_proxy_should_fail_test() {
        let proxy_collection: Collection<Proxy> = connect_mongo().await.collection("Proxy");

        let proxy_service = ProxyService::init(&proxy_collection);

        let service = Services {
            client: Client::new(),
            proxy_service,
        };

        let router = routes(service);

        let test_client = TestClient::new(router);

        let response = test_client
            .post("/proxy/create")
            .json(&json!(
                { "url": "http://invalid" }
            ))
            .send()
            .await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
