use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::check::{check_date, check_field};

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Trip {
    pub id: u32,
    #[validate(custom = "check_field")]
    pub name: String,
    #[validate(custom = "check_date")]
    pub date: String,
    #[validate(custom = "check_field")]
    pub destination: String,
    pub risk_assessment: String,
    #[validate(custom = "check_field")]
    pub description: String,
    pub vehicle: Vehicle,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Vehicle {
    Car,
    Plane,
    Train,
    Helicopter,
    Subway,
    #[serde(rename = "UFO")]
    Ufo,
    Other,
}

#[derive(Debug, Clone, Deserialize, Serialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct Expense {
    #[validate(custom = "check_field")]
    pub username: String,
    #[validate]
    pub trips: Vec<Trip>,
}
