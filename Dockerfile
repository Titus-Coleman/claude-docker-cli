FROM --platform=linux/amd64 rust:latest as builder
WORKDIR /usr/src/app
COPY . .
ARG ENV_FILE=.env
ENV $(shell sed 's/=.*//' ${ENV_FILE})
RUN --mount=type=secret,id=env,target=${ENV_FILE} cargo build --release

FROM --platform=linux/amd64 ubuntu:latest
COPY --from=builder /usr/src/app/target/release/claude-docker-cli /usr/local/bin/claude-docker-cli
COPY .env /usr/local/bin/
CMD ["claude-docker-cli"]