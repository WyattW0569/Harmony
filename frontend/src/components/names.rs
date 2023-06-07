use yew::prelude::*;
use rand::prelude::*;
use reqwasm::http::Request;
use std::collections::{HashMap};
use std::rc::Rc;
use std::cell::RefCell;


#[derive(Clone)]
pub struct NickName {
    names: Rc<RefCell<HashMap<String, String>>>,
}

impl Component for NickName {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let guest: String = String::from(format!("Guest{}",rand::thread_rng().gen_range(0..100)));

        let names: Rc<RefCell<HashMap<String, String>>> = Rc::new(RefCell::new(HashMap::new()));
        let names_clone = names.clone();

        link.send_future(async move {
            let nick_name_map: HashMap<String, String> = Request::get("http://10.57.17.0/api/nicks").send().await.unwrap().json().await.unwrap();

            *names_clone.borrow_mut() = nick_name_map;
        });

        NickName {
            names,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let name = &self.names.borrow();
        let names: Vec<_> = name.values().collect();
        
        html! {
            <h1> { format!("{:?}", names)} </h1>
        }
    }
}