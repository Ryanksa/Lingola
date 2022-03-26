use yew::prelude::*;

use crate::components::{button::Button, typography::Typography};

#[function_component(Landing)]
pub fn landing() -> Html {
  let handle_click = { Callback::from(move |event: MouseEvent| {}) };

  html! {
    <div class="view-landing-root">
      <Typography>{ "Typography" }</Typography>
      <Button on_click={handle_click} toggle=false>{ "Button" }</Button>
    </div>
  }
}
