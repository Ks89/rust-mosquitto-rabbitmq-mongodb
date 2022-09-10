use serde::{Deserialize, Serialize};

use crate::models::payload_trait::PayloadTrait;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Notification<T: PayloadTrait> {
    pub uuid: String,
    pub profile_token: String,
    pub payload: T,
}
