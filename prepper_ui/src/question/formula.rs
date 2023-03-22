use leptos::{ev::Event, *};

/// The formula answer set. Uses [MathLive](https://github.com/arnog/mathlive)
/// for input
#[component]
pub fn Formula(cx: Scope, correct: String) -> impl IntoView {
    let (message, set_message) = create_signal(cx, String::new());
    let submit = move |ev: Event| {
        let ans: String = event_target_value(&ev);
        if ans == correct {
            set_message.update(|x| *x = "Correct!!".to_string());
        } else {
            set_message.update(|x| *x = "Wrong".to_string());
        }
    };
    view! {
        cx,
        <div class="Answer Integer">
            <math-field
                id="formula"
                math-virtual-keyboard-policy="auto"
                on:change=submit></math-field>
        </div>
        <h1>{message}</h1>
    }
}
