FROM rust:1.82-alpine3.19

WORKDIR /app

COPY . .

RUN cargo install --path .

CMD ["advent-of-code-2024"]