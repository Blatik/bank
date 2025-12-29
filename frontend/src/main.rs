mod components;
mod pages;
mod api;
mod models;
mod storage;

use yew::prelude::*;
use yew_router::prelude::*;
use pages::{SearchPage, ComparisonPage, FavoritesPage};

#[derive(Clone, Routable, PartialEq, Eq)]
enum Route {
    #[at("/")]
    Search,
    #[at("/comparison")]
    Comparison,
    #[at("/favorites")]
    Favorites,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Search => html! { <SearchPage /> },
        Route::Comparison => html! { <ComparisonPage /> },
        Route::Favorites => html! { <FavoritesPage /> },
        Route::NotFound => html! { <h1>{"404 - Page not found"}</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <div class="app-container">
                <header class="app-header">
                    <h1>{"🌍 World Analytics"}</h1>
                    <nav class="app-nav">
                        <Link<Route> to={Route::Search}>{"Search"}</Link<Route>>
                        <Link<Route> to={Route::Comparison}>{"Compare"}</Link<Route>>
                        <Link<Route> to={Route::Favorites}>{"Favorites"}</Link<Route>>
                    </nav>
                </header>
                <main class="app-main">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
