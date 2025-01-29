# Latenode CLI Chatbot

A simple terminal-based AI chatbot using Latenode's webhook. This hobby project allows you to have conversations with an AI assistant directly from your terminal.

## About
This is a hobby project that turns Latenode's webhook functionality into a terminal chatbot. Built with Rust, it provides a clean and simple interface for chatting with AI right from your command line.

Features:
- Clean terminal interface
- Maintains conversation context within sessions
- Simple commands (just type naturally, '/exit' to quit)
- Lightweight and fast

## Setup

### Prerequisites
- Rust and Cargo installed
- Latenode webhook URL

### Installation

1. Clone the repository
```bash
git clone https://github.com/yourusername/latenode-cli.git
cd latenode-cli
```

2. Create a `.env` file in the project root with your Latenode webhook URL:
```bash
WEBHOOK_URL=https://webhook.latenode.com/xxxx/prod/xxxx-xxxx-xxxx-xxxx-xxxx
```
3. Build the project:
```bash
cargo build --release
```
4. Run the project:
```bash
./target/release/latenode-cli
```

To exit the chat, simply type `/exit`.


## Updates

To update the application, simply run the following command:
```bash
git pull
cargo build --release
```

## Changelog
- 2025-01-29: Initial release

