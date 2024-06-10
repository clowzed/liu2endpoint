FROM rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release
FROM debian:stable-slim
COPY --from=builder /usr/src/app/target/release/liu2endpoint /usr/local/bin/liu2endpoint
CMD ["liu2endpoint"]
