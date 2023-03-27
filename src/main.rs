mod webhook;

use std::env;
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Error, StatusCode};
use serde_json::Value;
use std::net::SocketAddr;

use crate::webhook::send_discord_notification;


async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Error> {
    // Read the body and parse it as JSON
    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
    let json_result: Result<Value, _> = serde_json::from_slice(&body_bytes);
    let webhook_url = env::var("WEBHOOK_URL").expect("WEBHOOK NOT FOUND");

    if let Ok(json) = json_result {
        // Print the JSON message
        println!("Received JSON: {}", json);

        send_discord_notification(&webhook_url, &json).await;
        // Send a response
        Ok(Response::new(Body::from("Message received and printed")))
    } else {
        println!("Invalid JSON received");
        let mut response = Response::new(Body::from("Invalid JSON format"));
        *response.status_mut() = StatusCode::BAD_REQUEST;
        Ok(response)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let port = env::var("PORT")
        .unwrap_or_else(|_| "9000".to_string())
        .parse::<u16>()
        .unwrap_or(9000);
    // Set the address to bind the server
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // Create a service function
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Error>(service_fn(handle_request)) }
    });

    // Create the server
    let server = Server::bind(&addr).serve(make_svc);

    // Run the server
    println!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}


