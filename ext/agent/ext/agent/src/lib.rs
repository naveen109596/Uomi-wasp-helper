use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C" {
    // External blockchain log function (from UOMI runtime)
    fn log(s: &str);
}

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub priority: u8,
}

#[wasm_bindgen]
pub fn run_agent(input: &str) -> String {
    // Try to parse the input JSON
    let parsed: Result<Task, _> = serde_json::from_str(input);

    match parsed {
        Ok(task) => {
            let msg = format!(
                "✅ Agent processed task: '{}' with priority {}",
                task.description, task.priority
            );
            log(&msg);
            serde_json::to_string(&msg).unwrap()
        }
        Err(_) => {
            let err = "⚠️ Invalid input format. Please send JSON { description, priority }";
            log(err);
            serde_json::to_string(&err).unwrap()
        }
    }
  }
