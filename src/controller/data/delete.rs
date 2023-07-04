use axum::{extract::State, routing::delete, Router};
use futures_util::{future::join_all, TryStreamExt};

use crate::{
    models::proxy::Proxy, request::data::delete::DeleteDataRequest, web::Web, Services, WebResult,
};

#[utoipa::path(
    delete,
    tag = "Sync",
    path = "/sync",
    request_body(
        content = DeleteDataRequest,
        description = "Delete data request model",
        example = json!(
            { 
                "type": "String",
                "key": "test_str",
                "ttl": "30", // seconds
            }
        )
    ),
    responses(
        (
            status = 200,
            description = "Delete data from all proxies success",
            body = SuccessResponse,
            example = json!(
                {
                    "code": "200",
                    "message": "Delete data from all proxies success",
                    "data": null,
                    "error": "",
                }
            )
        )
    )
)]
pub fn delete_data() -> Router<Services> {
    async fn delete_data_handler(
        State(Services {
            client,
            proxy_service,
        }): State<Services>,
        req: DeleteDataRequest,
    ) -> WebResult {
        // Get the database cursor, pointing to the list of paths in MongoDB
        let mut proxies = proxy_service.get_proxies().await?;

        // Create a tasks vector
        let mut tasks = vec![];

        // For each item in the collection
        while let Some(Proxy { url }) = proxies.try_next().await? {
            // Append a task into the tasks list
            tasks.push(
                client
                    .delete(format!("{url}/proxy-sync/v1"))
                    .json(&req)
                    .send(),
            )
        }

        // Run all tasks in parallel
        join_all(tasks).await;
        Ok(Web::ok("Delete data from all proxies successfully", ()))
    }
    Router::new().route("/", delete(delete_data_handler))
}

#[cfg(test)]
mod tests {
    use axum_test_helper::TestClient;
    use mongodb::Collection;
    use reqwest::{Client, StatusCode};
    use serde_json::json;

    use crate::{models::proxy::Proxy, mongo::connect_mongo, service::proxy::ProxyService, Services, controller::routes};

    #[tokio::test]
    async fn delete_data_should_success() {
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

        test_client.post("/sync").json(
            &json!(
                { 
                    "type": "String",
                    "key": "test_str",
                    "value": {
                        "hello": "world"
                    },
                }
            )
        ).send().await;
        
        let response = test_client.delete("/sync").json(
            &json!(
                { 
                    "type": "String",
                    "key": "test_str",
                }
            )
        ).send().await;

        assert_eq!(response.status(), StatusCode::OK);

        test_client
            .delete("/proxy/delete")
            .json(&json!(
                { "url": "http://localhost:3000" }
            ))
            .send()
            .await;
    }
}