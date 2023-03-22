use leptos::{html::Input, *};

/// The integer answer set, It takes in the correct value.
#[component]
pub fn Integer(cx: Scope, correct: isize) -> impl IntoView {
    let (message, set_message) = create_signal(cx, String::new());
    let input_ref = create_node_ref::<Input>(cx);
    let submit = move |ev: ev::SubmitEvent| {
        ev.prevent_default();
        let input = input_ref.get().unwrap();
        let ans: isize = input.value().parse().unwrap();
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
            placeholder="Answer" type="number"/>
            <button>"submit"</button>
        </form>
        <h1>{message}</h1>
    }
}
