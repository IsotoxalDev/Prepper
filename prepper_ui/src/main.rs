use prepper_core::{get_question, Answer};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct ChoiceData {
    #[prop_or("Blank Option".into())]
    text: String,
}

#[function_component]
fn Choice(props: &ChoiceData) -> Html {
    html! {
        <button type={"button"}>{&props.text}</button>
    }
}

#[derive(Properties, PartialEq)]
struct QuestionData {
    #[prop_or("Blank Question".into())]
    prompt: String,
    #[prop_or(vec!["Test".into(), "Test".into()])]
    choices: Vec<String>,
}

#[function_component]
fn Question(props: &QuestionData) -> Html {
    let choices: Html = props
        .choices
        .iter()
        .map(|ch| html! {<Choice text={ch.to_string()}/>})
        .collect();
    html! {
        <>
        <h1>{&props.prompt}</h1>
        {choices}

        </>
    }
}

#[function_component]
fn Prepper() -> Html {
    let q = get_question();
    let options = if let Answer::MCQ(choices) = q.answer {
        choices.choices()
    } else {
        vec![]
    };
    html! {
        <>
        {"Prepper"}
        <Question prompt={q.prompt} choices={options}/>
        </>
    }
}

fn main() {
    yew::Renderer::<Prepper>::new().render();
}
