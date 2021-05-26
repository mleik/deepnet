FROM rust:1.50-alpine3.13 as builder

RUN apk add --no-cache build-base

WORKDIR /app
COPY . .

RUN cargo build --release --bin deepnet-operator

FROM alpine:3.13.2

WORKDIR /app
COPY --from=builder /app/target/release/deepnet-operator /app/operator
USER daemon

EXPOSE 8080
CMD ["/app/operator"]