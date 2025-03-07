use clap::Parser;
use mint::{
    core::cli::{
        cli::{Cli, Commands},
        lambda::{create_lambda_function, invoke_lambda_function},
    },
    dynamodb::models::AttributeValue,
    memory::store::{MemoryStore, SharedStore},
    server::create_router,
    sns::service::SnsService,
};
use std::net::SocketAddr;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let store = SharedStore::new(RwLock::new(MemoryStore::default()));
    let sns_service = SnsService::new(store.clone());

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
        Commands::CreateTable { table_name } => {
            let mut data = store.write().await;
            data.dynamo.create_table(table_name.clone());
            println!("Table {} created successfully!", table_name);
        }
        Commands::PutItem { table_name, item } => {
            let mut data = store.write().await;
            let item_map: std::collections::HashMap<String, AttributeValue> =
                serde_json::from_str(item).unwrap();
            data.dynamo.put_item(table_name, item_map);
            println!("Item added to table {} successfully!", table_name);
        }
        Commands::GetItem { table_name, key } => {
            let data = store.read().await;
            if let Some(item) = data.dynamo.get_item(table_name, key) {
                println!("Item retrieved: {:?}", item);
            } else {
                println!("Item not found in table {}", table_name);
            }
        }
        Commands::CreateTopic { name } => {
            let topic_arn = sns_service.create_topic(name.clone()).await;
            println!("Topic {} created successfully!", topic_arn);
        }
        Commands::Publish { topic_arn, message } => {
            if sns_service.publish(topic_arn.clone()).await {
                println!("Message published to topic {}: {}", topic_arn, message);
            } else {
                println!("Topic not found: {}", topic_arn);
            }
        }
        Commands::ListTopics => {
            let topics = sns_service.list_topics().await;
            println!("Topics: {:?}", topics);
        }
        Commands::DeleteTopic { topic_arn } => {
            if sns_service.delete_topic(topic_arn.clone()).await {
                println!("Topic {} deleted successfully!", topic_arn);
            } else {
                println!("Topic not found: {}", topic_arn);
            }
        }
    }
}
