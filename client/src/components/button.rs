use crate::styler;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
  pub children: Children,
  pub on_click: Callback<MouseEvent>,
  pub toggle: bool,
  pub size: String,
}

struct State {
  toggle: bool,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
  let Props {
    children,
    on_click,
    toggle,
    size,
  } = props.clone();
  let state = use_state(|| State { toggle: false });

  let handle_onclick = {
    Callback::from(move |event: MouseEvent| {
      on_click.emit(event);
      if toggle {
        let state = state.clone();
        state.set(State {
          toggle: !state.toggle,
        });
      }
    })
  };

  let css = &format!(
    "
    font-size: {};
    padding: 0.75em 1.5em;
    color: #FFFFFF;
    background-color: {};
    cursor: pointer;
    border: none;
    border-radius: 0.375em;
    box-shadow: 0px 3px 3px -1px rgba(0, 0, 0, 0.2), 0px 1px 5px 0px rgba(0, 0, 0, 0.1);
    transition: background-color 0.2s, box-shadow 0.2s;
    &:hover,&:focus-visible {{
      background-color: {};
      box-shadow: 0px 3px 5px -1px rgba(0, 0, 0, 0.4), 0px 1px 7px 0px rgba(0, 0, 0, 0.2);
    }}
    &:active {{
      background-color: {};
    }}
    ",
    size,
    styler::PALETTE["primary"],
    styler::PALETTE["primary-hover"],
    styler::PALETTE["primary-active"]
  );
  html! {
    <button
      class={styler::build(css)}
      onclick={handle_onclick}
    >
      { for children.iter() }
    </button>
  }
}
