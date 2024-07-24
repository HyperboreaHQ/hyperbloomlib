use std::hash::Hasher;

use serde_json::{json, Value as Json};

use hyperborealib::prelude::*;

use super::Action;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub author: PublicKey,
    pub action: Action,
    pub sign: String
}

impl Block {
    /// Calculate [seahash](https://crates.io/crates/seahash) of the block.
    pub fn hash(&self) -> u64 {
        let mut hasher = seahash::SeaHasher::new();

        // Hash author
        for byte in self.author.to_bytes() {
            hasher.write_u8(byte);
        }

        // Hash action sign
        for byte in self.sign.bytes() {
            hasher.write_u8(byte);
        }

        hasher.finish()
    }
}

impl AsJson for Block {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        Ok(json!({
            "author": self.author.to_base64(),
            "action": self.action.to_json()?,
            "sign": self.sign
        }))
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        Ok(Self {
            author: json.get("author")
                .and_then(Json::as_str)
                .map(PublicKey::from_base64)
                .ok_or_else(|| AsJsonError::FieldNotFound("author"))??,

            action: json.get("action")
                .ok_or_else(|| AsJsonError::FieldNotFound("action"))
                .and_then(Action::from_json)?,

            sign: json.get("sign")
                .and_then(Json::as_str)
                .map(String::from)
                .ok_or_else(|| AsJsonError::FieldNotFound("sign"))?
        })
    }
}
