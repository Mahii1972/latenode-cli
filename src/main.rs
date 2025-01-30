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

fn read_multiline_input() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut lines = Vec::new();
    
    loop {
        print!("{} ", ">>>".bright_blue());
        stdout.flush().unwrap();
        
        buffer.clear();
        stdin.lock().read_line(&mut buffer).unwrap();
        
        let line = buffer.trim();
        
        if line == "/send" {
            break;
        }
        
        if line == "/exit" {
            return "/exit".to_string();
        }
        
        lines.push(line.to_string());
    }
    
    lines.join("\n")
}

fn format_code_blocks(text: &str) -> String {
    let mut formatted = String::new();
    let mut in_code_block = false;
    let mut lines = text.lines().peekable();

    while let Some(line) = lines.next() {
        if line.contains("```") {
            if !in_code_block {
                in_code_block = true;
                formatted.push_str(&format!("{}\n", "=".repeat(80).bright_blue()));
            } else {
                in_code_block = false;
                formatted.push_str(&format!("{}\n", "=".repeat(80).bright_blue()));
            }
        } else if in_code_block {
            formatted.push_str(&format!("{}\n", line.bright_yellow()));
        } else {
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
    println!("Type '/send' to submit your input");
    println!("Type '/exit' to end the session\n");

    let mut context = Vec::new();

    loop {
        let input = read_multiline_input();

        if input == "/exit" {
            println!("{}", "\nExiting...".bright_yellow());
            break;
        }

        if input.is_empty() {
            continue;
        }

        context.push(Message {
            role: "user".to_string(),
            content: input.clone(),
        });


        match send_chat_request(&webhook_url, input, context.clone()).await {
            Ok(response) => {
                print!("{} ", "$".bright_green());
                println!("{}", format_code_blocks(&response));

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
