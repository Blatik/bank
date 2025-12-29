use yew::prelude::*;
use crate::models::Indicator;

#[derive(Properties, PartialEq)]
pub struct IndicatorSelectorProps {
    pub on_select: Callback<Indicator>,
    pub indicators: Vec<Vec<Indicator>>,
    pub categories: Vec<String>,
}

#[function_component(IndicatorSelector)]
pub fn indicator_selector(props: &IndicatorSelectorProps) -> Html {
    let active_category = use_state(|| 0);

    let on_category_click = {
        let active_category = active_category.clone();
        Callback::from(move |index: usize| {
            active_category.set(index);
        })
    };

    html! {
        <div class="indicator-selector">
            <h2>{"Select Indicator"}</h2>
            <div class="category-tabs">
                {props.categories.iter().enumerate().map(|(idx, category)| {
                    let is_active = *active_category == idx;
                    let on_click = {
                        let on_category_click = on_category_click.clone();
                        Callback::from(move |_| {
                            on_category_click.emit(idx);
                        })
                    };

                    let class = if is_active {
                        "category-tab active"
                    } else {
                        "category-tab"
                    };

                    html! {
                        <button
                            key={idx}
                            class={class}
                            onclick={on_click}
                        >
                            {category}
                        </button>
                    }
                }).collect::<Html>()}
            </div>

            <div class="indicator-list">
                {if let Some(indicators) = props.indicators.get(*active_category) {
                    indicators.iter().map(|indicator| {
                        let indicator_clone = indicator.clone();
                        let on_click = {
                            let on_select = props.on_select.clone();
                            Callback::from(move |_| {
                                on_select.emit(indicator_clone.clone());
                            })
                        };

                        html! {
                            <div key={indicator.id.clone()} class="indicator-item" onclick={on_click}>
                                <div class="indicator-name">{&indicator.name}</div>
                                <div class="indicator-unit">{&indicator.unit}</div>
                            </div>
                        }
                    }).collect::<Html>()
                } else {
                    html! { <div>{"No indicators available"}</div> }
                }}
            </div>
        </div>
    }
}
