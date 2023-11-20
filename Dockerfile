FROM debian:stable-slim as build

# Install Rust
RUN apt-get update && \
    apt-get install -y curl && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . $HOME/.cargo/env

# Refresh terminal session here (you might need to start a new shell to apply the changes)

RUN apt-get install -y pkg-config
# Install dependencies
RUN apt-get update && \
    apt-get install -y build-essential && \
    apt-get install -y tesseract-ocr libtesseract-dev libleptonica-dev liblept5 && \
    apt-get install -y libclang-dev

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src
# Create a dummy main.rs file
RUN echo 'fn main() { println!("Hello, Docker!"); }' > src/main.rs

RUN . $HOME/.cargo/env && cargo install --path=.

RUN rm src/main.rs

COPY . .

# Build the Rust application
RUN . $HOME/.cargo/env && cargo build --release

# New stage to copy image into Alpine
FROM debian:stable-slim
RUN apt-get update && apt-get install -y tesseract-ocr
COPY --from=build /app/target/release/scanner /usr/local/bin/scanner
CMD ["/usr/local/bin/scanner"]
