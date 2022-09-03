use crate::services::http_get;
use crate::styler;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
struct WordAndDef {
    word: String,
    definition: String,
}

#[function_component(Learn)]
pub fn learn() -> Html {
    let words: UseStateHandle<Vec<WordAndDef>> = use_state(|| vec![]);
    let error: UseStateHandle<&str> = use_state(|| "");

    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match http_get("/api/word/random").await {
                    Ok(fetched_words) => words.set(fetched_words),
                    Err(_) => error.set("Out of words... Come back in an hour"),
                }
            });
            return || {};
        },
        (),
    );

    html! {
      <div></div>
    }
}
