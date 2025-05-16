use clap::{Parser, Subcommand}

#[derive(Parser)]
#[command(name = "git-cat", about = "Terminal cat🐱")]
pub struct Cli {
    #[command(subcommand)] // サブコマンド（feed, playなど）を指定
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Feed,
    Play,
    Status,
    Grow,
}