use colored::*;
use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Write, BufRead};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatRequest {
    question: String,
    context: Vec<Message>,
    model: String,
}

struct Spinner {
    active: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Spinner {
    fn new() -> Self {
        Spinner {
            active: Arc::new(AtomicBool::new(false)),
            handle: None,
        }
    }

    fn start(&mut self) {
        self.active.store(true, Ordering::SeqCst);
        let active = self.active.clone();
        
        self.handle = Some(thread::spawn(move || {
            let spinner_chars = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let mut i = 0;
            
            while active.load(Ordering::SeqCst) {
                print!("\r{} Thinking...", spinner_chars[i].bright_blue());
                io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(80));
                i = (i + 1) % spinner_chars.len();
            }
            print!("\r");
            io::stdout().flush().unwrap();
        }));
    }

    fn stop(&mut self) {
        self.active.store(false, Ordering::SeqCst);
        if let Some(handle) = self.handle.take() {
            handle.join().unwrap();
        }
        print!("\r");
        io::stdout().flush().unwrap();
    }
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
    model: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let request = ChatRequest { question, context, model };

    let response = client
        .post(webhook_url)
        .json(&request)
        .send()
        .await?
        .text()
        .await?;

    Ok(response)
}

fn select_model() -> String {
    loop {
        println!("\n{}", "Select a model:".bright_blue());
        println!("1. Claude-3.5-sonnet");
        println!("2. o3-mini");
        println!("3. Claude-3.7-sonnet");
        println!("4. Claude-3.7-sonnet-thinking");
        print!("{} ", ">>>".bright_blue());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().lock().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => return "claude-3.5-sonnet".to_string(),
            "2" => return "o3-mini".to_string(),
            "3" => return "claude-3.7-sonnet".to_string(),
            "4" => return "claude-3.7-sonnet-thinking".to_string(),
            _ => println!("{}", "Invalid selection. Please choose 1, 2, 3, or 4.".bright_red()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let webhook_url = env::var("WEBHOOK_URL")
        .expect("WEBHOOK_URL must be set in environment variables");

    println!("{}", "\nWelcome to Latenode CLI!".bright_green());
    
    let model = select_model();
    println!("{} {}", "\nUsing model:".bright_blue(), model);
    println!("\nType '/send' to submit your input");
    println!("Type '/exit' to end the session\n");

    let mut context = Vec::new();
    let mut spinner = Spinner::new();

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

        spinner.start();
        
        match send_chat_request(&webhook_url, input, context.clone(), model.clone()).await {
            Ok(response) => {
                spinner.stop();
                print!("{} ", "$".bright_green());
                println!("{}", format_code_blocks(&response));

                context.push(Message {
                    role: "assistant".to_string(),
                    content: response,
                });
            }
            Err(e) => {
                spinner.stop();
                println!("{} {}", "Error:".bright_red(), e);
            }
        }
    }

    Ok(())
}
