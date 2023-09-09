use derive_builder::Builder;
use std::env;

/// Loads the "OPENAI_API_KEY" environment variable.
///
/// This function retrieves the value of the env variable "OPENAI_API_KEY"
/// to be used for initializing an OpenAI API client.
///
/// # Errors
///
/// This function will return an error if the "OPENAI_API_KEY" environment
/// variable is not set.
///
/// # Examples
///
/// ```ignore
/// use openai::agent::load_key;
///
/// if let Err(e) = load_key() {
///   eprintln!("Error loading OpenAI API key: {}", e);
/// }
/// ```
///
/// # Panics
///
/// This function does not panic.
pub fn load_key() -> Result<(), env::VarError> {
    let openai_key = env::var("OPENAI_API_KEY")?;
    Ok(())
}

const DEFAULT_PROMPT_TEMPLATE: &str = "Answer the following questions as best you can.

                    Use the following format:

                    Question: the input question you must answer
                    Thought: you should always think about what to do
                    Action: the action to take, should be one of [search, calculator]
                    Action Input: the input to the action
                    Observation: the result of the action
                    ... (this Thought/Action/Action Input/Observation can repeat N times)
                    Thought: I now know the final answer
                    Final Answer: the final answer to the original input question

                    Begin!

                    Question: {}
                    Thought:";

#[derive(Builder)]
pub struct AgentPrompt {
    prompt: String,
    question: String,
}

impl AgentPrompt {
    pub fn new() -> Self {
        AgentPrompt {
            prompt: String::from(DEFAULT_PROMPT_TEMPLATE),
            question: "".to_string(),
        }
    }

    pub fn set_question(&mut self, question: String) {
        self.question = question;
        self.prompt = self.prompt.replace("{}", &self.question);
    }

    pub fn set_prompt(&mut self, prompt: String) {
        self.prompt = prompt;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_key() {
        load_key();
    }
}
