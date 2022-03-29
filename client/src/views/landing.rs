use crate::styler;
use yew::prelude::*;

use crate::components::{button::Button, typography::Typography};

#[function_component(Landing)]
pub fn landing() -> Html {
  let handle_click = { Callback::from(move |_event: MouseEvent| {}) };

  let css = "
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  ";
  html! {
    <div class={styler::build(css)}>
      <Typography size="2.5rem" weight={500}>{ "Typography" }</Typography>
      <Button on_click={handle_click} toggle=false size="1.25em">{ "Button" }</Button>
    </div>
  }
}
