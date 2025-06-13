const url: &'static str = "http://localhost:3000";

#[tokio::main]
async fn main() {
    //set_value(0, "hello").await;
    //set_value(1, "world").await;

    read_value(0).await;
    read_value(1).await;
    read_value(2).await;

    //set_expire(0, 10).await;
}

async fn read_value(key: u32) {
    let target = format!("{}/get/{}", url, key);
    let res = reqwest::get(target)
        .await.unwrap()
        .json::<Option<String>>()
        .await.unwrap();
    match res {
        Some(value) => println!("key: {}, value: {}", key, value),
        None => println!("key: {}, value not found", key),
    };
}

async fn set_value(key: u32, value: &str) {
    let target = format!("{}/set/{}/{}", url, key, value);
    reqwest::get(target)
        .await.unwrap();
}
async fn set_expire(key: u32, duration: u32) {
    let target = format!("{}/set-expire/{}/{}", url, key, duration);
    reqwest::get(target)
        .await.unwrap();
}
