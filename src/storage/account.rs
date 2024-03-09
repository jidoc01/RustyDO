use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Account {
    pub id: String,
    pub pw: String,
    pub name: String,
    pub bits: u32,
    pub exps: [u32; 8],
    pub items: [u8; 4],
    pub setting: AccountSetting,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AccountSetting {
    pub macro_texts: [String; 8],
    pub key_type: u8,
}

impl Default for AccountSetting {
    fn default() -> Self {
        Self {
            macro_texts: [
                "지덕".into(),
                "쥐덕".into(),
                "더덕".into(),
                "철푸덕".into(),
                "호더덕".into(),
                "을지문덕".into(),
                "도날드덕".into(),
                "푸더더덕".into(),
            ],
            key_type: 1u8,
        }
    }
}