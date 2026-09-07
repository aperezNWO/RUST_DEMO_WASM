# Stage 1: Build the Rust binary
FROM rust:1.75-slim AS builder

WORKDIR /app

# Copy dependency manifests
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to cache dependency compilation
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy actual source code
COPY . .

# Build the real binary
RUN cargo build --release --bin your_binary_name

# Stage 2: Runtime image
FROM debian:bookworm-slim

WORKDIR /app

# Install necessary runtime certificates/dependencies if required
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy compiled binary from the builder stage
COPY --from=builder /app/target/release/your_binary_name /app/server

# Expose port (adjust to match your application configuration)
EXPOSE 8080

# Command to run the application
CMD ["/app/server"]