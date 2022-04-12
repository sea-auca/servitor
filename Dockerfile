FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin servitor
WORKDIR ./servitor

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/servitor*
RUN cargo build --release

FROM alpine:latest
COPY --from=builder /home/rust/src/servitor/target/x86_64-unknown-linux-musl/release/servitor .

RUN mkdir logs
CMD ["./servitor"]