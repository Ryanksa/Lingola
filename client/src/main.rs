use yew::prelude::*;

pub mod components;

mod views;
use views::landing::Landing;

#[function_component(App)]
fn app() -> Html {
    html! {
        <Landing></Landing>
    }
}

fn main() {
    yew::start_app::<App>();
}
