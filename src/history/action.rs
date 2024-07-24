use serde_json::{json, Value as Json};

use hyperborealib::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
/// ## Actions
/// 
/// ### Servers
/// 
/// - `v1.server.passport.update` - update passport field value.
/// - `v1.server.passport.delete` - delete passport field value.
/// 
/// ### Members
/// 
/// - `v1.members.passport.update` - update passport field value.
/// - `v1.members.passport.delete` - delete passport field value.
/// - `v1.members.messages.new`    - new chat message.
pub enum Action {
    /// Update server passport field.
    ServerPassportUpdate {
        field: String,
        value: Json,

        /// Signer must be a server owner
        /// or server administrator.
        signer: PublicKey,
        sign: String
    },

    /// Delete server passport field.
    ServerPassportDelete {
        field: String
    },

    /// Update member passport field
    MembersPassportUpdate {
        field: String,
        value: Json,

        /// Must be signed by the member himself.
        sign: String
    },

    /// Delete member passport field.
    MembersPassportDelete {
        field: String
    },

    /// Create new message.
    MembersMessagesNew {
        channel_id: u64,
        message: String
    }
}

impl AsJson for Action {
    fn to_json(&self) -> Result<Json, AsJsonError> {
        match self {
            Self::ServerPassportUpdate { field, value, signer, sign } => {
                Ok(json!({
                    "type": "v1.server.passport.update",
                    "body": {
                        "field": field,
                        "value": value,
                        "signer": signer.to_base64(),
                        "sign": sign
                    }
                }))
            }

            Self::ServerPassportDelete { field } => {
                Ok(json!({
                    "type": "v1.server.passport.delete",
                    "body": {
                        "field": field
                    }
                }))
            }

            Self::MembersPassportUpdate { field, value, sign } => {
                Ok(json!({
                    "type": "v1.members.passport.update",
                    "body": {
                        "field": field,
                        "value": value,
                        "sign": sign
                    }
                }))
            }

            Self::MembersPassportDelete { field } => {
                Ok(json!({
                    "type": "v1.members.passport.delete",
                    "body": {
                        "field": field
                    }
                }))
            }

            Self::MembersMessagesNew { channel_id, message } => {
                Ok(json!({
                    "type": "v1.members.messages.new",
                    "body": {
                        "channel_id": channel_id,
                        "message": message
                    }
                }))
            }
        }
    }

    fn from_json(json: &Json) -> Result<Self, AsJsonError> where Self: Sized {
        let Some(action) = json.get("type").and_then(Json::as_str) else {
            return Err(AsJsonError::FieldNotFound("type"));
        };

        let Some(body) = json.get("body") else {
            return Err(AsJsonError::FieldNotFound("body"));
        };

        match action {
            "v1.server.passport.update" => {
                Ok(Self::ServerPassportUpdate {
                    field: body.get("field")
                        .and_then(Json::as_str)
                        .map(String::from)
                        .ok_or_else(|| AsJsonError::FieldNotFound("body.field"))?,

                    value: body.get("value")
                        .ok_or_else(|| AsJsonError::FieldNotFound("body.value"))?
                        .clone(),

                    signer: body.get("sign")
                        .and_then(Json::as_str)
                        .map(PublicKey::from_base64)
                        .ok_or_else(|| AsJsonError::FieldNotFound("body.signer"))??,

                    sign: body.get("sign")
                        .and_then(Json::as_str)
                        .map(String::from)
                        .ok_or_else(|| AsJsonError::FieldNotFound("body.sign"))?
                })
            }

            "v1.server.passport.delete" => {
                Ok(Self::ServerPassportDelete {
                    field: body.get("field")
                        .and_then(Json::as_str)
                        .map(String::from)
                        .ok_or_else(|| AsJsonError::FieldNotFound("body.field"))?
                })
            }

            "v1.members.passport.update" => {
                Ok(Self::MembersPassportUpdate {
                    field: body.get("field")
                        .and_then(Json::as_str)
                        .map(String::from)
                        .ok_or_else(|| AsJsonError::FieldNotFound("body.field"))?,

                    value: body.get("value")
                        .ok_or_else(|| AsJsonError::FieldNotFound("body.value"))?
                        .clone(),

                    sign: body.get("sign")
                        .and_then(Json::as_str)
                        .map(String::from)
                        .ok_or_else(|| AsJsonError::FieldNotFound("body.sign"))?
                })
            }

            "v1.members.passport.delete" => {
                Ok(Self::MembersPassportDelete {
                    field: body.get("field")
                        .and_then(Json::as_str)
                        .map(String::from)
                        .ok_or_else(|| AsJsonError::FieldNotFound("body.field"))?
                })
            }

            "v1.members.messages.new" => {
                Ok(Self::MembersMessagesNew {
                    channel_id: body.get("channel_id")
                        .and_then(Json::as_u64)
                        .ok_or_else(|| AsJsonError::FieldNotFound("body.channel_id"))?,

                    message: body.get("message")
                        .and_then(Json::as_str)
                        .map(String::from)
                        .ok_or_else(|| AsJsonError::FieldNotFound("body.message"))?
                })
            }

            _ => Err(AsJsonError::FieldValueInvalid("type"))
        }
    }
}
