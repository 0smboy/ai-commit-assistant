// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    api_key: String,
    api_type: String,
    repo_path: String,
}

#[derive(Debug, Serialize)]
struct CommitSuggestion {
    message: String,
    diff: String,
}

struct AppState {
    config: Arc<Mutex<Config>>,
}

#[tauri::command]
async fn generate_commit(
    repo_path: String,
    state: State<'_, AppState>,
) -> Result<CommitSuggestion, String> {
    let config = state.config.lock().await;
    
    // Get git diff
    let diff = get_git_diff(&repo_path).map_err(|e| e.to_string())?;
    
    // Generate commit message using selected API
    let message = match config.api_type.as_str() {
        "openai" => generate_openai_commit(&diff, &config.api_key).await?,
        "anthropic" => generate_anthropic_commit(&diff, &config.api_key).await?,
        _ => return Err("Unsupported API type".to_string()),
    };

    Ok(CommitSuggestion { message, diff })
}

#[tauri::command]
async fn update_config(
    api_key: String,
    api_type: String,
    repo_path: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut config = state.config.lock().await;
    config.api_key = api_key;
    config.api_type = api_type;
    config.repo_path = repo_path;
    Ok(())
}

async fn generate_openai_commit(diff: &str, api_key: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&serde_json::json!({
            "model": "gpt-3.5-turbo",
            "messages": [
                {
                    "role": "system",
                    "content": "You are a helpful assistant that generates concise and meaningful git commit messages."
                },
                {
                    "role": "user",
                    "content": format!("Generate a concise git commit message for these changes:\n\n{}", diff)
                }
            ]
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    Ok(data["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("Failed to generate commit message")
        .to_string())
}

async fn generate_anthropic_commit(diff: &str, api_key: &str) -> Result<String, String> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&serde_json::json!({
            "model": "claude-3-haiku-20240307",
            "max_tokens": 100,
            "messages": [{
                "role": "user",
                "content": format!("Generate a concise git commit message for these changes:\n\n{}", diff)
            }]
        }))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let data: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;
    Ok(data["content"][0]["text"]
        .as_str()
        .unwrap_or("Failed to generate commit message")
        .to_string())
}

fn get_git_diff(repo_path: &str) -> Result<String, git2::Error> {
    let repo = git2::Repository::open(repo_path)?;
    let mut diff_opts = git2::DiffOptions::new();
    let diff = repo.diff_index_to_workdir(None, Some(&mut diff_opts))?;
    
    let mut diff_text = String::new();
    diff.print(git2::DiffFormat::Patch, |_, _, line| {
        diff_text.push_str(&String::from_utf8_lossy(line.content()));
        true
    })?;
    
    Ok(diff_text)
}

fn main() {
    let config = Arc::new(Mutex::new(Config {
        api_key: String::new(),
        api_type: "openai".to_string(),
        repo_path: String::new(),
    }));

    tauri::Builder::default()
        .manage(AppState {
            config: config.clone(),
        })
        .invoke_handler(tauri::generate_handler![
            generate_commit,
            update_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
