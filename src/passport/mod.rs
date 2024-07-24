use std::collections::HashMap;

use serde_json::Value as Json;

use hyperborealib::prelude::*;

#[derive(Debug, thiserror::Error)]
pub enum PassportError {
    #[error(transparent)]
    Serialize(#[from] serde_json::Error),

    #[error(transparent)]
    Cryptography(#[from] CryptographyError)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassportValue {
    pub value: Json,
    pub sign: Vec<u8>
}

impl PassportValue {
    pub fn new(secret_key: &SecretKey, value: Json) -> Result<Self, PassportError> {
        let json = serde_json::to_vec(&value)?;

        Ok(Self {
            value,
            sign: secret_key.create_signature(json)
        })
    }

    pub fn validate(&self, public_key: &PublicKey) -> Result<bool, PassportError> {
        let json = serde_json::to_vec(&self.value)?;

        Ok(public_key.verify_signature(json, &self.sign)?)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Passport {
    owner: PublicKey,
    fields: HashMap<String, PassportValue>
}

impl Passport {
    pub fn new(owner: PublicKey) -> Self {
        Self {
            owner,
            fields: HashMap::new()
        }
    }

    pub fn set(&mut self, key: impl ToString, value: PassportValue) -> Result<bool, PassportError> {
        if value.validate(&self.owner)? {
            self.fields.insert(key.to_string(), value);

            return Ok(true);
        }

        Ok(false)
    }

    pub fn get(&self, key: impl AsRef<str>) -> Option<&PassportValue> {
        self.fields.get(key.as_ref())
    }

    pub fn remove(&mut self, key: impl AsRef<str>) {
        self.fields.remove(key.as_ref());
    }
}
