use yew::prelude::*;
use crate::models::Country;

#[derive(Properties, PartialEq)]
pub struct CountrySelectorProps {
    pub on_select: Callback<Country>,
    pub countries: Vec<Country>,
    pub selected_count: usize,
}

#[function_component(CountrySelector)]
pub fn country_selector(props: &CountrySelectorProps) -> Html {
    let search_input = use_state(|| String::new());

    let filtered_countries = {
        let search = (*search_input).clone();
        props.countries
            .iter()
            .filter(|c| {
                search.is_empty()
                    || c.name.to_lowercase().contains(&search.to_lowercase())
                    || c.code.to_lowercase().contains(&search.to_lowercase())
            })
            .cloned()
            .collect::<Vec<_>>()
    };

    let on_change = {
        let search_input = search_input.clone();
        Callback::from(move |e: Event| {
            if let Some(input) = e.target().and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                search_input.set(input.value());
            }
        })
    };

    html! {
        <div class="country-selector">
            <h2>{"Select Countries"} {" ("} {props.selected_count} {" / 3)"}</h2>
            <input
                type="text"
                placeholder="Search country..."
                value={(*search_input).clone()}
                onchange={on_change}
                class="search-input"
            />
            <div class="country-list">
                {filtered_countries.into_iter().map(|country| {
                    let country_clone = country.clone();
                    let on_click = {
                        let on_select = props.on_select.clone();
                        Callback::from(move |_| {
                            on_select.emit(country_clone.clone());
                        })
                    };

                    html! {
                        <div key={country.id.clone()} class="country-item" onclick={on_click}>
                            <div class="country-name">{&country.name}</div>
                            <div class="country-info">
                                <span>{&country.region}</span>
                                <span>{&country.capital}</span>
                            </div>
                        </div>
                    }
                }).collect::<Html>()}
            </div>
        </div>
    }
}
