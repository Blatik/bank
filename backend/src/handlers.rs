use crate::api::ApiClient;
use crate::data::get_indicators;
use crate::errors::ApiError;
use crate::models::{ComparisonRequest, ComparisonResponse, Country, IndicatorCategory};
use actix_web::{web, HttpResponse};
use std::collections::HashMap;
use std::sync::Arc;

pub async fn get_countries(
    api_client: web::Data<Arc<ApiClient>>,
) -> Result<HttpResponse, ApiError> {
    let countries = api_client.get_countries().await?;
    Ok(HttpResponse::Ok().json(countries))
}

pub async fn get_indicators() -> HttpResponse {
    let indicators = get_indicators();
    HttpResponse::Ok().json(indicators)
}

pub async fn get_data(
    api_client: web::Data<Arc<ApiClient>>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, ApiError> {
    let (country_id, indicator_id) = path.into_inner();
    let data = api_client
        .get_country_data(&country_id, &indicator_id)
        .await?;
    Ok(HttpResponse::Ok().json(data))
}

pub async fn compare_countries(
    api_client: web::Data<Arc<ApiClient>>,
    req: web::Json<ComparisonRequest>,
) -> Result<HttpResponse, ApiError> {
    let mut response_countries = Vec::new();
    let mut data_map: HashMap<String, HashMap<String, Vec<_>>> = HashMap::new();

    for country_id in &req.countries {
        let countries = api_client.get_countries().await?;
        if let Some(country) = countries.iter().find(|c| &c.id == country_id) {
            response_countries.push(country.clone());
        }
    }

    for country_id in &req.countries {
        for indicator_id in &req.indicators {
            if let Ok(data) = api_client.get_country_data(country_id, indicator_id).await {
                data_map
                    .entry(country_id.clone())
                    .or_insert_with(HashMap::new)
                    .insert(indicator_id.clone(), data.data);
            }
        }
    }

    let comparison = ComparisonResponse {
        countries: response_countries,
        data: data_map,
    };

    Ok(HttpResponse::Ok().json(comparison))
}
