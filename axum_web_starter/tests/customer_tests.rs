use reqwest::Client;
use serde_json::json;

async fn setup_create_customer() -> String {
    let client = Client::new();
    let response = client.post("http://localhost:3000/customers")
        .json(&json!({
            "name": "John Doe",
            "email": "john.doe@example.com"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body["id"].is_string());
    body["id"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn test_create_customer() {
    let customer_id = setup_create_customer().await;
    println!("Created Customer ID: {}", customer_id);
}

#[tokio::test]
async fn test_get_customer() {
    let customer_id = setup_create_customer().await;

    let client = Client::new();
    let response = client.get(&format!("http://localhost:3000/customers/{}", customer_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["id"], customer_id);
}
