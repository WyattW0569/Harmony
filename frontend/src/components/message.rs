use yew::prelude::*;
use uuid::Uuid;

pub enum Msg {
    ChangeColour,
}

pub struct MessageBlock {
    colour: String,
}

impl Component for MessageBlock {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            colour: "#46004f".to_string()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeColour => {
                self.colour = "#85006c".to_string();
                true // re-render 
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let style = format!("background-color: {}", self.colour);
        let DEMO_ID = Uuid::new_v4().to_string();
        html! {
            <div class="container" style={style}>
                <h1> { DEMO_ID } </h1>
                <button onclick = {link.callback(|_| Msg::ChangeColour)}> {"Join!"} </button>
            </div>
        }
    }
}