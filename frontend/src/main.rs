use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use uuid::Uuid;

mod components;

use components::message::{
    MessageBlock,
};

use components::input::{
    TextInput,
};

use components::websocket::{
    WebSocketClient,
};

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/room")]
    Room,
    #[not_found]
    #[at("/404")]
    NotFound,
}

async fn open_lobby(id: Uuid) {
    let url = format!("http://localhost/api/{}", id.to_string());
    return Request::get(&url).send().await.unwrap().json().await.unwrap();
}

async fn get_open_rooms() {
    let url = "http://localhost/api/rooms";
}


#[function_component(Home)]
fn home() -> Html {
    let MessageBlocks: Vec<_> = (0..3).map(|_| html_nested!{<MessageBlock/>}).collect();
    let navigator = use_navigator().unwrap();
    let to_test = Callback::from(move |_| navigator.push(&Route::Room));
    html! {
        <>
            <div class="container title">
                <img src="static/logo.png" alt="Harmony" class="centre"/> 
                <div class="container">
                    <button onclick={to_test}>{ "Create Room" }</button>
                </div>
            </div>
            <div class="server-list">
                <h1>{ " Server List "}</h1>
                <h2>{ "| under construction |" }</h2>
                { for MessageBlocks }
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
#[function_component(Room)]
fn room() -> Html {

    let id = Uuid::new_v4();
    open_lobby(id);
    
    html! {
        <>
            <h1>{ format!("test on -  {}",id) }</h1>
            <WebSocketClient url={format!("ws://10.57.17.0/api/{}",id)}/>
        </>
    }
}
 

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::Room => html! {<Room/>},
        Route::NotFound => html! {<h1>{ "404 lol" }</h1>},
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}