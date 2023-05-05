use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;

use components::message::{
    MessageBlock,
};

use components::input::{
    InputBlock,
};

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/test")]
    Test,
    #[not_found]
    #[at("/404")]
    NotFound,
}


#[function_component(Home)]
fn home() -> Html {
    let MessageBlocks: Vec<_> = (0..3).map(|_| html_nested!{<MessageBlock/>}).collect();
    let navigator = use_navigator().unwrap();
    let to_test = Callback::from(move |_| navigator.push(&Route::Test));
    html! {
        <>
            <div class="container">
                <h1>{ "test!" }</h1>
                <button onclick={to_test}>{ "to test!!" }</button>
            </div>
            <div>
                <h1>{ "hey hehe" }</h1>
                <h2>{ "hey hehe but smaller" }</h2>
                { for MessageBlocks }
            </div>
            <div>
                <InputBlock/>
            </div>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::Test => html! { <h1>{ "IT WORKED" }</h1>},
        Route::NotFound => html! {<h1>{ "404 lol" }</h1>},
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}