use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Temperature {
    pub temperature: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Humidity {
    pub humidity: f32,
}

pub trait PayloadTrait {}
impl PayloadTrait for Temperature {}
impl PayloadTrait for Humidity {}
