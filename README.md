
```markdown
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
- In-memory data store for fast and reliable testing.
- Language-agnostic API access or SDKs for popular programming languages.
- Ideal for offline development and cost-efficient cloud testing.

---

## Quick Start

### Prerequisites

- Install **Rust**: [Get Started with Rust](https://www.rust-lang.org/tools/install)

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

---

### Use SDKs for Easier Interaction

Use one of the **MINT SDKs** to simplify integration with your preferred language:

- **[Rust SDK](https://github.com/frelyio/mint-sdk-rust)**
- **[Python SDK](https://github.com/frelyiomint-sdk-python)**
- **[Java SDK](https://github.com/frelyio/mint-sdk-java)**
- **[JavaScript SDK](https://github.com/frelyio/mint-sdk-js)**

Example with the Python SDK:
```python
from mint_sdk import MINTClient

client = MINTClient("http://127.0.0.1:3000")

# Cognito
client.cognito().sign_up("testuser", "password123")

# S3
client.s3().create_bucket("test-bucket")
client.s3().put_object("test-bucket", "example.txt", "Hello, MINT!")
```

---

## Contributing

We welcome contributions! Please feel free to:
- Open issues for bug reports or feature requests.
- Submit pull requests to enhance functionality.

---

## License

MINT is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.
```

---

### Why It Works

- **Concise and User-Friendly**: Focuses on getting developers started quickly.
- **Examples**: Includes HTTP and SDK usage examples for easy reference.
- **Future-Proof**: Mentions SDKs to guide users to extended functionality.

