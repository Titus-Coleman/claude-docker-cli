FROM --platform=linux/amd64 rust:latest as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM --platform=linux/amd64 ubuntu:latest
COPY --from=builder /usr/src/app/target/release/claude-docker-cli /usr/local/bin/claude-docker-cli
CMD ["claude-docker-cli"]