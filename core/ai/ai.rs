// explain_error(command, output) -> explanation
use anyhow::Result;
use color_eyre::eyre::Error;

// // pipeline
// main.rs detects error (exit_code != 0)
//        ↓
// gather context:
//   - the command that failed
//   - the full output/error message
//   - the current directory
//   - maybe last 2-3 commands from db (for context)
//        ↓
// core/ai.rs builds a prompt from this context
//        ↓
// sends HTTP request to ollama (localhost:11434)
//        ↓
// ollama model processes and returns explanation + fix
//        ↓
// core/ai.rs parses the response
//        ↓
// main.rs receives parsed result
//        ↓
// updates ai_state struct (context, explanation, fix, what_it_does)
//        ↓
// ui/ai.rs renders ai_state on screen

pub struct AiString {
    pub explanation: String,
    pub fix: String,
    pub what_it_does: String,
}

pub async fn explain_error(command: &str, output: &str, exit_code: i32) -> Result<AiString> {
    // prompt to send to ollama
    let prompt = format!(
        "You are Lore, a local terminal assistant.

    Explain the terminal output below.
    Be concise.
    Do not invent facts.
    If there is an error, explain:
    1. what failed
    2. why it likely failed
    3. the next command to try

    Command:
    {command}

    Output:
    {output}"
    );

    // response
    let response = reqwest::Client::new()
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "lore",
            "prompt": prompt,
            "max_tokens": 200,
            "temperature": 0.5,
        }))
        .send()
        .await
        .map_err(|e| Error::from(e))?;

    let _response_json = response
        .json::<serde_json::Value>()
        .await
        .map_err(|e| Error::from(e))?;

    Ok(AiString {
        explanation: String::new(),
        fix: String::new(),
        what_it_does: String::new(),
    })
}
