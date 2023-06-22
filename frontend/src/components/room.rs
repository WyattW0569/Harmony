use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use crate::Route;
use web_sys::console;

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
        let style = "padding:20px; background-color: #D3D3D3;";
        let link = ctx.link();
        let navigator = link.navigator().unwrap();
        let url_id = self.id.clone();
        let onclick = Callback::from(move |_| navigator.push(&Route::Room { id: url_id.to_owned() }));
        let room_char = self.id.clone().chars().filter(|x| x.is_alphabetic()).next().unwrap().to_ascii_uppercase();

        html! {
            <button {onclick} class="pt-3" type="submit" style={"background-color: transparent; border-color: transparent;"}>
                <div class="container border border-5 border-white rounded gy-2 row" style={style}>
                    <div style={"display: flex; align-items: center;"}> 
                        <img src="../static/room_logo.png" class="float-end" width="50" height="50"/>
                        <h2 style={"white-space: nowrap; padding-right: 55px; padding-left: 45px;"}>{ format!("Chat Room {}", room_char) }</h2>
                        <div class="h-75 border border-4" style={"display: flex; align-items: center; padding-left: 10px; padding-right: 20px; margin-right: 30px;"}>
                            <img src="../static/user_icon.png" class="float-end pb-1" width="37" height="37"/>
                            <p class="float-start fs-3 pt-2" style={"white-space: nowrap;"}> { format!("{} / 16", self.pop.clone()) } </p>
                        </div>
                    </div>
                </div>
            </button>
        }
    }
}

pub struct RoomsListBlock {
    rooms: Rc<RefCell<Vec<String>>>,
    population: Rc<RefCell<Vec<i32>>>,
    listener: LocationHandle,
}

impl Component for RoomsListBlock {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let rooms: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(Vec::new()));
        let population: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(Vec::new()));
        let listener = link.add_location_listener(link.callback(
            |_| console::log_1(&"RoomList Component Route change listener".into())
        )) 
        .unwrap();

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
            listener,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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