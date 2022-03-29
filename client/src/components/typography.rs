use crate::styler;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
  pub children: Children,
  pub size: String,
  pub weight: u16,
}

#[function_component(Typography)]
pub fn typography(props: &Props) -> Html {
  let Props {
    children,
    size,
    weight,
  } = props.clone();

  let css = &format!(
    "
    font-size: {};
    font-weight: {};
    ",
    size, weight
  );
  html! {
    <div class={styler::build(css)}>
      { for children.iter() }
    </div>
  }
}
