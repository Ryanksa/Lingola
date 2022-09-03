use crate::router::Route;
use crate::styler;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{button::Button, text::Text};

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
      padding: 3rem;
      display: flex;
      align-items: center;
      justify-content: center;
      cursor: pointer;
      box-sizing: border-box;
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
          <Text size="3.5rem" weight={500} color={styler::PALETTE["light-base"]}>
            { "Lingola" }
          </Text>
        </div>
        <div class="panel left">
          <Text size="1.5rem" weight={300} color={styler::PALETTE["light-base"]}>
            { "An app to learn new words and build your vocabulary pool." }
          </Text>
        </div>
        <div class="panel right">
          <Text size="1.5rem" weight={300} color={styler::PALETTE["light-base"]}>
            { "You'll get presented with a word and two definitions to pick from." }
          </Text>
        </div>
        <div class="button-container">
          <Button on_click={handle_get_started} toggle=false size="1.25em">{ "Get Started!" }</Button>
        </div>
      </div>
    }
}
