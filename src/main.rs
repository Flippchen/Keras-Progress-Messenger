use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Error, StatusCode};
use serde_json::Value;
use std::net::SocketAddr;

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Error> {
    // Read the body and parse it as JSON
    let body_bytes = hyper::body::to_bytes(req.into_body()).await?;
    let json_result: Result<Value, _> = serde_json::from_slice(&body_bytes);

    if let Ok(json) = json_result {
        // Print the JSON message
        println!("Received JSON: {}", json);

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
    // Set the address to bind the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 9000));

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


