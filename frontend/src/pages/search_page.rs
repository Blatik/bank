use yew::prelude::*;
use yew_router::prelude::*;
use crate::api::ApiClient;
use crate::models::{Country, Indicator, IndicatorCategory, CountryData};
use crate::components::{CountrySelector, IndicatorSelector, ChartViewer};
use crate::storage::StorageManager;
use serde_json::json;
use uuid::Uuid;

#[function_component(SearchPage)]
pub fn search_page() -> Html {
    let countries = use_state(Vec::<Country>::new);
    let selected_countries = use_state(Vec::<Country>::new);
    let indicators = use_state(Vec::<Vec<Indicator>>::new);
    let selected_indicator = use_state(Option::<Indicator>::new);
    let country_data = use_state(Vec::<CountryData>::new);
    let loading = use_state(|| false);

    let navigator = use_navigator().unwrap();

    // Load countries and indicators on mount
    {
        let countries = countries.clone();
        let indicators = indicators.clone();
        let loading = loading.clone();

        use_effect_with_deps(
            move |_| {
                let countries = countries.clone();
                let indicators = indicators.clone();
                let loading = loading.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    loading.set(true);
                    let client = ApiClient::new();

                    if let Ok(fetched_countries) = client.get_countries().await {
                        countries.set(fetched_countries);
                    }

                    if let Ok(fetched_indicators) = client.get_indicators().await {
                        indicators.set(vec![
                            fetched_indicators.economic,
                            fetched_indicators.demographic,
                            fetched_indicators.social,
                            fetched_indicators.environmental,
                        ]);
                    }

                    loading.set(false);
                });

                || ()
            },
            (),
        );
    }

    let on_country_select = {
        let selected_countries = selected_countries.clone();
        Callback::from(move |country: Country| {
            let mut new_selected = (*selected_countries).clone();
            if new_selected.len() < 3 {
                if !new_selected.iter().any(|c| c.id == country.id) {
                    new_selected.push(country);
                    selected_countries.set(new_selected);
                }
            }
        })
    };

    let on_indicator_select = {
        let selected_indicator = selected_indicator.clone();
        Callback::from(move |indicator: Indicator| {
            selected_indicator.set(Some(indicator));
        })
    };

    // Load data when countries or indicator changes
    {
        let country_data = country_data.clone();
        let selected_countries = selected_countries.clone();
        let selected_indicator = selected_indicator.clone();
        let loading = loading.clone();

        use_effect_with_deps(
            move |_| {
                let country_data = country_data.clone();
                let selected_countries = selected_countries.clone();
                let selected_indicator = selected_indicator.clone();
                let loading = loading.clone();

                wasm_bindgen_futures::spawn_local(async move {
                    if let Some(indicator) = (*selected_indicator).as_ref() {
                        loading.set(true);
                        let client = ApiClient::new();
                        let mut data = Vec::new();

                        for country in selected_countries.iter() {
                            if let Ok(country_data_item) = client.get_data(&country.id, &indicator.id).await {
                                data.push(country_data_item);
                            }
                        }

                        country_data.set(data);
                        loading.set(false);
                    }
                });

                || ()
            },
            ((*selected_countries).clone(), (*selected_indicator).clone()),
        );
    }

    let on_add_favorite = {
        Callback::from(move |_| {
            // Implementation for adding favorite
        })
    };

    let on_remove_country = {
        let selected_countries = selected_countries.clone();
        Callback::from(move |idx: usize| {
            let mut new_selected = (*selected_countries).clone();
            new_selected.remove(idx);
            selected_countries.set(new_selected);
        })
    };

    html! {
        <div class="search-page">
            <div class="controls-section">
                <CountrySelector
                    countries={(*countries).clone()}
                    selected_count={selected_countries.len()}
                    on_select={on_country_select}
                />

                <div class="selected-countries">
                    <h3>{"Selected Countries"}</h3>
                    <div class="country-pills">
                        {selected_countries.iter().enumerate().map(|(idx, country)| {
                            let on_click = {
                                let on_remove_country = on_remove_country.clone();
                                Callback::from(move |_| {
                                    on_remove_country.emit(idx);
                                })
                            };

                            html! {
                                <div key={country.id.clone()} class="country-pill">
                                    <span>{&country.name}</span>
                                    <button onclick={on_click} class="remove-btn">{"×"}</button>
                                </div>
                            }
                        }).collect::<Html>()}
                    </div>
                </div>

                <IndicatorSelector
                    categories={vec![
                        "Economic".to_string(),
                        "Demographic".to_string(),
                        "Social".to_string(),
                        "Environmental".to_string(),
                    ]}
                    indicators={(*indicators).clone()}
                    on_select={on_indicator_select}
                />

                <button class="action-btn" onclick={on_add_favorite}>
                    {"⭐ Add to Favorites"}
                </button>
            </div>

            <div class="chart-section">
                {if *loading {
                    html! { <div class="loading">{"Loading..."}</div> }
                } else {
                    html! { <ChartViewer data={(*country_data).clone()} /> }
                }}
            </div>
        </div>
    }
}
