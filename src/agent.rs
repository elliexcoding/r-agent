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
pub fn load_key() -> Result<(), env::VarError>{
    let openai_key = env::var("OPENAI_API_KEY")?;
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_key() {
        load_key();
    }
}