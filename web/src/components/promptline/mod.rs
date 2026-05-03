use leptos::prelude::*;
use leptos::{ev, html};
use std::cmp::min;

stylance::import_style!(style, "promptline.css");

fn levenshtein(lhs: &str, rhs: &str) -> usize {
    if lhs == rhs {
        return 0;
    }

    let lhs_chars: Vec<char> = lhs.chars().collect();
    let rhs_chars: Vec<char> = rhs.chars().collect();

    if lhs_chars.is_empty() {
        return rhs_chars.len();
    }

    if rhs_chars.is_empty() {
        return lhs_chars.len();
    }

    let mut previous: Vec<usize> = (0..=rhs_chars.len()).collect();

    for (lhs_index, lhs_char) in lhs_chars.iter().enumerate() {
        let mut current = vec![lhs_index + 1; rhs_chars.len() + 1];

        for (rhs_index, rhs_char) in rhs_chars.iter().enumerate() {
            let substitution_cost = if lhs_char == rhs_char {
                previous[rhs_index]
            } else {
                previous[rhs_index] + 1
            };

            let insertion_cost = current[rhs_index] + 1;
            let deletion_cost = previous[rhs_index + 1] + 1;

            current[rhs_index + 1] = min(substitution_cost, min(insertion_cost, deletion_cost));
        }

        previous = current;
    }

    previous[rhs_chars.len()]
}

fn closest_history_match(input: &str, history: &[String]) -> Option<String> {
    if input.trim().is_empty() {
        return history.last().cloned();
    }

    let normalized_input = input.to_lowercase();

    let mut prefix_best: Option<(usize, String)> = None;
    for candidate in history.iter().rev().take(10) {
        let normalized_candidate = candidate.to_lowercase();
        if normalized_candidate.starts_with(&normalized_input) {
            let continuation_len = candidate.len().saturating_sub(input.len());
            match &prefix_best {
                Some((best_len, _)) if *best_len <= continuation_len => {}
                _ => prefix_best = Some((continuation_len, candidate.clone())),
            }
        }
    }

    if let Some((_, candidate)) = prefix_best {
        return Some(candidate);
    }

    let mut best_distance = usize::MAX;
    let mut best_match: Option<String> = None;

    for candidate in history.iter().rev().take(10) {
        let distance = levenshtein(&normalized_input, &candidate.to_lowercase());
        if distance < best_distance {
            best_distance = distance;
            best_match = Some(candidate.clone());
        }
    }

    best_match.filter(|candidate| candidate != input)
}

#[component]
pub fn Promptline(
    history: ReadSignal<Vec<String>>,
    on_submit: Callback<String>,
    #[prop(default = "❯".to_string())] symbol: String,
) -> impl IntoView {
    let (input_value, set_input_value) = signal(String::new());
    let (history_index, set_history_index) = signal(None::<usize>);
    let (history_draft, set_history_draft) = signal(String::new());
    let input_ref: NodeRef<html::Textarea> = NodeRef::new();

    let suggestion_candidate = Memo::new(move |_| {
        let value = input_value.get();
        let submitted_history = history.get();
        closest_history_match(&value, &submitted_history)
    });

    let suggestion_continuation = Memo::new(move |_| {
        let value = input_value.get();
        let candidate = suggestion_candidate.get();

        match candidate {
            Some(command) if command.starts_with(value.as_str()) && command != value => {
                command.get(value.len()..).map(ToString::to_string)
            }
            _ => None,
        }
    });

    let submit_command = {
        let on_submit = on_submit.clone();
        move || {
            let command = input_value.get_untracked().trim().to_string();
            if command.is_empty() {
                return;
            }

            on_submit.run(command);
            set_input_value.set(String::new());
            set_history_index.set(None);
            set_history_draft.set(String::new());
        }
    };

    Effect::new(move |_| {
        let _ = input_value.get();
        if let Some(input) = input_ref.get() {
            let _ = input.set_attribute("style", "height: auto;");
            let _ = input.set_attribute("style", &format!("height: {}px;", input.scroll_height()));
        }
    });

    Effect::new(move |_| {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    });

    window_event_listener(ev::click, move |_| {
        if let Some(input) = input_ref.get() {
            let _ = input.focus();
        }
    });

    view! {
        <form
            class=style::promptline
            on:submit=move |event| {
                event.prevent_default();
                submit_command();
            }
        >
            <span class=style::symbol>{symbol}</span>
            <div class=style::input_wrap>
                <div class=style::overlay>
                    <span class=style::typed>{move || input_value.get()}</span>
                    <span class=style::ghost>{move || suggestion_continuation.get().unwrap_or_default()}</span>
                    <span class=style::tab_hint>
                        {move || {
                            if suggestion_continuation.get().is_some() {
                                " [tab]"
                            } else {
                                ""
                            }
                        }}
                    </span>
                </div>
                <textarea
                    class=style::input
                    node_ref=input_ref
                    rows="1"
                    spellcheck="false"
                    prop:value=move || input_value.get()
                    on:input:target=move |event| {
                        set_input_value.set(event.target().value());
                        set_history_index.set(None);
                    }
                    on:keydown=move |event| {
                        if event.key() == "Tab" {
                            event.prevent_default();
                            if let Some(next) = suggestion_candidate.get_untracked() {
                                set_input_value.set(next);
                                set_history_index.set(None);
                            }
                        } else if event.key() == "Enter" && !event.shift_key() {
                            event.prevent_default();
                            submit_command();
                        } else if event.key() == "ArrowUp" {
                            let submitted_history = history.get_untracked();
                            if submitted_history.is_empty() {
                                return;
                            }

                            event.prevent_default();

                            match history_index.get_untracked() {
                                None => {
                                    set_history_draft.set(input_value.get_untracked());
                                    let next_index = submitted_history.len() - 1;
                                    set_history_index.set(Some(next_index));
                                    set_input_value.set(submitted_history[next_index].clone());
                                }
                                Some(0) => {}
                                Some(current) => {
                                    let next_index = current - 1;
                                    set_history_index.set(Some(next_index));
                                    set_input_value.set(submitted_history[next_index].clone());
                                }
                            }
                        } else if event.key() == "ArrowDown" {
                            let submitted_history = history.get_untracked();
                            if submitted_history.is_empty() {
                                return;
                            }

                            event.prevent_default();

                            match history_index.get_untracked() {
                                None => {}
                                Some(current) if current + 1 < submitted_history.len() => {
                                    let next_index = current + 1;
                                    set_history_index.set(Some(next_index));
                                    set_input_value.set(submitted_history[next_index].clone());
                                }
                                Some(_) => {
                                    set_history_index.set(None);
                                    set_input_value.set(history_draft.get_untracked());
                                }
                            }
                        }
                    }
                />
            </div>
        </form>
    }
}
