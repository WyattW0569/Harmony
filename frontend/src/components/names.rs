use yew::prelude::*;
use yew_router::prelude::*;
use reqwasm::http::Request;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use web_sys::console;


#[derive()]
pub struct NickName {
    names: Rc<RefCell<HashMap<String, String>>>,
    listener: LocationHandle,
}

impl Component for NickName {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();

        let names: Rc<RefCell<HashMap<String, String>>> = Rc::new(RefCell::new(HashMap::new()));

        let names_clone = names.clone();

        let listener = link.add_location_listener(link.callback(
            |_| console::log_1(&"NickName Component Route change listener".into())
        ))
        .unwrap();

        link.send_future(async move {
            let nick_name_map: HashMap<String, String> = Request::get("http://10.57.17.0/api/nicks")
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
            listener,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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