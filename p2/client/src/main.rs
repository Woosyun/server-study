use std::{
    thread,
    time::Duration,
};
#[tokio::main]
async fn main() {
    set(0, "hello".to_string()).await;
    set(0, "world".to_string()).await;
    let world = get(0).await;
    println!("value from 0: {:?}", world);
    expires(0, 1).await;

    thread::sleep(Duration::from_secs(3));

    let none = get(0).await;
    println!("value from 0: {:?}", none)
}

async fn get(key: u64) -> Option<String> {
    let url = format!("http://localhost:3000/get/{}", key);
    reqwest::get(url)
        .await.unwrap()
        .json::<Option<String>>()
        .await.unwrap()
}
async fn set(key: u64, value: String) {
    let url = format!("http://localhost:3000/set/{}", key);
    let client =  reqwest::Client::new();
    client.post(url)
        .body(value)
        .send()
        .await.unwrap();
}
async fn expires(key: u64, duration: u64) {
    let url = format!("http://localhost:3000/expires/{}", key);
    let client =  reqwest::Client::new();
    client.post(url)
        .json(&duration)
        .send()
        .await.unwrap();
}
