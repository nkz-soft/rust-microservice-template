FROM ubuntu:22.04

EXPOSE 8080

WORKDIR /app

COPY ./target/release/starter .
COPY config.app.toml .

CMD ["./starter"]
