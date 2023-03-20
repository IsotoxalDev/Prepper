use leptos::*;

#[component]
pub fn Choices(cx: Scope, choices: Vec<String>) -> impl IntoView {
    let (choices, _) = create_signal(cx, choices);
    view! {
        cx,
        <div class="MCQAnswer">
            <For
                each={choices}
                key=|text| text.to_string()
                view=move |cx, text: String| view! {cx,
                    <button class="Choice"><b>{text}</b></button>
                }
            />
        </div>
    }
}
