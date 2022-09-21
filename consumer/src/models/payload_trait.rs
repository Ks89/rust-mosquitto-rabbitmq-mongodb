use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Temperature {
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Humidity {
    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Light {
    pub value: f64,
}

pub trait PayloadTrait {
    fn get_value(&self) -> f64;
}

impl PayloadTrait for Temperature {
    fn get_value(&self) -> f64 {
        self.value
    }
}

impl PayloadTrait for Humidity {
    fn get_value(&self) -> f64 {
        self.value
    }
}

impl PayloadTrait for Light {
    fn get_value(&self) -> f64 {
        self.value
    }
}
