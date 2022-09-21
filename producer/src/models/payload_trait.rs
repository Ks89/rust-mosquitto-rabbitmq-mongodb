use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Temperature {
    pub value: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Humidity {
    pub value: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Light {
    pub value: f32,
}

pub trait PayloadTrait {
    fn get_value(&self) -> f32;
}

impl PayloadTrait for Temperature {
    fn get_value(&self) -> f32 {
        self.value
    }
}

impl PayloadTrait for Humidity {
    fn get_value(&self) -> f32 {
        self.value
    }
}

impl PayloadTrait for Light {
    fn get_value(&self) -> f32 {
        self.value
    }
}

