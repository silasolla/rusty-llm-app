FROM rust:1.87-slim AS builder
WORKDIR /app
COPY . .
RUN apt-get update && apt-get install -y pkg-config libssl-dev
RUN cargo build --release

FROM gcr.io/distroless/cc:latest
WORKDIR /app
COPY --from=builder ./app/target/release/app ./target/release/app
EXPOSE 8080
ENTRYPOINT ["./target/release/app"]
