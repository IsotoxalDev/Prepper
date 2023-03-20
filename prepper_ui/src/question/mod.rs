mod choice;
mod integer;
mod string;

use choice::*;
use integer::*;
use string::*;

use super::{Answer, Question};
use leptos::*;

#[component]
pub fn QuestionView(cx: Scope, question: ReadSignal<Question>) -> impl IntoView {
    view! {
        cx,
        <div class="QuestionBox">
            <h1 class="QuestionText">{question.with(|q| q.prompt.clone())}</h1>
        </div>
            {
                question.with(|q| match &q.answer {
                    Answer::MCQ(ch) => view!{cx, <Choices choices=ch.choices()/>}.into_view(cx),
                    Answer::MSQ(ch) => view!{cx, <Choices choices=ch.choices()/>}.into_view(cx),
                    Answer::Integer(correct) => view!{cx, <Integer correct=*correct/>}.into_view(cx),
                    Answer::String(correct) => view!{cx, <String correct=correct.into()/>}.into_view(cx),
                    _ => todo!{}
                })            }
    }
}
