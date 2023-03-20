use leptos::*;
use std::time::Duration;

#[component]
pub fn Timer(cx: Scope) -> impl IntoView {
    let (time, set_time) = create_signal(cx, 0);
    let update = move || set_time.update(|value| *value += 1);

    set_interval(update, Duration::from_secs(1));

    view! {
        cx,
        <div class="Timer">
        {time}
        </div>
    }
}

#[component]
pub fn TopMenu(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="TopMenu">
            <button>"back"</button>
            <Timer />
        </div>
    }
}
