use yew::prelude::*;
use web_sys::{MessageEvent, WebSocket};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use std::collections::HashMap;
use web_sys::{HtmlInputElement, InputEvent, KeyboardEvent};
use rand::seq::SliceRandom;


pub struct WebSocketClient {
    messages: Vec<String>,
    socket: Option<WebSocket>,
    input: String,
    on_message_callback: Closure<dyn FnMut(MessageEvent)>,
}

pub enum Msg{
    Send,
    MessageRecieved(String),
    Input(String),
    NoOpp,
}


#[derive(Properties, Clone, PartialEq)]
pub struct Props{
    pub url: String,
} 

impl Component for WebSocketClient {
    type Properties = Props;
    type Message = Msg;

    fn create(ctx: &Context<Self>) -> Self{
        let props = ctx.props().clone();
        let websocket = WebSocket::new(&props.url).unwrap();

        let on_message_callback = {
            let link = ctx.link().clone();
            Closure::wrap(Box::new(move | event: MessageEvent| {
                let message = event.data().as_string().unwrap();
                link.send_message(Msg::MessageRecieved(message));
            }) as Box<dyn FnMut(MessageEvent)>)
        };
        websocket.set_onmessage(Some(on_message_callback.as_ref().unchecked_ref()));

        WebSocketClient {
            messages: vec![],
            socket: Some(websocket),
            on_message_callback,
            input: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Send => {
                if let Some(ref mut websocket) = self.socket {
                    websocket.send_with_str(format!("{}",&self.input).as_str()).unwrap();
                    self.input = String::new();
                }
            }
            Msg::MessageRecieved(incoming_msg) => self.messages.push(incoming_msg),
            Msg::Input(outgoing_msg) => self.input = outgoing_msg,
            Msg::NoOpp => (),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html{
        let link = ctx.link();

        let input_component  = {
            let on_input = link.callback(|e: InputEvent| {
                let input_value = e.target().unwrap().dyn_into::<HtmlInputElement>().unwrap().value();

                Msg::Input(input_value)
            });

            let on_keydown = link.callback(|e: KeyboardEvent| {
                if e.key_code() == 13{
                   return Msg::Send;
                }
                return Msg::NoOpp;
            });

            html! {
                <input class = "container"
                    oninput={ on_input }
                    onkeydown={ on_keydown }
                    type="text"
                    value={self.input.clone()}
                />
            }
        };

        let mut user_colors: HashMap<String, String> = HashMap::from([("Guest".to_string(), "green".to_string())]);
        let colors = vec!["red", "blue", "yellow", "grey", "violet", "green"];

        self.messages.iter().skip(1).map(|message| {
            user_colors.insert(
                message.split("|").collect::<Vec<_>>().get(0).unwrap().to_string(),
                colors.choose(&mut rand::thread_rng()).unwrap().to_string(),
        )});
        
        html!{
            <>
                <div style="height: 600px;" class="overflow-hidden">
                    <div style="height: 550px;" class="scrollable overflow-auto" id="text-box">
                        { for self.messages.iter().skip(1).map(|message| html_nested! {
                            <>
                                <p style={format!("color: {}", user_colors.get(message.split("|").next().map(str::trim).unwrap_or("")).unwrap() )}> { message.split("|").collect::<Vec<_>>().get(0) }</p>
                                <p class="container-sm bg-light rounded shadow-sm p-3 mb-1">{ message }</p>
                            </>
                        })}
                    </div>
                    <div class="align-items-end">
                        {input_component}
                    </div>
                </div>
            </>
        }
    }
}