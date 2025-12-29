use yew::prelude::*;
use crate::models::{DataPoint, CountryData};

#[derive(Properties, PartialEq)]
pub struct ChartViewerProps {
    pub data: Vec<CountryData>,
}

#[function_component(ChartViewer)]
pub fn chart_viewer(props: &ChartViewerProps) -> Html {
    if props.data.is_empty() {
        return html! {
            <div class="chart-container">
                <p>{"Select countries and indicator to view data"}</p>
            </div>
        };
    }

    let max_value = props.data
        .iter()
        .flat_map(|d| d.data.iter())
        .map(|p| p.value)
        .fold(f64::NEG_INFINITY, f64::max);

    let min_value = props.data
        .iter()
        .flat_map(|d| d.data.iter())
        .map(|p| p.value)
        .fold(f64::INFINITY, f64::min);

    html! {
        <div class="chart-container">
            <h2>{props.data.first().map(|d| &d.indicator_name).unwrap_or(&"Chart".to_string())}</h2>
            
            <div class="chart-legend">
                {props.data.iter().map(|country_data| {
                    html! {
                        <div key={country_data.country_id.clone()} class="legend-item">
                            <span class="legend-color"></span>
                            <span>{&country_data.country_name}</span>
                        </div>
                    }
                }).collect::<Html>()}
            </div>

            <div class="chart-bars">
                {props.data.first().map(|first_data| {
                    first_data.data.iter().enumerate().map(|(idx, _)| {
                        let year_str = props.data.first()
                            .and_then(|d| d.data.get(idx))
                            .map(|p| &p.year)
                            .unwrap_or(&"Unknown".to_string());

                        html! {
                            <div key={idx} class="bar-group">
                                <div class="bar-label">{year_str}</div>
                                {props.data.iter().map(|country_data| {
                                    if let Some(point) = country_data.data.get(idx) {
                                        let percentage = if (max_value - min_value).abs() > 0.1 {
                                            ((point.value - min_value) / (max_value - min_value)) * 100.0
                                        } else {
                                            50.0
                                        };

                                        html! {
                                            <div key={country_data.country_id.clone()} 
                                                 class="bar" 
                                                 style={format!("height: {}%", percentage)}>
                                                <div class="bar-value">{format!("{:.2}", point.value)}</div>
                                            </div>
                                        }
                                    } else {
                                        html! { <div key={country_data.country_id.clone()} class="bar empty"></div> }
                                    }
                                }).collect::<Html>()}
                            </div>
                        }
                    }).collect::<Html>()
                }).unwrap_or_default()}
            </div>
        </div>
    }
}
