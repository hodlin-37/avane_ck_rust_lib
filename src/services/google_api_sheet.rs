use crate::{schemas::struct_google_api::{GoogleSheetResponse, GoogleDriveFileListResponse}, utils::http::{http_request_get, http_request_post}};
use urlencoding::encode;
use reqwest::Response;
use anyhow::{Result, Context};
use log::{info};
use std::collections::HashMap;

pub async fn get_sheet_values(
    spreadsheet_id: &str,
    range: &str,
    access_token: &str,
) -> Result<GoogleSheetResponse> {
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}:append?access_token={}",
        spreadsheet_id, range, access_token
    );

    let response: Response = http_request_get(&url, None)
        .await
        .with_context(|| format!("❌ Sheet verisi alınamadı → spreadsheet_id: {}", spreadsheet_id))?;

    let json_response: GoogleSheetResponse = response
        .json()
        .await
        .with_context(|| "❌ Sheet JSON verisi çözümlenemedi (yanıt uyumsuz)")?;

    info!(
        "✅ Sheet verisi alındı → spreadsheet_id: '{}', range: '{}'",
        spreadsheet_id, range
    );

    Ok(json_response)
}

pub async fn get_spreadsheet_by_name(
    spreadsheet_name: &str,
    access_token: &str,
    folder_id: &str,
) -> Result<String> {
    let drive_query = format!(
        "name='{}' and mimeType='application/vnd.google-apps.spreadsheet' and '{}' in parents and trashed=false",
        spreadsheet_name, folder_id
    );

    let url = format!(
        "https://www.googleapis.com/drive/v3/files?q={}&fields=files(id,name)",
        encode(&drive_query)
    );

    let headers = Some(HashMap::from([
        ("authorization".to_string(), format!("Bearer {}", access_token)),
        ("Accept".to_string(), "application/json".to_string()),
    ]));

    let response: Response = http_request_get(&url, headers)
        .await
        .with_context(|| format!("❌ Drive API çağrısı başarısız → spreadsheet_name: '{}'", spreadsheet_name))?;

    let json_response: GoogleDriveFileListResponse = response
        .json()
        .await
        .with_context(|| "❌ Drive yanıtı JSON olarak çözümlenemedi")?;

    let file_id = json_response
        .files
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("❌ İstenilen dosya bulunamadı → '{}'", spreadsheet_name))?
        .id
        .clone();

    info!("✅ Spreadsheet bulundu → '{}', file_id: '{}'", spreadsheet_name, file_id);

    Ok(file_id)
}

pub async fn append_sheet_values(
    spreadsheet_id: &str,
    range: &str,
    values: Vec<Vec<String>>,
    access_token: &str,
) -> Result<()> {
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}:append?valueInputOption=USER_ENTERED&access_token={}",
        spreadsheet_id, range, access_token
    );

    let body = serde_json::json!({
        "values": values
    });

    let response: Response = http_request_post(&url, &body, None)
        .await
        .with_context(|| format!("❌ Sheet verisi alınamadı → spreadsheet_id: {}", spreadsheet_id))?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("❌ Sheet'e veri ekleme başarısız, HTTP Status: {}", response.status()));
    }

    info!("✅ Sheet'e veri eklendi → spreadsheet_id: '{}', range: '{}'", spreadsheet_id, range);

    Ok(())
}

pub async fn clear_sheet_range (
    spreadsheet_id: &str,
    range: &str,
    access_token: &str,
) -> Result<()> {
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}:clear?access_token={}",
        spreadsheet_id, range, access_token
    );

    let response: Response = http_request_post(&url, &serde_json::json!({}), None)
        .await
        .with_context(|| format!("❌ Sheet aralığını temizleme başarısız → spreadsheet_id: {}", spreadsheet_id))?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!("❌ Sheet aralığını temizleme başarısız, HTTP Status: {}", response.status()));
    }

    info!("✅ Sheet aralığı temizlendi → spreadsheet_id: '{}', range: '{}'", spreadsheet_id, range);

    Ok(())
}