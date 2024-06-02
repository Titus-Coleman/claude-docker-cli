# Rust based Generative SQL CLI

A simple docker-based CLI for running Anthropic's Claude LLM. This is built primarily to test SQL code generation based on user input using a provided schema. Will be used to test cargo lambda code for use in an AWS infrastructure.

## Functionality

- Launches Docker image \*only tested on M2 Mac

### TODO:

- Retreives schema from local env path
- Connects to local PostgresDB via env
- Input: natural langage query from user
- Output: raw SQL to be sent to running cargo lambda
- Receives response from cargo lambda

### Step 1: Build Image

<code>docker build --platform=linux/amd64 -t claude-docker-cli .</code>

### Step 2: Run Docker Container

<code>docker run --platform=linux/amd64 claude-docker-cli claude-docker-cli hello</code>
