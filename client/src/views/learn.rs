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
    let client = reqwest::Client::new();
    let words: UseStateHandle<Vec<WordAndDef>> = use_state(|| vec![]);

    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_words: Vec<WordAndDef> = client
                    .get("http://localhost:5000/word/random")
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<WordAndDef>>()
                    .await
                    .unwrap();
                words.set(fetched_words);
            });
            return || {};
        },
        (),
    );

    html! {
      <div></div>
    }
}
