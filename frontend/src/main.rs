use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
enum Route {
    #[at("/")]
    Home,
    #[at("/test")]
    Test,
    #[not_found]
    #[at("/404")]
    NotFound,
}

enum Msg {
    ChangeColour,
}

struct MessageBlock {
    Colour: String,
}

impl Component for MessageBlock {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            Colour: "#9999ff".to_string()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ChangeColour => {
                self.Colour = "#00FF00".to_string();
                true // re-render 
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let style = format!("background-color: {}", self.Colour);
        html! {
            <div class="container" style={style}>
                <h1> { "This is a Message Block" } </h1>
                <button onclick = {link.callback(|_| Msg::ChangeColour)}> {"click me!"} </button>
            </div>
        }
    }
}

#[function_component(Home)]
fn home() -> Html {
    let MessageBlocks: Vec<_> = (0..3).map(|_| html_nested!{<MessageBlock/>}).collect();
    let navigator = use_navigator().unwrap();
    let to_test = Callback::from(move |_| navigator.push(&Route::Test));
    html! {
        <>
            <div class="container">
                <h1>{ "test!" }</h1>
                <button onclick={to_test}>{ "to test!!" }</button>
            </div>
            <div>
                <h1>{ "hey hehe" }</h1>
                <h2>{ "hey hehe but smaller" }</h2>
                { for MessageBlocks }
            </div>
        </>
    }
}

#[function_component(App)]
fn app() -> Html {
    
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {<Home />},
        Route::Test => html! { <h1>{ "IT WORKED" }</h1>},
        Route::NotFound => html! {<h1>{ "404 lol" }</h1>},
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}