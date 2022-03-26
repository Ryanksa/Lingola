use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
  pub children: Children,
}

#[function_component(Typography)]
pub fn typography(props: &Props) -> Html {
  let Props { children } = props.clone();
  html! {
    <div class="component-typography-root">
      { for children.iter() }
    </div>
  }
}
