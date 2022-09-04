use crate::components::{button::Button, text::Text};
use crate::services::http_get;
use crate::styler;
use reqwest::Error;
use serde::Deserialize;
use yew::prelude::*;

#[derive(PartialEq)]
enum State {
    CHOOSING,
    CHOSEN,
}

#[derive(Clone, PartialEq, Deserialize)]
struct WordAndDef {
    word: String,
    definition: String,
}

#[function_component(Learn)]
pub fn learn() -> Html {
    let words: UseStateHandle<Vec<WordAndDef>> = use_state(|| vec![]);
    let question = use_state(|| String::new());
    let answer = use_state(|| String::new());
    let state = use_state(|| State::CHOOSING);
    let error = use_state(|| "");
    let empty = WordAndDef {
        word: String::new(),
        definition: String::new(),
    };
    let counter = use_state(|| 0);

    {
        let words = words.clone();
        let question = question.clone();
        let answer = answer.clone();
        let error = error.clone();
        use_effect_with_deps(
            |_| {
                wasm_bindgen_futures::spawn_local(async move {
                    let result: Result<Vec<WordAndDef>, Error> = http_get("/api/word/random").await;
                    match result {
                        Ok(fetched_words) => {
                            if fetched_words.len() >= 2 {
                                if fetched_words[0].word.len() <= fetched_words[1].word.len() {
                                    question.set(fetched_words[0].word.clone());
                                    answer.set(fetched_words[0].definition.clone());
                                } else {
                                    question.set(fetched_words[1].word.clone());
                                    answer.set(fetched_words[1].definition.clone());
                                }
                            }
                            words.set(fetched_words);
                        }
                        Err(_) => error.set("Out of words... Come back later"),
                    }
                });
                return || {};
            },
            (*counter,),
        );
    }

    let get_result_colour = |definition: &str| -> &'static str {
        if definition == *answer {
            return styler::PALETTE["success"];
        } else {
            return styler::PALETTE["error"];
        }
    };

    let handle_choose = {
        let state = state.clone();
        Callback::from(move |_event: MouseEvent| {
            state.set(State::CHOSEN);
        })
    };

    let handle_next = {
        let state = state.clone();
        let counter = counter.clone();
        Callback::from(move |_event: MouseEvent| {
            counter.set(*counter + 1);
            state.set(State::CHOOSING);
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
            flex-direction: column;
            gap: 2rem;
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

        .error-container {{
            background-color: {};
            border-radius: 0.5em;
            padding: 1em;
            position: relative;
            z-index: 1;
        }}
    ",
        styler::PALETTE["dark-base"],
        styler::PALETTE["dark-base"],
        styler::PALETTE["primary-transparent"],
        styler::PALETTE["light-base"]
    );

    if (*error).len() > 0 {
        return html! {
            <div class={styler::build(css)}>
                <div class="word-container">
                    <Text size="3.5rem" weight={500} color={styler::PALETTE["light-base"]}>
                        { "Lingola" }
                    </Text>
                </div>
                <div class="panel left"></div>
                <div class="panel right"></div>
                <div class="error-container">
                <Text size="2rem" weight={300} color={styler::PALETTE["error"]}>
                    {format!("{}", (*error))}
                </Text>
                </div>
            </div>
        };
    }

    if *state == State::CHOOSING {
        return html! {
          <div class={styler::build(css)}>
            <div class="word-container">
                <Text size="3.5rem" weight={500} color={styler::PALETTE["light-base"]}>
                    { format!("{}", *question) }
                </Text>
            </div>
            <div class="panel left" onclick={handle_choose.clone()}>
                <Text size="1.5rem" weight={300} color={styler::PALETTE["light-base"]}>
                { format!("{}", (*words).get(0).unwrap_or(&empty).definition) }
                </Text>
            </div>
            <div class="panel right" onclick={handle_choose.clone()}>
                <Text size="1.5rem" weight={300} color={styler::PALETTE["light-base"]}>
                { format!("{}", (*words).get(1).unwrap_or(&empty).definition) }
                </Text>
            </div>
          </div>
        };
    }

    if *state == State::CHOSEN {
        return html! {
          <div class={styler::build(css)}>
            <div class="word-container">
                <Text size="3.5rem" weight={500} color={styler::PALETTE["light-base"]}>
                    { format!("{}", *question) }
                </Text>
            </div>
            <div class="panel left">
                <Text size="1.5rem" weight={300} color={get_result_colour(&(*words).get(0).unwrap_or(&empty).definition)}>
                { format!("{}", (*words).get(0).unwrap_or(&empty).definition) }
                </Text>
                <Text size="3rem" weight={400} color={styler::PALETTE["light-base"]}>
                { format!("{}", (*words).get(0).unwrap_or(&empty).word) }
                </Text>
            </div>
            <div class="panel right">
                <Text size="1.5rem" weight={300} color={get_result_colour(&(*words).get(1).unwrap_or(&empty).definition)}>
                { format!("{}", (*words).get(1).unwrap_or(&empty).definition) }
                </Text>
                <Text size="3rem" weight={400} color={styler::PALETTE["light-base"]}>
                { format!("{}", (*words).get(1).unwrap_or(&empty).word) }
                </Text>
            </div>
            <div class="button-container">
                <Button on_click={handle_next} toggle=false size="1.25em">{ "Next" }</Button>
            </div>
          </div>
        };
    }

    html! {}
}
