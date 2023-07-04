#![allow(dead_code, unused_variables)]

use std::net::SocketAddr;

use axum::response::Response;
use controller::routes;
use dotenvy::var;
use error::Error;
use models::proxy::Proxy;
use mongodb::Collection;
use reqwest::Client;
use service::proxy::ProxyService;

use crate::mongo::connect_mongo;
pub mod controller;

mod error;
mod helper;
mod models;
mod mongo;
mod request;
mod service;
mod web;

type WebResult = std::result::Result<Response, Error>;

#[derive(Clone)]
pub struct Services {
    pub client: Client,
    pub proxy_service: ProxyService,
}

#[tokio::main]
async fn main() {
    let proxy_collection: Collection<Proxy> = connect_mongo().await.collection("Proxy");

    let proxy_service = ProxyService::init(&proxy_collection);

    let service = Services {
        client: Client::new(),
        proxy_service,
    };

    let router = routes(service);

    let port = var("PORT")
        .expect("PORT is required in .env")
        .parse()
        .expect("Cannot parse PORT to number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .expect("Server crashed")
}
