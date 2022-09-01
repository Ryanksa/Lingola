use crate::router::Route;
use crate::styler;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{button::Button, typography::Typography};

#[function_component(Home)]
pub fn home() -> Html {
  let navigator = use_history().unwrap();

  let handle_get_started = {
    Callback::from(move |_event: MouseEvent| {
      navigator.push(Route::Learn);
    })
  };

  let css = &format!(
    "
    width: 100%;
    height: calc(100vh - 4em);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: space-between;
    padding: 2em 0;
    position: relative;

    &::before {{
      content: '';
      position: absolute;
      left: calc(50% - 2px);
      top: 0;
      width: 4px;
      height: 100%;
      background-color: {};
    }}

    .word-container {{
      background-color: {};
      border-radius: 0.5em;
      padding: 1em;
      position: relative;
      z-index: 1;
    }}

    .panel {{
      position: absolute;
      top: 0;
      width: 50%;
      height: 100%;
      display: grid;
      place-items: center;
      cursor: pointer;
    }}

    .panel:hover {{
      background-color: {};
      opacity: 0.5;
    }}

    .panel.left {{
      left: 0;
    }}

    .panel.right {{
      left: 50%;
    }}

    .button-container {{
      position: relative;
      z-index: 1;
    }}
    ",
    styler::PALETTE["dark-base"],
    styler::PALETTE["dark-base"],
    styler::PALETTE["primary-transparent"]
  );
  html! {
    <div class={styler::build(css)}>
      <div class="word-container">
        <Typography size="3.5rem" weight={500} color={styler::PALETTE["light-base"]}>
          { "Lingola" }
        </Typography>
      </div>
      <div class="panel left">
        <Typography size="1rem" weight={300} color="">
          { "An app to learn new words and build your vocabulary pool." }
        </Typography>
      </div>
      <div class="panel right">
        <Typography size="1rem" weight={300} color="">
          { "You'll get presented with a word and two definitions to pick from." }
        </Typography>
      </div>
      <div class="button-container">
        <Button on_click={handle_get_started} toggle=false size="1.25em">{ "Get Started!" }</Button>
      </div>
    </div>
  }
}
