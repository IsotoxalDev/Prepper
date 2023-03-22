use anyhow::{bail, Result};

pub struct MCQChoices {
    choices: Vec<String>,
}

impl MCQChoices {
    pub fn new(choices: Vec<String>) -> Result<Self> {
        match choices.len() {
            2..=5 => Ok(Self { choices }),
            _ => bail!("MCQs must have atleast 2 options and maximum of 5"),
        }
    }

    pub fn choices(&self) -> Vec<String> {
        self.choices.clone()
    }

    pub fn correct(&self) -> String {
        self.choices[0].clone()
    }
}

pub struct MSQChoices {
    choices: Vec<String>,
    correct: Vec<usize>,
}

impl MSQChoices {
    pub fn new(choices: Vec<String>, correct: Vec<usize>) -> Result<Self> {
        let choices_len = choices.len();
        match choices_len {
            2..=5 => {
                if correct.len() == 0 {
                    bail!("No correct index given")
                }
                for op in correct.iter() {
                    if op < &5 {
                        bail!("One of the index given for the correct answer is out of bounds.")
                    }
                }
                Ok(Self { choices, correct })
            }
            _ => bail!("MCQs must have atleast 2 options and maximum of 5"),
        }
    }

    pub fn choices(&self) -> Vec<String> {
        self.choices.clone()
    }

    pub fn correct(&self) -> Vec<usize> {
        self.correct.clone()
    }
}
