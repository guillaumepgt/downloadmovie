FROM rust:latest

WORKDIR /api_send_link

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

COPY . .

RUN cargo build --release

RUN cargo install --path .

CMD cargo run