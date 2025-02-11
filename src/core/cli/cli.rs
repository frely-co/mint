use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mint")]
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
    CreateUser { username: String, password: String },
    /// Authenticate a Cognito user
    AuthenticateUser { username: String, password: String },
    CreateLambda {
        function_name: String,
        runtime: String,
        role_arn: String,
        handler: String,
        zip_file: String,
    },
    /// Invoke a Lambda function using SAM CLI
    InvokeLambda {
        function_name: String,
        payload: String,
        output_file: String,
    },
    /// Create a DynamoDB table
    CreateTable { table_name: String },
    /// Put an item in a DynamoDB table
    PutItem { table_name: String, item: String },
    /// Get an item from a DynamoDB table
    GetItem { table_name: String, key: String },
}
