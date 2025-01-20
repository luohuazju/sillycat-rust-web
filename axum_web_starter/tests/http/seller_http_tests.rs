use reqwest::Client;
use serde_json::json;

async fn setup_seller() -> String {
    let client = Client::new();
    let response = client.post("http://localhost:3000/sellers")
        .json(&json!({
            "name": "Setup User",
            "company_name": "Setup Company"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    body["id"].as_str().unwrap().to_string() // Return the created UUID
}

async fn teardown_seller(seller_id: &str) {
    let client = Client::new();
    let response = client.delete(&format!("http://localhost:3000/sellers/{}", seller_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_crud_operations() {
    let seller_id = setup_seller().await;

    // Test Update
    let client = Client::new();
    let response = client.put(&format!("http://localhost:3000/sellers/{}", seller_id))
        .json(&json!({
            "name": "Updated User",
            "company_name": "Updated Company"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["name"], "Updated User");
    assert_eq!(body["company_name"], "Updated Company");

    // Test Get
    let response = client.get(&format!("http://localhost:3000/sellers/{}", seller_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["id"], seller_id);

    // Teardown
    teardown_seller(&seller_id).await;

    // Verify Teardown
    let response = client.get(&format!("http://localhost:3000/sellers/{}", seller_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn test_list_sellers() {
    let seller_id = setup_seller().await;

    let client = Client::new();
    let response = client.get("http://localhost:3000/sellers")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.as_array().unwrap().iter().any(|c| c["id"] == seller_id));

    teardown_seller(&seller_id).await;
}
