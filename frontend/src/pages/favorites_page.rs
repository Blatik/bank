use yew::prelude::*;

#[function_component(FavoritesPage)]
pub fn favorites_page() -> Html {
    html! {
        <div class="favorites-page">
            <h1>{"Favorites"}</h1>
            <p>{"Your favorite searches will appear here..."}</p>
        </div>
    }
}
