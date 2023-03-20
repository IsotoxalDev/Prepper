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
    fn get_prompt(&self) -> String {
        format!("{}", self.prompt)
    }
    fn new_mcq(prompt: String, answers: Vec<String>) -> Result<Question> {
        let answer = Answer::MCQ(MCQChoices::new(answers)?);
        Ok(Self { prompt, answer })
    }
    fn new_msq(prompt: String, answers: Vec<String>, correct: Vec<usize>) -> Result<Question> {
        let answer = Answer::MSQ(MSQChoices::new(answers, correct)?);
        Ok(Self { prompt, answer })
    }
    fn new_integer(prompt: String, answer: isize) -> Result<Question> {
        let answer = Answer::Integer(answer);
        Ok(Self { prompt, answer })
    }
    fn new_formula(prompt: String, answer: String) -> Result<Question> {
        let answer = Answer::Formula(answer);
        Ok(Self { prompt, answer })
    }
    fn new_string(prompt: String, answer: String) -> Result<Question> {
        let answer = Answer::String(answer);
        Ok(Self { prompt, answer })
    }
}

pub fn get_question() -> Question {
    Question::new_string("How is apple spelled?".to_string(), "Apple".to_string()).unwrap()
}
