FROM lukemathwalker/cargo-chef:latest-rust-1.65.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

# Planner
FROM chef as planner
COPY . .
# Compute a lock-like file for our project
RUN cargo chef prepare --recipe-path recipe.json

# Builder
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin event-bus 

# Runtime
FROM debian:bullseye-slim AS runtime
WORKDIR /app
RUN apt-get update -y \
	&& apt-get install -y --no-install-recommends openssl ca-certificates \
	&& apt-get autoremove -y \
	&& apt-get clean -y \
	&& rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/event-bus event-bus 
ENV APP_ENVIRONMENT production
ENTRYPOINT [ "./event-bus" ]