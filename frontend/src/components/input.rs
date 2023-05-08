use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;

struct Msg{
   Message: String,
}

#[function_component(TextInput)]
pub fn text_input() -> Html {
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let on_change = Callback::from(move |e: Event| {
        let target: EventTarget = e
        .target()
        .expect("Event should have a target when dispatched");
    input_value_handle.set(target.unchecked_into::<HtmlInputElement>().value());
    let input_value = input_value_handle.to_string();
    });


    html! {
        <>
            <h1> { "Input!!!!" } </h1>
            <input onchange={on_change}
            type="text"
            value={input_value.clone()}
            />
            <h1> { input_value } </h1>
        </>
    }
}