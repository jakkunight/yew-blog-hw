use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/login")]
    Login,
    #[at("/echo")]
    Echo,
    #[at("/blog")]
    Blog,
    #[at("/not-found")]
    #[not_found]
    NotFound,
}

use crate::routes::*;

pub fn router(route: Route) -> Html {
    match route {
        Route::Index => html! {
            <Blog />
        },
        Route::Login => html! {
            <Login />
        },
        Route::Echo => html! {
            <Echo />
        },
        Route::Blog => html! {
            <Blog />
        },
        Route::NotFound => html! {
            <NotFound />
        },
    }
}

#[function_component]
pub fn Router() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={router} />
        </BrowserRouter>
    }
}
