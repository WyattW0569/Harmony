use yew::prelude::*;
use web_sys::{ErrorEvent, MessageEvent, WebSocket};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, InputEvent};

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
        match msg{
            Msg::Send => {
                if let Some(ref mut websocket) = self.socket {
                    websocket.send_with_str(&self.input).unwrap();
                    self.input = String::new();
                }
            }
            Msg::MessageRecieved(incoming_msg) => self.messages.push(incoming_msg),
            Msg::Input(outgoing_msg) => self.input = outgoing_msg,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html{
        let link = ctx.link();
        let on_input = link.callback(|e: Event| Msg::Input(e.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok()).unwrap().value()));
        let on_click = {link.callback(|_| Msg::Send)};
        /*let on_change = Callback::from(move |e: Event| {
            let target: EventTarget = e
            .target()
            .expect("Event should have a target when dispatched");
        input_value_handle.set(target.unchecked_into::<HtmlInputElement>().value());
        let input_value = input_value_handle.to_string();
        });*/

        html!{
            <>
                <div>
                    { for self.messages.iter().map(|message| html_nested! {<p>{ message }</p>}) }
                </div>
                <div>
                    <h1> { "Input!!!!" } </h1>
                    <input onchange={ on_input }
                    type="text"
                    value={self.input.clone()}
                    />
                    <button onclick={ on_click }> {"Send"} </button>
                </div>
            </>
        }
    }
}