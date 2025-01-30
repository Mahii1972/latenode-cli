use colored::*;
use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Write, BufRead};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    question: String,
    context: Vec<Message>,
}

fn format_code_blocks(text: &str) -> String {
    let mut formatted = String::new();
    let mut in_code_block = false;
    let mut lines = text.lines().peekable();

    while let Some(line) = lines.next() {
        if line.contains("```") {
            if !in_code_block {
                // Start of code block
                in_code_block = true;
                formatted.push_str(&format!("{}\n", "=".repeat(80).bright_blue()));
                // Skip the language identifier line
                if let Some(next_line) = lines.next() {
                    if !next_line.contains("```") {
                        formatted.push_str(&format!("{}\n", next_line.bright_yellow()));
                    }
                }
            } else {
                // End of code block
                in_code_block = false;
                formatted.push_str(&format!("{}\n", "=".repeat(80).bright_blue()));
            }
        } else if in_code_block {
            // Inside code block - format with different color
            formatted.push_str(&format!("{}\n", line.bright_yellow()));
        } else {
            // Regular text
            formatted.push_str(&format!("{}\n", line));
        }
    }
    formatted
}

async fn send_chat_request(
    webhook_url: &str,
    question: String,
    context: Vec<Message>,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let request = ChatRequest { question, context };

    let response = client
        .post(webhook_url)
        .json(&request)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let webhook_url = env::var("WEBHOOK_URL")
        .expect("WEBHOOK_URL must be set in environment variables");

    println!("{}", "\nWelcome to Latenode CLI!".bright_green());
    println!("Type '/exit' to end the session\n");

    let mut context = Vec::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("{} ", ">>>".bright_blue());
        stdout.flush().unwrap();

        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "/exit" {
            println!("{}", "\nExiting...".bright_yellow());
            break;
        }

        if input.is_empty() {
            continue;
        }

        // Add user message to context
        context.push(Message {
            role: "user".to_string(),
            content: input.to_string(),
        });

        match send_chat_request(&webhook_url, input.to_string(), context.clone()).await {
            Ok(response) => {
                print!("{} ", "$".bright_green());
                // Format and print the response with code block highlighting
                println!("{}", format_code_blocks(&response));

                // Add assistant response to context
                context.push(Message {
                    role: "assistant".to_string(),
                    content: response,
                });
            }
            Err(e) => {
                println!("{} {}", "Error:".bright_red(), e);
            }
        }
    }

    Ok(())
}
