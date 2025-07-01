use std::collections::{HashMap, HashSet};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

use anyhow::{Context, Result, anyhow, bail};
use futures::stream::{FuturesUnordered, StreamExt};
use log::{error, info, warn};
use serde_json::{Value, json};
use tokio::{
    fs,
    fs::File,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    sync::Mutex,
    time::{Duration, sleep},
};

use crate::schemas::struct_migros::{MigrosKeysRow};
use crate::services::google_api_sheet::{get_sheet_values};

pub const KEYS_SHEET_ID: &str = "1accka-4YjSUwd27UNgG3xcpWy4Inz0W4E_NVNfT3-xk";
pub const MIGROS_KEYS_RANGE: &str = "MİGROS!A2:I";

pub fn parse_migros_keys(row: &[String]) -> Result<MigrosKeysRow> {
    if row.len() < 9 {
        bail!(
            "❌ Satırda beklenen en az 9 sütun var, ama {} bulundu.",
            row.len()
        );
    }

    Ok(MigrosKeysRow {
        chain_id: row[0]
            .parse::<i64>()
            .with_context(|| format!("❌ chain_id değeri sayıya çevrilemedi → '{}'", row[0]))?,
        store_id: row[1]
            .parse::<i64>()
            .with_context(|| format!("❌ store_id değeri sayıya çevrilemedi → '{}'", row[1]))?,
        menu_id: row[2]
            .parse::<i64>()
            .with_context(|| format!("❌ menu_id değeri sayıya çevrilemedi → '{}'", row[2]))?,
        brand_name: row[3].clone(),
        brand_name_platform: row[4].clone(),
        branch_name: row[5].clone(),
        branch_brand_name: row[6].clone(),
        branch_name_platform: row[7].clone(),
        restaurant_key: row[8].clone(),
    })
}

pub async fn filter_migros_keys_by_branch(
    auth_token: &str,
    branch_name: &str,
) -> Result<Vec<MigrosKeysRow>> {
    let sheet_values = get_sheet_values(KEYS_SHEET_ID, MIGROS_KEYS_RANGE, auth_token)
        .await
        .with_context(|| "❌ Migros key sheet verileri alınamadı")?;

    let mut parsed_keys = vec![];
    let mut skipped_count = 0;

    for (i, row) in sheet_values.values.iter().enumerate() {
        match parse_migros_keys(row) {
            Ok(parsed) => parsed_keys.push(parsed),
            Err(e) => {
                skipped_count += 1;
                warn!("⚠️ Satır {} parse edilemedi → Hata: {:?}", i + 1, e);
            }
        }
    }

    let filtered_keys: Vec<MigrosKeysRow> = parsed_keys
        .into_iter()
        .filter(|key| key.branch_name == branch_name)
        .collect();

    info!(
        "✅ '{}' şubesi için {} anahtar bulundu. Atlanan satır sayısı: {}",
        branch_name,
        filtered_keys.len(),
        skipped_count
    );

    Ok(filtered_keys)
}