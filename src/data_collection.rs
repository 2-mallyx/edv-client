use std::time::Duration;

pub async fn data_collection() {
    loop {
        println!("Task2");

        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
