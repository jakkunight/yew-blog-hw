use serde_json::json;

#[allow(dead_code)]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TestMessage {
    name: String,
    email: String,
    pass: String,
}

#[allow(dead_code)]
pub async fn echo_test(name: &str, email: &str, pass: &str) -> Result<String, String> {
    let test_value = json!({
        "name": name,
        "email": email,
        "pass": pass
    });
    let response = reqwasm::http::Request::post("http://localhost:3000/echo")
        .header("Access-Control-Allow-Origin", "*")
        .header("Content-Type", "application/json")
        .body(test_value.to_string())
        .send()
        .await
        .unwrap();
    let data = response.json().await.unwrap();
    Ok(data)
}
