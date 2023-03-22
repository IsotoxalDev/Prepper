mod choice;
mod formula;
mod integer;
mod string;

use choice::*;
use formula::*;
use integer::*;
use string::*;

use super::{Answer, Question};
use leptos::*;

#[component]
pub fn QuestionView(cx: Scope, question: ReadSignal<Question>) -> impl IntoView {
    view! { cx,
        <div class="QuestionBox">
            <h1 class="QuestionText">{question.with(|q| q.prompt.clone())}</h1>
        </div>
        {question
            .with(|q| match &q.answer {
                Answer::MCQ(ch) => {
                    view! { cx, <Choice choices=ch.choices() correct=vec![0]/> }
                        .into_view(cx)
                }
                Answer::MSQ(ch) => {
                    view! { cx, <Choice choices=ch.choices() correct=ch.correct()/> }
                        .into_view(cx)
                }
                Answer::Integer(correct) => {
                    view! { cx, <Integer correct=*correct/> }
                        .into_view(cx)
                }
                Answer::String(correct) => {
                    view! { cx, <String correct=correct.into()/> }
                        .into_view(cx)
                }
                Answer::Formula(correct) => {
                    view! { cx, <Formula correct=correct.into()/> }
                        .into_view(cx)
                }
            })}
    }
}
