use yew::prelude::*;
use uuid::Uuid;

console::log_1(&"Hello using web-sys".into());


async fn get_open_rooms() -> String {
    let url = "http://localhost/api/rooms";
    let response = Request::get(url).send().await.unwrap();
    response
}

pub struct RoomBlock {
    id: String,
    pop: i32,
}

pub struct Props {
    key: String,
}

impl Component for RoomBlock {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let server_list = get_open_rooms();
        let props = ctx.Props();
        Self {
            id: "TESTING".to_string(),
            pop: 69.,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let style = format!("background-color: {}", self.colour);
        let DEMO_ID = Uuid::new_v4().to_string();
        html! {
            <div class="container" style={style}>
                <h1> { DEMO_ID } </h1>
                <button> {"Join!"} </button>
            </div>
        }
    }
}