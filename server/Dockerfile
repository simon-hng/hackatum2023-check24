# Use the official Rust image as the base image
FROM rust:bookworm as builder
WORKDIR /code

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

# Copy only the necessary files from the builder image for the main project
COPY --from=builder /code/target/release/server server

# Expose the port that your Rust server will be running on
EXPOSE 8080

# Command to run the application
CMD ["/app/server"]
