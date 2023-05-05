use yew::prelude::*;

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
            colour: "#9999ff".to_string()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeColour => {
                self.colour = "#00FF00".to_string();
                true // re-render 
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let style = format!("background-color: {}", self.colour);
        html! {
            <div class="container" style={style}>
                <h1> { "This is a Message Block" } </h1>
                <button onclick = {link.callback(|_| Msg::ChangeColour)}> {"click me!"} </button>
            </div>
        }
    }
}