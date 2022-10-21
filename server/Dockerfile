################
##### Builder
FROM rust as builder

# install protobuf
RUN apt-get update && apt-get install -y protobuf-compiler libprotobuf-dev

# pre build app
WORKDIR /usr/src
RUN USER=root cargo new app
COPY Cargo.toml /usr/src/app/
WORKDIR /usr/src/app
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release

# build app
COPY build.rs /usr/src/app/
COPY src /usr/src/app/src/
COPY proto /usr/src/app/proto/
RUN touch /usr/src/app/src/main.rs
RUN cargo build --target x86_64-unknown-linux-musl --release


################
##### Runtime
FROM gcr.io/distroless/static-debian11 as runner

# get binary
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/grpc_server /

# set run env
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

# run it
CMD ["/grpc_server"]