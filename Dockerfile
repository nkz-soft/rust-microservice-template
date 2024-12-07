FROM  gcr.io/distroless/cc-debian12:nonroot

EXPOSE 8080

WORKDIR /app

COPY ./target/release/starter .
COPY config.app.toml .

USER nonroot
CMD ["./starter"]
