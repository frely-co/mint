# MINT - Mock Infrastructure and Testing

**MINT** (Mock Infrastructure and Testing) is a lightweight, high-performance mock server designed to simulate AWS services. It provides developers with a realistic, in-memory mock environment for testing and debugging cloud-based applications without relying on real AWS services.

---

## Features

- Simulate key AWS services like:
  - **Cognito**: User authentication and registration.
  - **S3**: Object storage and retrieval.
  - **DynamoDB**: NoSQL database operations.
  - **SQS**: Message queue simulation.
  - **SNS**: Topic-based messaging.
  - **Lambda**: Serverless function execution.
- In-memory data store for fast and reliable testing.
- Language-agnostic API access or SDKs for popular programming languages.
- Ideal for offline development and cost-efficient cloud testing.

---

## Quick Start

### Prerequisites

- Install **Rust**: [Get Started with Rust](https://www.rust-lang.org/tools/install)
- Install **SAM CLI**: [Install SAM CLI](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/serverless-sam-cli-install.html)

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/frelyio/mint.git
   cd mint
   ```

2. Build and run the server:
   ```bash
   cargo build
   cargo run
   ```

3. The server runs on `http://127.0.0.1:3000` by default.

### Running with Docker

```bash
# Build the Docker image
docker build -t mint .

# Run the Docker container, mapping the container's port 3000 to the host's port 3000
docker run -p 3000:3000 mint
```

---

## Example Usage

### Interact with MINT using HTTP

- **Sign up a user (Cognito)**:
  ```bash
  curl -X POST http://127.0.0.1:3000/ \
    -H "X-Amz-Target: AWSCognitoIdentityProviderService.SignUp" \
    -H "Content-Type: application/json" \
    -d '{"username": "testuser", "password": "password123"}'
  ```

- **Store an object in S3**:
  ```bash
  curl -X POST http://127.0.0.1:3000/s3/putObject \
    -H "Content-Type: application/json" \
    -d '{"bucketName": "test-bucket", "key": "example.txt", "content": "Hello, MINT!"}'
  ```

- **Publish a message to SNS**:
  ```bash
  curl -X POST http://127.0.0.1:3000/ \
    -H "Content-Type: application/x-www-form-urlencoded" \
    -d 'Action=Publish&Message=Hello%20MINT'
  ```

- **Put an item in DynamoDB**:
  ```bash
  curl -X POST http://127.0.0.1:3000/ \
    -H "X-Amz-Target: DynamoDB_20120810.PutItem" \
    -H "Content-Type: application/x-amz-json-1.0" \
    -d '{"TableName": "test-table", "Item": {"id": {"S": "1"}, "name": {"S": "test"}}}'
  ```

- **Create a Lambda function**:
  ```bash
  curl -X POST http://127.0.0.1:3000/ \
    -H "X-Amz-Target: AWSLambda.CreateFunction" \
    -H "Content-Type: application/json" \
    -d '{"FunctionName": "test-function", "Runtime": "nodejs14.x", "Role": "arn:aws:iam::123456789012:role/lambda-role", "Handler": "index.handler", "Code": {"ZipFile": "base64-encoded-zip-file"}}'
  ```

- **Invoke a Lambda function**:
  ```bash
  curl -X POST http://127.0.0.1:3000/ \
    -H "X-Amz-Target: AWSLambda.Invoke" \
    -H "Content-Type: application/json" \
    -d '{"FunctionName": "test-function", "Payload": "{\"key\": \"value\"}"}'
  ```

### Interact with MINT using CLI

- **Create a DynamoDB table**:
  ```bash
  cargo run -- create-table --table-name test-table
  ```

- **Put an item in a DynamoDB table**:
  ```bash
  cargo run -- put-item --table-name test-table --item '{"id": {"S": "1"}, "name": {"S": "test"}}'
  ```

- **Get an item from a DynamoDB table**:
  ```bash
  cargo run -- get-item --table-name test-table --key 1
  ```

- **Create a Lambda function**:
  ```bash
  cargo run -- create-lambda --function-name test-function --runtime nodejs14.x --role-arn arn:aws:iam::123456789012:role/lambda-role --handler index.handler --zip-file path/to/zip/file.zip
  ```

- **Invoke a Lambda function**:
  ```bash
  cargo run -- invoke-lambda --function-name test-function --payload '{"key": "value"}' --output-file output.txt
  ```

---

### Use SDKs for Easier Interaction

Use one of the **MINT SDKs** to simplify integration with your preferred language:

- **[Rust SDK](https://github.com/frelyio/mint-sdk-rust)**
- **[Python SDK](https://github.com/frelyio/mint-sdk-python)**
- **[Java SDK](https://github.com/frelyio/mint-sdk-java)**
- **[JavaScript SDK](https://github.com/frelyio/mint-sdk-js)**

Example with the Rust SDK:
```rust
use mint_sdk::{MINTClient, Cognito, S3};

fn main() {
    let client = MINTClient::new("http://127.0.0.1:3000");

    // Cognito
    client.cognito().sign_up("testuser", "password123");

    // S3
    client.s3().create_bucket("test-bucket");
    client.s3().put_object("test-bucket", "example.txt", "Hello, MINT!");
}
```

---

## Contributing

We welcome contributions! Please feel free to:
- Open issues for bug reports or feature requests.
- Submit pull requests to enhance functionality.

---

## License

MINT is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

### Why It Works

- **Concise and User-Friendly**: Focuses on getting developers started quickly.
- **Examples**: Includes HTTP and SDK usage examples for easy reference.
- **Future-Proof**: Mentions SDKs to guide users to extended functionality.
