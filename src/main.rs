use hyper::{Body, Client, Request};
use std::{io::Error, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_transfer = tokio::spawn(data_transfer());

    let data_collection = tokio::spawn(data_collection());


    tokio::join!(data_transfer, data_collection);

    Ok(())
}


async fn data_collection() {
    loop {
        println!("Task2");

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}

async fn data_transfer(){
        let http_client = Client::new();
        loop {
            let request = Request::post("http://technik.prignitz-compi.de")
                .header("Content-Type", "application/json")
                .body(Body::from("{}"))
                .unwrap();

            match http_client.request(request).await {
                Ok(response) => println!("Success!"),
                Err(e) => println!("Error: {}", e),
            }

            tokio::time::sleep(Duration::from_secs(6)).await;
        }
}