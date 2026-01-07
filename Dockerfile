FROM rust:1.92-slim-bookworm AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app

RUN adduser book && chown -R book /app
USER book
COPY --from=builder ./app/target/release/book_manager ./target/release/app

# Open port 8080 and run application
ENV PORT 8080
EXPOSE $PORT
ENTRYPOINT [ "./target/release/app" ]
