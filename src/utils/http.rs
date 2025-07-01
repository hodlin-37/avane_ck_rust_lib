use log::info;
use anyhow::{Result, Context};
use serde::Serialize;
use reqwest::{Client, Response};
use std::collections::HashMap;

pub async fn http_request_post<T: Serialize + std::fmt::Debug>(
    url: &str,
    body: &T,
    headers: Option<HashMap<String, String>>,
) -> Result<Response> {
    let client = Client::new();
    let mut request = client.post(url).json(body);

    if let Some(headers_map) = headers {
        for (key, value) in headers_map {
            request = request.header(&key, &value);
        }
    }

    let response = request
        .send()
        .await
        .with_context(|| format!("❌ POST isteği başarısız: {}", url))?;

    info!("✅ POST → {:?} | Body: {:?}", url, body);
    Ok(response)
}

pub async fn http_request_get(
    url: &str,
    headers: Option<HashMap<String, String>>,
) -> Result<Response> {
    let client = Client::new();
    let mut request = client.get(url);

    if let Some(headers_map) = headers {
        for (key, value) in headers_map {
            request = request.header(&key, &value);
        }
    }

    let response = request
        .send()
        .await
        .with_context(|| format!("❌ GET isteği başarısız: {}", url))?;

    info!("✅ GET → {}", url);
    Ok(response)
}

pub async fn http_request_put<T: Serialize + std::fmt::Debug>(
    url: &str,
    body: &T,
    headers: Option<HashMap<String, String>>,
) -> Result<Response> {
    let client = Client::new();
    let mut request = client.put(url).json(body);

    if let Some(headers_map) = headers {
        for (key, value) in headers_map {
            request = request.header(&key, &value);
        }
    }

    let response = request
        .send()
        .await
        .with_context(|| format!("❌ PUT isteği başarısız: {}", url))?;

    info!("✅ PUT → {:?} | Body: {:?}", url, body);
    Ok(response)
}

pub async fn http_request_patch<T: Serialize + std::fmt::Debug>(
    url: &str,
    body: &T,
    headers: Option<HashMap<String, String>>,
) -> Result<Response> {
    let client = Client::new();
    let mut request = client.patch(url).json(body);

    if let Some(headers_map) = headers {
        for (key, value) in headers_map {
            request = request.header(&key, &value);
        }
    }

    let response = request
        .send()
        .await
        .with_context(|| format!("❌ PATCH isteği başarısız: {}", url))?;

    info!("✅ PATCH → {:?} | Body: {:?}", url, body);
    Ok(response)
}
