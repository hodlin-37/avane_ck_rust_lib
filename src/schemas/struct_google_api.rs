use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ServiceAccountCredentials {
    pub r#type: String,
    pub project_id: String,
    pub private_key_id: String,
    pub private_key: String,
    pub client_email: String,
    pub client_id: String,
    pub auth_uri: String,
    pub token_uri: String,
    pub auth_provider_x509_cert_url: String,
    pub client_x509_cert_url: String,
    pub universe_domain: String,
}

#[derive(Debug, Serialize)]
pub struct JWTClaims<'a> {
    pub iss: &'a str, // Issuer
    pub scope: &'a str, // Scopes
    pub aud: &'a str, // Audience
    pub exp: usize, // Expiration time
    pub iat: usize, // Issued at time
}

#[derive(Debug, Deserialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: usize,
    pub token_type: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleSheetResponse {
    #[serde(default)]
    pub range: Option<String>,

    #[serde(rename = "majorDimension", default)]
    pub major_dimension: Option<String>,

    pub values: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct GoogleDriveFileInfo{
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct GoogleDriveFileListResponse {
    pub files: Vec<GoogleDriveFileInfo>,
}