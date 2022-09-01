use crate::views::home::Home;
use crate::views::learn::Learn;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
  #[at("/")]
  Home,
  #[at("/learn")]
  Learn,
}

pub fn switch(routes: &Route) -> Html {
  match routes {
    Route::Home => html! { <Home /> },
    Route::Learn => html! { <Learn /> },
  }
}
