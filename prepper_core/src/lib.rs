mod choices;

use anyhow::{bail, Result};
use choices::{MCQChoices, MSQChoices};

pub enum Answer {
    MCQ(MCQChoices),
    MSQ(MSQChoices),
    Integer(isize),
    Formula(String),
    String(String),
}

pub struct Question {
    pub prompt: String,
    pub answer: Answer,
}

impl Question {
    fn new_mcq(prompt: String, answers: Vec<String>) -> Result<Question> {
        let answer = Answer::MCQ(MCQChoices::new(answers)?);
        Ok(Self { prompt, answer })
    }
}

pub fn get_question() -> Question {
    Question::new_mcq("I am stupid ?".into(), vec!["Yes".into(), "No".into()]).unwrap()
}
