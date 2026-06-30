// explain_error(command, output) -> explanation
use anyhow::Result;

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
pub fn explain_error() {}
