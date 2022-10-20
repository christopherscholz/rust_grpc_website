################
##### Builder
FROM rust as builder

RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev

WORKDIR /rust

RUN USER=root cargo new app
COPY Cargo.toml /rust/app/
WORKDIR /rust/app
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

COPY build.rs /rust/app/
COPY src /rust/app/src/
COPY proto /rust/app/proto/
RUN touch /rust/app/src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release


################
##### Runtime
FROM alpine as runner
COPY --from=builder /rust/app/target/x86_64-unknown-linux-musl/release/grpc_server /usr/local/bin
WORKDIR /rust/app
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["/usr/local/bin/grpc_server"]