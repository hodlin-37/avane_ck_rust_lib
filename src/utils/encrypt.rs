use aes::Aes256;
use block_modes::{BlockMode, Ecb};
use block_modes::block_padding::Pkcs7;
use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;
use serde_json;
use log::trace;
use anyhow::{Context, Result};

type Aes256Ecb = Ecb<Aes256, Pkcs7>;

pub fn encrypt_migros<T: Serialize>(data: &T, secret_key: &str) -> Result<String> {

    // 1. JSON'a çevir
    let json_string = serde_json::to_string(data)
        .context("❌ Veri JSON'a çevrilemedi")?;

    // 2. Byte dizisine çevir
    let json_bytes = json_string.as_bytes();

    // 3. Şifreleyici oluştur
    let secret_key_bytes = secret_key.as_bytes();
    let cipher = Aes256Ecb::new_from_slices(secret_key_bytes, &[])
        .context("❌ Şifreleyici (AES256 ECB) oluşturulamadı")?;

    // 4. Şifreleme
    let cipher_text = cipher.encrypt_vec(json_bytes);

    // 5. Base64'e çevir
    let encoded = general_purpose::STANDARD.encode(cipher_text);
    trace!("Data: {:?}, Encrypted Key: {:?}", json_string, encoded);

    Ok(encoded)
}
