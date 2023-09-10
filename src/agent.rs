use derive_builder::Builder;
use reqwest::Client as HttpClient;
use std::env;
use serde::{Deserialize, Serialize};

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
pub fn load_key() -> Result<String, env::VarError> {
    let openai_key = env::var("OPENAI_API_KEY")?;
    Ok(openai_key)
}

const API_URL_V1: &str = "https://api.openai.com/v1/";
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


// API Response
#[derive(Debug, Deserialize, Serialize)]
struct OpenAIResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<OpenAIChoice>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OpenAIChoice {
    text: String,
}

pub struct Client {
    client: HttpClient,
    api_key: String,
    base_url: String,
}

impl Client {
    // Initialize a new client
    pub fn new() -> Self {
        let api_key: String = load_key().expect("Error: OpenAI Key was not found and loaded.");
        Client {
            client: HttpClient::new(),
            api_key,
            base_url: String::from(API_URL_V1),
        }
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
