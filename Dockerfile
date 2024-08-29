FROM rust:latest

# Setting up work directory
WORKDIR /app

# Copying the source code
COPY ./src /app/src
COPY ./Cargo.toml /app/Cargo.toml

# Building the source code
RUN cargo build --release

# Exposing the binary
CMD ["/app/target/release/axum-redis-example"]