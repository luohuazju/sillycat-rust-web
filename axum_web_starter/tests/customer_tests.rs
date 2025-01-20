use reqwest::Client;
use serde_json::json;

async fn setup_customer() -> String {
    let client = Client::new();
    let response = client.post("http://localhost:3000/customers")
        .json(&json!({
            "name": "Setup User",
            "email": "setup.user@example.com"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    body["id"].as_str().unwrap().to_string() // Return the created UUID
}

async fn teardown_customer(customer_id: &str) {
    let client = Client::new();
    let response = client.delete(&format!("http://localhost:3000/customers/{}", customer_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn test_crud_operations() {
    let customer_id = setup_customer().await;

    // Test Update
    let client = Client::new();
    let response = client.put(&format!("http://localhost:3000/customers/{}", customer_id))
        .json(&json!({
            "name": "Updated User",
            "email": "updated.user@example.com"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["name"], "Updated User");
    assert_eq!(body["email"], "updated.user@example.com");

    // Test Get
    let response = client.get(&format!("http://localhost:3000/customers/{}", customer_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert_eq!(body["id"], customer_id);

    // Teardown
    teardown_customer(&customer_id).await;

    // Verify Teardown
    let response = client.get(&format!("http://localhost:3000/customers/{}", customer_id))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 404);
}

#[tokio::test]
async fn test_list_customers() {
    let customer_id = setup_customer().await;

    let client = Client::new();
    let response = client.get("http://localhost:3000/customers")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body: serde_json::Value = response.json().await.unwrap();
    assert!(body.as_array().unwrap().iter().any(|c| c["id"] == customer_id));

    teardown_customer(&customer_id).await;
}
