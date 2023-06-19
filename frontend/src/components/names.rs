use yew::prelude::*;
use rand::prelude::*;
use reqwasm::http::Request;
use std::collections::HashMap;
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
            let nick_name_map: HashMap<String, String> = Request::get("http://192.168.0.147/api/nicks")
                .send()
                .await
                .unwrap()
                .json()
                .await
                .unwrap();

            *names_clone.borrow_mut() = nick_name_map;
        });

        NickName {
            names,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // possibly move future here, with refresh button ?
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let name_map = &self.names.borrow();

        let current_users_component: Vec<_> = name_map.iter().map(|(id, nick)| html!{
            <>
                <p>{id}</p>
                <p>{nick}</p>
            </>
            }).collect();

        
        html! {
            {for current_users_component}
        }
    }
}