use yew::prelude::*;
pub enum Msg {
    Update(String),
}

pub struct InputBlock {
    content: String,
}

impl Component for InputBlock {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            content: "".to_string()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update(content) => self.content = content.to_uppercase(),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <textarea
                oninput={link.callback(|event: InputData| Msg::Update(event.value))}
                value={self.content.clone()}>
            </textarea>
        }
    }
}
//https://dev.to/fllstck/basic-interactions-with-yew-3pa3 for tom <3