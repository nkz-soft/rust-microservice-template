FROM ubuntu:22.04

EXPOSE 8080

WORKDIR /app

COPY ./target/release/rust-microservice-template .
COPY config.app.toml .

CMD ["./rust-microservice-template"]
