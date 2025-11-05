FROM rust:latest

WORKDIR /usr/local/ade

COPY src/ ./src/
COPY Cargo.toml .
COPY Cargo.lock .

RUN apt-get update && apt-get install -y openssl pkg-config && rm -rf /var/lib/apt/lists/*;
RUN cargo build --release;

CMD ["/usr/local/ade/target/release/ade"]
