use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_stdweb::format::Json;
use reqwasm::http::Request;
use uuid::Uuid;
use std::collections::{HashMap, HashSet};

mod components;

use components::room::{
    RoomsListBlock,
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

//  **
//  The following must be set to Local IP of Host Computer
//    -  frontend/src/main.rs HOST_IP 
//    -  frontend/trunk.toml [[proxy]]
//    -  frontend/src/components/room.rs get_open_rooms() 
//
//  **

static HOST_IP: &str = "10.57.16.255";

#[function_component(Home)]
fn home() -> Html {
    //let MessageBlocks: Vec<_> = (0..3).map(|_| html_nested!{<MessageBlock/>}).collect();
    let navigator = use_navigator().unwrap();
    let to_test = Callback::from(move |_| navigator.push(&Route::Room{ id: Uuid::new_v4().to_string()}));

    html! {
        <>
            <div class="containerours title">
                <img src="static/logo.png" alt="Harmony" class="centre"/> 
            </div>
            <div class="container-xxl">
                <div class="row">
                    <div class="col-4">
                        <div class="border border-5 border-white rounded p-3">
                            <div class="d-grid">
                                <button class="btn btn-dark" onclick={to_test}>{ "Create Room" }</button>
                                <img class ="img-responsive" width ="100%" src="static/funny-cat-3.jpg" alt="funnycat"/>
                            </div>
                        </div>
                    </div>
                    <div class="col-8">
                        <div class="border border-5 border-white rounded p-3">
                            <div class="server-list">
                                <h1>{ " Server List "}</h1>
                                <h2>{ "| under construction |" }</h2>
                                <RoomsListBlock/>
                            </div>
                        </div>
                    </div>
                </div>
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
    
    html! {
        <>  
            <div class="containerours title">
                <img src="../static/logo.png" alt="Harmony" class="centre"/>
            </div>
            <div class="container-xxl">
                <div class="row">
                    <div class="col-4">
                        <div class="border border-5 border-white rounded p-3">
                            { "test" }
                        </div>
                    </div>
                    <div class="col-8">
                        <div class="border border-5 border-white rounded p-3">
                            <h1 class="display-1">{ format!("Connected to -  {}",url_id) }</h1>
                            <div>
                                <WebSocketClient url={format!("ws://{}/api/{}",HOST_IP.clone(), url_id)}/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    }
}
 

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home/>},
        Route::Room { id } => html! {<Room url_id={id}/>},
        Route::NotFound => html! {<h1>{ "404 lol" }</h1>},
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}