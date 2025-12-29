use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Country {
    pub id: String,
    pub name: String,
    pub region: String,
    pub capital: String,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Indicator {
    pub id: String,
    pub name: String,
    pub category: String,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndicatorCategory {
    pub economic: Vec<Indicator>,
    pub demographic: Vec<Indicator>,
    pub social: Vec<Indicator>,
    pub environmental: Vec<Indicator>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub year: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountryData {
    pub country_id: String,
    pub country_name: String,
    pub indicator_id: String,
    pub indicator_name: String,
    pub data: Vec<DataPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonRequest {
    pub countries: Vec<String>,
    pub indicators: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResponse {
    pub countries: Vec<Country>,
    pub data: HashMap<String, HashMap<String, Vec<DataPoint>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}
