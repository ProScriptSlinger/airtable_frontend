use wasm_bindgen::JsError;
use serde::{Serialize, Deserialize};
use gloo::net::http::Request;
use log::error;
use serde_json::{json, Value};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SimpleTableItem {
    pub id: Option<String>,
    pub title: String,
    pub body: String,
}



pub async fn fetch_table() -> Result<Vec<SimpleTableItem>, JsError> {
    let response = Request::get("http://127.0.0.1:9090/tables")
        .send()
        .await;

    match response {
        Ok(res) => {
            match res.json().await {
                Ok(data) => Ok(data),
                Err(json_err) => {
                    error!("Error parsing JSON response: {:?}", json_err);
                    Err(json_err.into())  // Remove semicolon here
                },
            }
        }
        Err(fetch_err) => {
            error!("Error fetching data: {:?}", fetch_err);
            Err(fetch_err.into())  // Remove semicolon here
        },
    }
}

pub async fn post_table(item: SimpleTableItem) -> Result<(), JsError> {
    let body = json!(item).to_string();

    let response = Request::post("http://127.0.0.1:9090/tables")
        .header("Content-Type", "application/json")
        .body(body)
        .map_err(|e| JsError::new(&format!("Failed to create request: {}", e)))?
        .send()
        .await
        .map_err(|e| JsError::new(&format!("Failed to send request: {}", e)))?;

    if response.ok() {
        match response.json::<serde_json::Value>().await {
            Ok(_) => Ok(()),
            Err(json_err) => {
                web_sys::console::error_1(&format!("Error parsing JSON response: {:?}", json_err).into());
                Err(JsError::from(json_err))
            },
        }
    } else {
        web_sys::console::error_1(&format!("HTTP error: {}", response.status()).into());
        Err(JsError::new(&format!("HTTP error with status: {}", response.status())))
    }
}

pub async fn put_table(item: SimpleTableItem) -> Result<(), JsError> {
    // Extract the ID to construct the proper URL
    let table_id_with_prefix = item.id.clone().unwrap_or_else(|| "unknown_id".to_string());

    let id_parts: Vec<&str> = table_id_with_prefix.split(':').collect();
    let table_id = id_parts.get(1).unwrap_or(&"").to_string();
    web_sys::console::error_1(&format!("table id ------> {:?}", table_id).into());
    let url = format!("http://127.0.0.1:9090/tables/{}", table_id);
    
    let body = json!(item).to_string();

    let response = Request::put(&url)
        .header("Content-Type", "application/json")
        .body(body)
        .map_err(|e| JsError::new(&format!("Failed to create request: {}", e)))?
        .send()
        .await
        .map_err(|e| JsError::new(&format!("Failed to send request: {}", e)))?;

    if response.ok() {
        match response.json::<serde_json::Value>().await {
            Ok(_) => Ok(()),
            Err(json_err) => {
                web_sys::console::error_1(&format!("Error parsing JSON response: {:?}", json_err).into());
                Err(JsError::from(json_err))
            },
        }
    } else {
        web_sys::console::error_1(&format!("HTTP error: {}", response.status()).into());
        Err(JsError::new(&format!("HTTP error with status: {}", response.status())))
    }
}

pub async fn delete_table(table_id: String) -> Result<(), JsError> {
    let url = format!("http://127.0.0.1:9090/tables/{}", table_id);

    let response = Request::delete(&url)
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| JsError::new(&format!("Failed to send request: {}", e)))?;

    if response.ok() {
        match response.json::<serde_json::Value>().await {
            Ok(_) => Ok(()),
            Err(json_err) => {
                web_sys::console::error_1(&format!("Error parsing JSON response: {:?}", json_err).into());
                Err(JsError::from(json_err))
            },
        }
    } else {
        web_sys::console::error_1(&format!("HTTP error: {}", response.status()).into());
        Err(JsError::new(&format!("HTTP error with status: {}", response.status())))
    }
}

