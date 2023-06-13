use yew::prelude::*;
use yew_router::prelude::*;
use uuid::Uuid;
use reqwasm::http::Request;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use crate::Route;



async fn get_open_rooms() -> HashMap<String, HashSet<String>> {
    let url = "http://10.57.17.0/api/rooms";
    let room_map = Request::get(url).send().await.unwrap().json().await.unwrap();
    room_map
}

pub struct RoomBlock {
    id: String,
    pop: i32,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    id: String,
    pop: i32,
}

impl Component for RoomBlock {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        Self {
            id: props.id,
            pop: props.pop,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = "padding:20px;";
        let link = ctx.link();
        let navigator = link.navigator().unwrap();
        let url_id = self.id.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Room { id: url_id.to_owned() }));

        html! {
            <div class="container border border-5 border-white rounded-pill gy-3" style={style}>
                <h1 class="display-6"> { self.id.clone() } </h1>
                <h2> { self.pop.clone() } </h2>
                <button class="btn btn-dark" {onclick}> {"Join!"} </button>
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
        let link = ctx.link().clone();
        let rooms: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
        let population: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));

        let rooms_clone = rooms.clone();
        let population_clone = population.clone();
        link.send_future(async move {
            let open_rooms = get_open_rooms().await;

            let rooms = open_rooms.keys().cloned().collect();
            *rooms_clone.borrow_mut() = rooms;

            let population = open_rooms
                .values()
                .map(|set| set.len() as i32)
                .collect();

            *population_clone.borrow_mut() = population;
        });

        RoomsListBlock {
            rooms,
            population,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let rooms_borrowed = self.rooms.borrow();
        let population_borrowed = self.population.borrow();

        let combined = rooms_borrowed.iter().zip(population_borrowed.iter());

        let room_and_population_blocks: Vec<Html> = combined.map(|(id, pop)| {
            html! {
                <RoomBlock id={id.clone()} pop={pop.clone()}/>
            }
        }).collect();

        html! {
            { for room_and_population_blocks }
        }   
    }
}