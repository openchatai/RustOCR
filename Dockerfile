# Dockerfile
FROM rust:latest

WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the Rust application
RUN cargo build --release

# Set the entry point for the application
CMD ["./target/release/scanner"]