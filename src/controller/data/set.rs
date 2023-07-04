use axum::{extract::State, routing::post, Router};
use futures_util::{future::join_all, TryStreamExt};


use crate::{
    models::proxy::Proxy,
    request::data::set::{SetDataRequest, SetMultiDataRequest},
    web::Web,
    Services, WebResult,
};


#[utoipa::path(
    post,
    tag = "Sync",
    path = "/sync",
    request_body(
        content = SetDataRequest,
        description = "Set data request model",
        example = json!(
            { 
                "type": "String",
                "key": "test_str",
                "value": {
                    "hello": "world"
                },
            }
        )
    ),
    responses(
        (
            status = 200,
            description = "Set data to all proxies success",
            body = SuccessResponse,
            example = json!(
                {
                    "code": "200 OK",
                    "message": "Set data to all proxies successfully",
                    "data": null,
                    "error": "",
                }
            )
        )
    )
)]
pub fn set_data() -> Router<Services> {
    async fn set_data_handler(
        State(Services {
            client,
            proxy_service,
        }): State<Services>,
        req: SetDataRequest,
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
                    .post(format!("{url}/proxy-sync/v1"))
                    .json(&req)
                    .send(),
            )
        }

        // Run all tasks in parallel
        join_all(tasks).await;
        Ok(Web::ok("Set data to all proxies successfully", ()))
    }
    Router::new().route("/", post(set_data_handler))
}

#[utoipa::path(
    post,
    tag = "Sync",
    path = "/sync/multi",
    request_body(
        content = SetMultiDataRequest,
        description = "Set multi data request model",
        example = json!(
            { 
                "type": "Multi",
                "data": {
                    "hello1": "world1",
                    "hello2": "world2",
                    "someObject": {
                        "inner_key": "outer_key",
                    }
                }
            }
        )
    ),
    responses(
        (
            status = 200,
            description = "Set multi data to all proxies success",
            body = SuccessResponse,
            example = json!(
                {
                    "code": "200",
                    "message": "Set multi data to all proxies successfully",
                    "data": null,
                    "error": "",
                }
            )
        )
    )
)]
pub fn set_multi_data() -> Router<Services> {  
    async fn set_multi_data_handler(
        State(Services {
            client,
            proxy_service,
        }): State<Services>,
        req: SetMultiDataRequest,
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
                    .post(format!("{url}/proxy-sync/v1/multi"))
                    .json(&req)
                    .send(),
            )
        }

        // Run all tasks in parallel
        join_all(tasks).await;
        Ok(Web::ok("Set multi data to all proxies successfully", ()))
    }
    Router::new().route("/multi", post(set_multi_data_handler))
}

#[cfg(test)]
mod tests {
    use axum_test_helper::TestClient;
    use mongodb::Collection;
    use reqwest::{Client, StatusCode};
    use serde_json::json;

    use crate::{models::proxy::Proxy, mongo::connect_mongo, service::proxy::ProxyService, Services, controller::routes, web::Web};

    #[tokio::test]
    async fn set_data_should_success() {
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

        let response = test_client.post("/sync").json(
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

        assert_eq!(response.status(), StatusCode::OK);

        let Web {code, message, data, error} = response.json().await;
        assert_eq!(code, StatusCode::OK.to_string());
        assert_eq!(message, "Set data to all proxies successfully");
        assert_eq!(data, json!( null ));
        assert_eq!(error, "");


        test_client.delete("/sync").json(
            &json!(
                { 
                    "type": "String",
                    "key": "test_str",
                }
            )
        ).send().await;

        test_client
            .delete("/proxy/delete")
            .json(&json!(
                { "url": "http://localhost:3000" }
            ))
            .send()
            .await;
    }

    #[tokio::test]
    async fn set_multi_data_should_success() {
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

        let response = test_client.post("/sync/multi").json(
            &json!(
                { 
                    "type": "Multi",
                    "data": {
                        "test_str": {
                            "hello": "world"
                        },
                    }
                }
            )
        ).send().await;

        assert_eq!(response.status(), StatusCode::OK);

        let Web {code, message, data, error} = response.json().await;
        assert_eq!(code, StatusCode::OK.to_string());
        assert_eq!(message, "Set multi data to all proxies successfully");
        assert_eq!(error, "");

        test_client.delete("/sync").json(
            &json!(
                { 
                    "type": "String",
                    "key": "test_str",
                }
            )
        ).send().await;

        test_client
            .delete("/proxy/delete")
            .json(&json!(
                { "url": "http://localhost:3000" }
            ))
            .send()
            .await;
    }

}