FROM rust:1.82-alpine
WORKDIR /app

COPY . .

RUN cargo install --path .

ENTRYPOINT ["advent-of-code-2024"]

CMD ["1"]