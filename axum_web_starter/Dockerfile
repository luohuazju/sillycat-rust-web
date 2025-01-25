FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN SQLX_OFFLINE=true cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/axum_web_starter /app/axum_web_starter
EXPOSE 8080
CMD ["/app/axum_web_starter"]
