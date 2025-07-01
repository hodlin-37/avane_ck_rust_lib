use serde::{Deserialize, Serialize};
use crate::utils::input::{string_to_f64, string_to_i64, string_to_option_i64};
use crate::schemas::struct_enums::ModifierGroupIdsEnum;

#[derive(Clone, Deserialize, Debug)]
pub struct MigrosKeysRow {
    pub chain_id: i64,
    pub store_id: i64,
    pub menu_id: i64,
    pub brand_name: String,
    pub brand_name_platform: String,
    pub branch_name: String,
    pub branch_brand_name: String,
    pub branch_name_platform: String,
    pub restaurant_key: String,
}

#[derive(Clone, Serialize, Debug)]
pub struct MigrosPayloadMenuDetails {
    #[serde(rename = "storeId")]
    pub store_id: i64,
    #[serde(rename = "storeGroupId")]
    pub store_group_id: i64,
}

#[derive(Clone, Serialize, Debug)]
pub struct MigrosPayloadOptions {
    #[serde(rename = "menuId")]
    pub menu_id: i64,
}

#[derive(Clone, Serialize, Debug)]
pub struct MigrosEncryptedPayload {
    pub value: String,
}

#[derive(Clone, Serialize, Debug, Deserialize)]
pub struct MigrosActiveMenu {
    pub r#type: String,
    pub restaurant_id: i64,
    pub category_id: i64,
    pub header_info_id: i64,
    pub product_id: i64,
    pub modifier_group_ids: ModifierGroupIdsEnum,
    pub status: bool,
    pub urun_id: Option<i64>,
    pub flag: Option<bool>,
    pub x_api_key: String,
}

#[derive(Debug, Deserialize)]
pub struct MigrosResponseMenuDetails {
    pub data: MigrosResponseMenuDetailsData,
}

#[derive(Debug, Deserialize)]
pub struct MigrosResponseMenuDetailsData {
    #[serde(rename = "menuHeaderInfos")]
    pub menu_header_infos: Vec<MigrosResponseMenuHeaderInfos>,
}

#[derive(Debug, Deserialize)]
pub struct MigrosResponseMenuHeaderInfos {
    pub id: i64,
    #[serde(rename = "foodMenuItemDetailsDTOs")]
    pub food_menu_item_details_dtos: Vec<MigrosResponseFoodMenuItemDetailsDTOs>,
}

#[derive(Debug, Deserialize)]
pub struct MigrosResponseFoodMenuItemDetailsDTOs {
    #[serde(rename = "productId")]
    pub product_id: i64,
    pub status: String,
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct MigrosResponseOptionsDetails {
    pub data: Vec<MigrosResponseOptionsDetailsData>,
}

#[derive(Debug, Deserialize)]
pub struct MigrosResponseOptionsDetailsData {
    #[serde(rename = "optionsInfo")]
    pub options_info: MigrosResponseOptionsInfo,
}

#[derive(Debug, Deserialize)]
pub struct MigrosResponseOptionsInfo {
    #[serde(rename = "objectOptionHeaderInfosV2")]
    pub object_option_header_infos_v2: Vec<MigrosResponseObjectOptionHeaderInfosV2>,
}

#[derive(Debug, Deserialize)]
pub struct MigrosResponseObjectOptionHeaderInfosV2 {
    #[serde(rename = "optionHeaderDTO")]
    pub option_header_dto: MigrosResponseOptionHeaderDTO,
    #[serde(rename = "ownerId")]
    pub owner_id: i64,
    #[serde(rename = "objectOptionItemInfosV2")]
    pub object_option_item_infos_v2: Vec<MigrosResponseObjectOptionItemInfosV2>,
}

#[derive(Debug, Deserialize)]
pub struct MigrosResponseOptionHeaderDTO {
    pub id:i64,
}

#[derive(Debug, Deserialize)]
pub struct MigrosResponseObjectOptionItemInfosV2 {
    #[serde(rename = "optionItemDTO")]
    pub option_item_dto: MigrosResponseOptionItemDTO,
    pub id: i64,
}

#[derive(Debug, Deserialize)]
pub struct MigrosResponseOptionItemDTO {
    pub id: i64,
    pub status: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MigrosBranchJsonRow {
    pub restaurant_key: String,
    pub integration_name: String,
    pub r#type: String,
    #[serde(deserialize_with = "string_to_i64")]
    pub restaurant_id: i64,
    #[serde(deserialize_with = "string_to_i64")]
    pub product_id: i64,
    pub product_name: String,
    pub category_name: String,
    #[serde(deserialize_with = "string_to_f64")]
    pub price: f64,
    pub description: Option<String>,
    #[serde(default, deserialize_with = "string_to_option_i64")]
    pub urun_id: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MigrosBranchJsonResponse {
    pub data: Vec<MigrosBranchJsonRow>,
}

#[derive(Debug, Clone, Serialize)]
pub struct MigrosPayloadStatusOptions {
    #[serde(rename = "storeId")]
    pub store_id: i64,
    #[serde(rename = "optionItemId")]
    pub option_item_id: i64,
}

#[derive(Debug, Clone, Serialize)]
pub struct MigrosPayloadStatusProduct {
    #[serde(rename = "storeId")]
    pub store_id: i64,
    #[serde(rename = "productId")]
    pub product_id: i64,
    pub status: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MigrosActivateResponse {
    pub success: bool,
    #[serde(rename = "errorMessage")]
    pub error_message: Option<MigrosErrorMessage>,
    #[serde(rename = "validatonErrorMessages")]
    pub validaton_error_messages: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MigrosErrorMessage {
    #[serde(rename = "errorCode")]
    pub error_code: String,
    #[serde(rename = "errorTitle")]
    pub error_title: String,
    #[serde(rename = "errorDetail")]
    pub error_detail: String,
    #[serde(rename = "isSystemError")]
    pub is_system_error: bool,
}