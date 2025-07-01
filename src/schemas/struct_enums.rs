use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)] // Serde'nin JSON çıktısını düzleştirmesi için
pub enum ModifierGroupIdsEnum {
    Ids(Vec<i64>),
    None(String), // örnek: "-" olarak tutulacak
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)] // Serde'nin JSON çıktısını düzleştirmesi için
pub enum ModifierGroupIdsEnumString {
    Ids(Vec<String>),
    None(String), // örnek: "-" olarak tutulacak
}

#[derive(Clone)]
pub enum RoofStep {
    KeyFiltrele,
    MenuGetir,
    Flagger,
    IstekAt,
}

pub struct RoofStepResult {
    pub step: RoofStep,
    pub result: Result<(), String>,
}