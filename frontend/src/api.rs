use gloo_net::http::Client;
use gloo_net::error::FetchError;
use crate::models::{Country, IndicatorCategory, CountryData, ComparisonRequest};
use serde_json::json;

const API_BASE: &str = "http://localhost:8080/api";

pub struct ApiClient {
    client: Client,
}

impl ApiClient {
    pub fn new() -> Self {
        ApiClient {
            client: Client::new(),
        }
    }

    pub async fn get_countries(&self) -> Result<Vec<Country>, FetchError> {
        let response = self
            .client
            .get(&format!("{}/countries", API_BASE))
            .send()
            .await?;
        response.json().await
    }

    pub async fn get_indicators(&self) -> Result<IndicatorCategory, FetchError> {
        let response = self
            .client
            .get(&format!("{}/indicators", API_BASE))
            .send()
            .await?;
        response.json().await
    }

    pub async fn get_data(&self, country_id: &str, indicator_id: &str) -> Result<CountryData, FetchError> {
        let response = self
            .client
            .get(&format!("{}/data/{}/{}", API_BASE, country_id, indicator_id))
            .send()
            .await?;
        response.json().await
    }

    pub async fn compare_countries(
        &self,
        countries: Vec<String>,
        indicators: Vec<String>,
    ) -> Result<serde_json::Value, FetchError> {
        let request = ComparisonRequest { countries, indicators };
        let response = self
            .client
            .post(&format!("{}/compare", API_BASE))
            .json(&request)
            .map_err(|_| FetchError::SerdeError)?
            .send()
            .await?;
        response.json().await
    }
}
