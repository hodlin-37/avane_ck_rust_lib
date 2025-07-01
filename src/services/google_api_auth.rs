use crate::utils::http::http_request_post;
use std::collections::HashMap;
use std::fs;
use crate::schemas::struct_google_api::{ServiceAccountCredentials, JWTClaims, GoogleTokenResponse};
use jsonwebtoken::{encode, Header, EncodingKey, Algorithm};
use anyhow::{Result, Context};
use log::info;

const GOOGLE_API_SCOPES: [&str; 4] = [
    "https://www.googleapis.com/auth/cloud-platform",
    "https://www.googleapis.com/auth/drive",
    "https://www.googleapis.com/auth/spreadsheets",
    "https://www.googleapis.com/auth/gmail.readonly",
];

fn auth_service_account(file_path: &str) -> Result<ServiceAccountCredentials> {

    let file_content = fs::read_to_string(file_path)
        .with_context(|| format!("❌ API kimlik dosyası okunamadı: '{}'", file_path))?;

    let credentials: ServiceAccountCredentials = serde_json::from_str(&file_content)
        .with_context(|| "❌ API kimlik dosyası bozuk ya da geçersiz JSON formatında")?;

    Ok(credentials)
}

fn create_jwt_token(credentials: &ServiceAccountCredentials) -> Result<String> {
    let now = chrono::Utc::now().timestamp() as usize;

    let claims = JWTClaims {
        iss: &credentials.client_email,
        scope: &GOOGLE_API_SCOPES.join(" "),
        aud: &credentials.token_uri,
        exp: now + 3600,
        iat: now,
    };

    let encoding_key = EncodingKey::from_rsa_pem(credentials.private_key.as_bytes())
        .with_context(|| "❌ RSA imzalama anahtarı oluşturulamadı (private_key geçersiz olabilir)")?;

    let header = Header::new(Algorithm::RS256);

    let jwt = encode(&header, &claims, &encoding_key)
        .with_context(|| "❌ JWT token oluşturulurken hata oluştu")?;

    Ok(jwt)
}

pub async fn get_access_token(file_path: &str) -> Result<String> {
    let credentials = auth_service_account(file_path)
        .with_context(|| "❌ Google API kimlik bilgileri alınamadı")?;

    let jwt_token = create_jwt_token(&credentials)
        .with_context(|| "❌ JWT token oluşturulamadı")?;

    let url = credentials.token_uri.clone();

    let mut headers = HashMap::new();
    headers.insert(
        "Content-Type".to_string(),
        "application/x-www-form-urlencoded".to_string(),
    );

    let mut body = HashMap::new();
    body.insert("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer");
    body.insert("assertion", &jwt_token);

    let response = http_request_post(&url, &body, Some(headers))
        .await
        .with_context(|| "❌ Access token alınırken POST isteği başarısız oldu")?;

    let token_response: GoogleTokenResponse = response
        .json()
        .await
        .with_context(|| "❌ Access token yanıtı parse edilemedi (JSON uyumsuzluğu)")?;

    info!("✅ Google access token başarıyla alındı.");
    Ok(token_response.access_token)
}
