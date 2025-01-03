use std::process::Command;

/// Create a Lambda function using SAM CLI
pub fn create_lambda_function(
    function_name: &str,
    runtime: &str,
    role_arn: &str,
    handler: &str,
    zip_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("sam")
        .args(&[
            "local", "generate-event", "lambda", "invoke",
            "--function-name", function_name,
            "--runtime", runtime,
            "--role", role_arn,
            "--handler", handler,
            "--zip-file", zip_file,
        ])
        .output()?;

    println!("Lambda creation output: {:?}", output);
    Ok(())
}

/// Invoke a Lambda function using SAM CLI
pub fn invoke_lambda_function(
    function_name: &str,
    payload: &str,
    output_file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let output = Command::new("sam")
        .args(&[
            "local", "invoke",
            "--function-name", function_name,
            "--event", payload,
            "--log-file", output_file,
        ])
        .output()?;

    println!("Lambda invocation output: {:?}", output);
    Ok(())
}
