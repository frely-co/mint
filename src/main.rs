use clap::Parser;
use mint::{
    core::cli::{
        cli::{Cli, Commands},
        lambda::{create_lambda_function, invoke_lambda_function},
    },
    memory::store::{MemoryStore, SharedStore},
    server::create_router,
};
use std::net::SocketAddr;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let store = SharedStore::new(RwLock::new(MemoryStore::default()));

    match &cli.command {
        Commands::Server => {
            let app = create_router(store);
            let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
            println!("Mock AWS service running at http://{}", addr);
            axum_server::bind(addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        Commands::CreateUser { username, password } => {
            let mut data = store.write().await;
            data.cognito
                .users
                .insert(username.clone(), password.clone());
            println!("User {} created successfully!", username);
        }
        Commands::AuthenticateUser { username, password } => {
            let data = store.read().await;
            if let Some(stored_password) = data.cognito.users.get(username) {
                if stored_password == password {
                    println!("User {} authenticated successfully!", username);
                } else {
                    println!("Invalid password for user {}", username);
                }
            } else {
                println!("User not found: {}", username);
            }
        }
        Commands::CreateLambda {
            function_name,
            runtime,
            role_arn,
            handler,
            zip_file,
        } => match create_lambda_function(function_name, runtime, role_arn, handler, zip_file) {
            Ok(_) => println!("Lambda function created successfully"),
            Err(e) => eprintln!("Failed to create Lambda function: {}", e),
        },
        Commands::InvokeLambda {
            function_name,
            payload,
            output_file,
        } => match invoke_lambda_function(function_name, payload, output_file) {
            Ok(_) => println!("Lambda function invoked successfully"),
            Err(e) => eprintln!("Failed to invoke Lambda function: {}", e),
        },
    }
}
