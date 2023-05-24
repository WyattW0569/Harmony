use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_stdweb::format::Json;
use reqwasm::http::Request;
use uuid::Uuid;
use std::collections::{HashMap, HashSet};

mod components;

use components::room::{
    RoomBlock,
};

use components::websocket::{
    WebSocketClient,
};

#[derive(Debug, Clone, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/room/:id")]
    Room{ id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

// Ip of Host Computer
static HOST_IP: &str = "10.57.17.0";

async fn connect_to_lobby(id: Uuid) {
    let url = format!("http://localhost/api/{}", id.to_string());
    return Request::get(&url).send().await.unwrap().json().await.unwrap();
}

#[function_component(Home)]
fn home() -> Html {
    //let MessageBlocks: Vec<_> = (0..3).map(|_| html_nested!{<MessageBlock/>}).collect();
    let navigator = use_navigator().unwrap();
    let testing_url_string = String::from("Testing");
    let to_test = Callback::from(move |_| navigator.push(&Route::Room{ id: "test".to_string()}));

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
                <RoomBlock/>
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

#[derive(Properties, PartialEq)]
pub struct Props {
    pub url_id: String,
}


#[function_component(Room)]
fn room(props: &Props) -> Html {
    let url_id = props.url_id.clone();

    let id = Uuid::new_v4();
    connect_to_lobby(id);
    
    html! {
        <>
            <h1>{ format!("test on -  {}",id) }</h1>
            <WebSocketClient url={format!("ws://{}/api/{}",HOST_IP.clone(), id)}/>
        </>
    }
}
 

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::Room { id } => html! {<Room url_id={"test".to_string()}/>},
        Route::NotFound => html! {<h1>{ "404 lol" }</h1>},
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}