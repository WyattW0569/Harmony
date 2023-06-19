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
//
//  **

//https://lib.rs/crates/yew-api-hook

static HOST_IP: &str = "192.168.0.147";

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
                                <button class="btn btn-dark" onclick={to_room}>{ "Create Room" }</button>
                                 <div class="text-center pt-3">
                                    <p class="fw-bold fst-italic text-white">
                                        { "Lorem ipsum dolor sit amet, 
                                        consectetur adipiscing elit. Nunc iaculis 
                                        pharetra tempus. Sed ante dui, lobortis in iaculis id, 
                                        consectetur efficitur enim. Quisque vehicula vulputate viverra. 
                                        In congue et eros at semper. Suspendisse tincidunt ac purus id volutpat. 
                                        Quisque a massa at enim molestie commodo sit amet sed metus. 
                                        Proin semper eros eu arcu interdum convallis. Nunc nunc lectus, posuere 
                                        "}
                                    </p>
                                </div>
                                <div class="container">
                                    <div class="row">
                                        <div class="col">
                                            <img class ="img-responsive" width = "50px" height = "50px" src="static/funny-cat-3.jpg" alt="funnycat"/>
                                        </div>
                                        <div class="col">
                                            <img class ="img-responsive" width ="50px" height = "50px" src="static/funny-cat-3.jpg" alt="funnycat"/>
                                        </div>
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