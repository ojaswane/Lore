// explain_error(command, output) -> explanation
use anyhow::Result;
use serde::Deserialize;

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

#[derive(Deserialize)]
struct OllamaResponse {
    response: String,
}

pub fn explain_error(command: &str, output: &str, exit_code: i32) -> Result<AiString> {
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
    {output}

    Exit code:
    {exit_code}"
    );

    // response
    let response_json = reqwest::blocking::Client::new()
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "lore",
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": 0.5,
                "num_predict": 200
            }
        }))
        .send()
        .and_then(|response| response.error_for_status())?
        .json::<OllamaResponse>()?;

    Ok(AiString {
        explanation: response_json.response,
        fix: String::from("Review the explanation above and run the safest next command manually."),
        what_it_does: String::from(
            "Asks your local Ollama model to explain the last terminal output.",
        ),
    })
}

pub fn ask_question(
    question: &str,
    command: &str,
    output: &str,
    exit_code: i32,
) -> Result<AiString> {
    let prompt = format!(
        "You are Lore, a local terminal assistant.

    Answer the user's question using the terminal context below.
    Be concise.
    Do not invent facts.
    If the question asks for a command, suggest one safe command and explain it briefly.

    User question:
    {question}

    Last command:
    {command}

    Last output:
    {output}

    Exit code:
    {exit_code}"
    );

    let response_json = reqwest::blocking::Client::new()
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": "lore",
            "prompt": prompt,
            "stream": false,
            "options": {
                "temperature": 0.5,
                "num_predict": 250
            }
        }))
        .send()
        .and_then(|response| response.error_for_status())?
        .json::<OllamaResponse>()?;

    Ok(AiString {
        explanation: response_json.response,
        fix: String::from("Use the answer above as guidance before running anything."),
        what_it_does: String::from("Answers your sidebar question with the last terminal output as context."),
    })
}
