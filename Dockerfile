# Build Stage
FROM docker.io/rust:1.52 as builder
WORKDIR /usr/src/
RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new ocp-rust-ex
WORKDIR /usr/src/ocp-rust-ex
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Bundle Stage
FROM scratch
COPY --from=builder /usr/local/cargo/bin/ocp_rust_ex .
ENTRYPOINT ["./ocp_rust_ex"]
