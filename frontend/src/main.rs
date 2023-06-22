use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use uuid::Uuid;

mod components;

use components::room::{
    RoomsListBlock,
};

use components::names::{
    NickName,
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

// whisper functionality is going to need to be completely reworked with custom names
//  **
//  The following must be set to Local IP of Host Computer
//    -  frontend/src/main.rs HOST_IP 
//    -  frontend/trunk.toml [[proxy]]
//    -  frontend/src/components/room.rs get_open_rooms()
//    -  frontend/src/components/names.rs let nick_name_map 
//
//  **

//https://lib.rs/crates/yew-api-hook

static HOST_IP: &str = "10.57.17.0";

#[function_component(Home)]
fn home() -> Html {
    //let MessageBlocks: Vec<_> = (0..3).map(|_| html_nested!{<MessageBlock/>}).collect();
    let navigator = use_navigator().unwrap();
    let to_room = Callback::from(move |_| navigator.push(&Route::Room{ id: Uuid::new_v4().to_string()}));

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
                                <button class="btn btn-outline-light" onclick={to_room}>
                                <p class="display-6">{"Create Room" }</p>
                                </button>
                                 <div class="text-center pt-3">
                                    <p class="fw-bold fst-italic text-white">
                                        {"
                                        Welcome to Harmony! This is a simple chat app designed to be used by computers sharing the same network.
                                        It was built by two highschool students over the course of a few months, as a first go at using the rust language
                                        for web app development. It's built using the Yew and Actix Web frameworks. We hope you enjoy! Much love,
                                        "}
                                    </p>
                                </div>
                                <div class="container">
                                    <div class="row">
                                        <div class="col">
                                            <h1 class="display-6 float-end"> { "Wyatt" } </h1>
                                            <img class ="img-responsive float-start" width = "50px" height = "50px" src="static/Moogle.jpg" alt="Moogle"/>
                                        </div>
                                        <div class="col">
                                            <h1 class="display-6 float-end"> { "Tom" } </h1>
                                            <img class ="img-responsive float-start" width ="50px" height = "50px" src="static/Mike.jpg" alt="Prison Mike"/>
                                        </div>
                                        <h1 class = "display-6 text-center">{"<3"} </h1>
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="col-8">
                        <div class="border border-5 border-white rounded p-3">
                            <div class="server-list">
                                <h1 class="display-3">{ "Server List"}</h1>
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
    let navigator = use_navigator().unwrap();
    // using .replace() instead of .push() because clicking home twice if user is already on home page will panic!
    let to_home = Callback::from(move |_| navigator.push(&Route::Home));
    
    
    html! {
        <>  
            <div class="containerours title">
                <button onclick={to_home} type="submit" style={"background-color: transparent;"}>
                    <img src="../static/logo.png" alt="Harmony" class="centre"/>
                </button>
            </div>
            <div class="container-xxl">
                <div class="row">
                    <div class="col-4">
                        <div class="border border-5 border-white rounded p-3 bg-light">
                            <h2>{"Active Harmony Users"}</h2>
                            <NickName/>
                        </div>
                    </div>
                    <div class="col-8">
                        <div class="border border-5 border-white rounded p-3 bg-light">
                            <h2 class="display-5">{ format!("Room") }</h2>
                            <h2 class="display-6">{ format!("{}", url_id) }</h2>
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