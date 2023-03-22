use leptos::{html::Input, *};

/// The string answer set. The correct/expected value is passed in
#[component]
pub fn String(cx: Scope, correct: String) -> impl IntoView {
    let (message, set_message) = create_signal(cx, String::new());
    let input_ref = create_node_ref::<Input>(cx);
    let submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let input = input_ref.get().unwrap();
        let ans = input.value();
        if ans == correct {
            set_message.update(|x| *x = "Correct!!".to_string());
        } else {
            set_message.update(|x| *x = "Wrong".to_string());
        }
    };
    view! {
        cx,
        <form on:submit=submit class="Answer Integer">
            <input
            autofocus
            node_ref=input_ref
            placeholder="Answer"/>
            <button>"submit"</button>
        </form>
        <h1>{message}</h1>
    }
}
