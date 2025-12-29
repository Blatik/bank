use crate::models::{Country, CountryData, DataPoint};
use crate::errors::ApiError;
use reqwest::Client;
use serde_json::Value;
use std::sync::Arc;

const WB_API: &str = "https://api.worldbank.org/v2";

pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        ApiClient {
            client: Client::new(),
        }
    }

    pub async fn get_countries(&self) -> Result<Vec<Country>, ApiError> {
        let url = format!("{}/country?format=json&per_page=500", WB_API);
        let response = self.client.get(&url).send().await?;
        let data: Value = response.json().await?;

        let countries = data
            .get(1)
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|c| {
                let capital = c.get("capitalCity")?.as_str()?;
                if capital.is_empty() {
                    return None;
                }

                let id = c.get("id")?.as_str()?.to_string();
                let name = c.get("name")?.as_str()?.to_string();
                let region = c
                    .get("region")
                    .and_then(|r| r.get("value"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("Unknown")
                    .to_string();
                let capital = capital.to_string();

                Some(Country {
                    id: id.clone(),
                    name,
                    region,
                    capital,
                    code: id,
                })
            })
            .collect();

        Ok(countries)
    }

    pub async fn get_country_data(
        &self,
        country_id: &str,
        indicator_id: &str,
    ) -> Result<CountryData, ApiError> {
        let url = format!(
            "{}/country/{}/indicator/{}?format=json&per_page=500",
            WB_API, country_id, indicator_id
        );

        let response = self.client.get(&url).send().await?;
        let data: Value = response.json().await?;

        let country_name = data
            .get(1)
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("country"))
            .and_then(|c| c.get("value"))
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let indicator_name = data
            .get(1)
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|c| c.get("indicator"))
            .and_then(|c| c.get("value"))
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown")
            .to_string();

        let mut data_points: Vec<DataPoint> = data
            .get(1)
            .and_then(|v| v.as_array())
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|point| {
                let value = point.get("value")?.as_f64()?;
                let year = point.get("date")?.as_str()?.to_string();
                Some(DataPoint { year, value })
            })
            .collect();

        data_points.sort_by(|a, b| a.year.cmp(&b.year));

        if data_points.is_empty() {
            return Err(ApiError::NoDataAvailable);
        }

        Ok(CountryData {
            country_id: country_id.to_string(),
            country_name,
            indicator_id: indicator_id.to_string(),
            indicator_name,
            data: data_points,
        })
    }
}
