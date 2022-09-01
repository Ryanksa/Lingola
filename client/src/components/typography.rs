use crate::styler;
use yew::prelude::*;

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
  pub children: Children,
  pub size: String,
  pub weight: u16,
  pub color: String,
}

#[function_component(Typography)]
pub fn typography(props: &Props) -> Html {
  let Props {
    children,
    size,
    weight,
    color,
  } = props.clone();

  let css = &format!(
    "
    font-size: {};
    font-weight: {};
    color: {};
    ",
    size, weight, color
  );
  html! {
    <div class={styler::build(css)}>
      { for children.iter() }
    </div>
  }
}
