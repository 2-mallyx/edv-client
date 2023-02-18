use std::time::Duration;
use hyper::{Body, Client, Request};


pub async fn data_transfer(){
    let http_client = Client::new();
    loop {
        let request = Request::post("http://technik.prignitz-compi.de")
            .header("Content-Type", "application/json")
            .body(Body::from("{}"))
            .unwrap();

        match http_client.request(request).await {
            Ok(_response) => println!("Success!"),
            Err(e) => println!("Error: {}", e),
        }

        tokio::time::sleep(Duration::from_secs(6)).await;
    }
}