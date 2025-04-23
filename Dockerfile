FROM rust:1.81 as builder

WORKDIR /app

COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=builder /app/target/release/handly-backend .

EXPOSE 8080

CMD ["./handly-backend"]
