# Use a pre-configured image specifically for Rust with musl
FROM clux/muslrust:1.83.0-stable AS planner
WORKDIR /app
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM clux/muslrust:1.83.0-stable AS builder
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json --no-default-features --features cloud

COPY . .
RUN cargo build \
    --no-default-features \
    --features cloud \
    --release \
    --bin api \
    --target x86_64-unknown-linux-musl \
    && strip target/x86_64-unknown-linux-musl/release/api

# Create an AWS Lambda compatible image
FROM public.ecr.aws/lambda/provided:al2
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/api /api
ENV AWS_LAMBDA_HTTP_IGNORE_STAGE_IN_PATH=true
ENTRYPOINT ["/api"]