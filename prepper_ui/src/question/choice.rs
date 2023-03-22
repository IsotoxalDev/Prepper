use leptos::*;

/// The input for MCQ questions. This component assumes that the given data of
/// choices is not more than 5.
#[component]
pub fn Choice(cx: Scope, choices: Vec<String>, correct: Vec<usize>) -> impl IntoView {
    let (choices, _) = create_signal(cx, choices);
    view! { cx,
        <div class="MCQAnswer">
            <For
                each={choices}
                key=|text| text.to_string()
                view=move |cx, text: String| {
                    view! { cx,
                        <button class="Choice">
                            <b>{text}</b>
                        </button>
                    }
                }
            />
        </div>
    }
}
