use reqwest;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ResponseBody {
    user_id: u8,
    id: u8,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await
        .expect("Issue in Request");

    println!("{:#?}", response);

    let body = response.text().await.expect("Body Issue");

    let deserialize = serde_json::from_str::<ResponseBody>(&body).unwrap();

    println!("{:#?}", body);
    println!("{:#?}", deserialize);
}
