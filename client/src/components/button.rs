use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
  pub children: Children,
  pub on_click: Callback<MouseEvent>,
  pub toggle: bool,
}

struct State {
  toggle: bool,
}

#[function_component(Button)]
pub fn button(props: &Props) -> Html {
  let Props {
    on_click,
    toggle,
    children,
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

  html! {
    <button
      class="component-button-root"
      onclick={handle_onclick}
    >
      { for children.iter() }
    </button>
  }
}
