use std::process::Command;

/// Create a Lambda function using SAM CLI
pub fn create_lambda_function(
    function_name: &str,
    runtime: &str,
    role_arn: &str,
    handler: &str,
    zip_file: &str,
) {
    let output = Command::new("sam")
        .args([
            "local",
            "generate-event",
            "lambda",
            "invoke",
            "--function-name", function_name,
            "--runtime", runtime,
            "--role", role_arn,
            "--handler", handler,
            "--zip-file", zip_file,
        ])
        .output()
        .expect("Failed to create lambda function using SAM CLI");
    
    println!("Lambda creation output: {:?}", output);
}

/// Invoke a Lambda function using SAM CLI
pub fn invoke_lambda_function(function_name: &str, payload: &str, output_file: &str) {
    let output = Command::new("sam")
        .args([
            "local",
            "invoke",
            "--function-name", function_name,
            "--event", payload,
            "--log-file", output_file,
        ])
        .output()
        .expect("Failed to invoke lambda using SAM CLI");

    println!("Lambda invocation output: {:?}", output);
}

