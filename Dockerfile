# Use the official Rust image as a base
FROM rust:latest as builder

# Set the working directory
WORKDIR /usr/src/app

# Copy the source code
COPY . .

# Build the project
RUN cargo build --release

# Use a smaller base image with a compatible GLIBC version
FROM debian:bookworm-slim

# Install necessary dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app/target/release/mint /usr/local/bin/mint

# Expose the port the app runs on
EXPOSE 3000

# Set the entry point to run the binary
ENTRYPOINT ["mint"]

# Use the correct CMD syntax to pass arguments
CMD ["--host", "0.0.0.0", "--port", "3000"]

# Ensure the container listens on all interfaces
ENV RUST_BACKTRACE=1
ENV RUST_LOG=info
