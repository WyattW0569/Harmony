use yew::prelude::*;
use uuid::Uuid;
use reqwasm::http::Request;
use std::rc::Rc;
use std::cell::RefCell;


async fn get_open_rooms() -> String {
    let url = "http://localhost/api/rooms";
    let response = Request::get(url).send().await.unwrap();
    //response
    "test".to_string()
}

pub struct RoomBlock {
    id: String,
    pop: i32,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    key: String,
}

impl Component for RoomBlock {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        //let server_list = get_open_rooms().await;
        let props = ctx.props();
        Self {
            id: "test".to_string(),
            pop: 69,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let DEMO_ID = Uuid::new_v4().to_string();
        html! {
            <div class="container">
                <h1> { DEMO_ID } </h1>
                <button> {"Join!"} </button>
            </div>
        }
    }
}

pub struct RoomsListBlock {
    rooms: Rc<RefCell<Vec<String>>>,
    population: Rc<RefCell<Vec<i32>>>,
}

impl Component for RoomsListBlock {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            rooms: Rc::new(RefCell::new(Vec::new())),
            population: Rc::new(RefCell::new(Vec::new())),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <h1>{"yo"}</h1>
            </>
        }   
    }
}