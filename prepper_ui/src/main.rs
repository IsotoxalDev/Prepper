use leptos::*;
use prepper_core::{get_question, Answer, Question};

mod question;
mod top_menu;

use question::*;
use top_menu::*;

#[component]
pub fn Prepper(cx: Scope) -> impl IntoView {
    let (question, _set_question) = create_signal(cx, get_question());
    view! { cx,
        <TopMenu/>
        <QuestionView question={question}/>
    }
}

pub fn main() {
    mount_to_body(move |cx| view! { cx, <Prepper/> });
}
