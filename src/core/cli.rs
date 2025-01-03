
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mint-cli")]
#[command(about = "CLI for Mint AWS Mock Library")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run the server
    Server,
    /// Create a Cognito user
    CreateUser {
        username: String,
        password: String,
    },
    /// Authenticate a Cognito user
    AuthenticateUser {
        username: String,
        password: String,
    },
}

