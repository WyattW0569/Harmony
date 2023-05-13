use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use uuid::Uuid;

mod components;

use components::websocket::{
    WebSocketClient,
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

fn gen_uuid() -> Uuid{ 
    let id = Uuid::new_v4();
    id
}

async fn open_lobby(id: Uuid) {
    let url = format!("http://localhost/api/{}", id.to_string());
    return Request::get(&url).send().await.unwrap().json().await.unwrap();
}


#[function_component(Home)]
fn home() -> Html {
    let navigator = use_navigator().unwrap();
    let to_test = Callback::from(move |_| navigator.push(&Route::Test));
    html! {
        <body >
            <h1 class ="centre">
                { "Harmony" }
            </h1>
            <div class="container">
                <button onclick={to_test}>{ "Create Lobby" }</button>
            </div>
        </body>
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
#[function_component(Lobby)]
fn test() -> Html {

    let id = gen_uuid();
    open_lobby(id);
    
    html! {
        <>
            <h1>{ format!("Lobby on -  {}",id) }</h1>
            <WebSocketClient url={format!("ws://10.0.0.15/api/{}",id)}/>
        </>
    }
}
 

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::Test => html! {<Lobby/>},
        Route::NotFound => html! {<h1>{ "404 lol" }</h1>},
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}