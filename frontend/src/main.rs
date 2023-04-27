use yew::prelude::*;

#[function_component(App)]
fn App() -> Html {
    html! {
        <h1>{ Hello from yew }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
