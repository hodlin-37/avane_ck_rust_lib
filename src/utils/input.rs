use std::fs::File;
use std::io::Read;
use serde_json::Value;
use serde::{Deserializer, Deserialize};

pub fn find_value_by_key_in_json(path: &str, target_key: &str) -> Option<String> {
    let mut file = File::open(path).ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).ok()?;

    let json: Value = serde_json::from_str(&contents).ok()?;

    // Sadece string olan değerleri döndür
    match json.get(target_key)?.as_str() {
        Some(s) => Some(s.to_string()),
        None => None,
    }
}

pub fn string_to_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<i64>().map_err(serde::de::Error::custom)
}

pub fn string_to_f64<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<f64>().map_err(serde::de::Error::custom)
}

pub fn string_to_option_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Value::deserialize(deserializer)?;

    match value {
        Value::Number(n) => Ok(n.as_i64()),
        Value::String(s) => {
            // Sadece tamamen sayılardan oluşuyorsa parse etmeyi dene
            if s.chars().all(|c| c.is_ascii_digit()) {
                s.parse::<i64>().ok().map_or(Ok(None), |v| Ok(Some(v)))
            } else {
                Ok(None)
            }
        }
        Value::Null => Ok(None),
        _ => Ok(None),
    }
}

pub fn split_pipe_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let items = s
        .split('|')
        .map(|item| item.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    Ok(items)
}

// Custom deserializer for Option<Vec<i64>>
pub fn string_to_option_vec_i64<'de, D>(deserializer: D) -> Result<Option<Vec<i64>>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<serde_json::Value>::deserialize(deserializer)?;
    match opt {
        Some(serde_json::Value::String(s)) => {
            // Try to parse comma-separated string into Vec<i64>
            let vec: Vec<i64> = s
                .split(',')
                .filter_map(|x| x.trim().parse::<i64>().ok())
                .collect();
            if vec.is_empty() {
                Ok(None)
            } else {
                Ok(Some(vec))
            }
        }
        Some(serde_json::Value::Array(arr)) => {
            let mut vec = Vec::new();
            for v in arr {
                match v {
                    serde_json::Value::Number(num) => {
                        if let Some(i) = num.as_i64() {
                            vec.push(i);
                        }
                    }
                    serde_json::Value::String(s) => {
                        if let Ok(i) = s.parse::<i64>() {
                            vec.push(i);
                        }
                    }
                    _ => {}
                }
            }
            if vec.is_empty() {
                Ok(None)
            } else {
                Ok(Some(vec))
            }
        }
        Some(serde_json::Value::Number(num)) => {
            if let Some(i) = num.as_i64() {
                Ok(Some(vec![i]))
            } else {
                Ok(None)
            }
        }
        Some(_) => Ok(None),
        None => Ok(None),
    }
}