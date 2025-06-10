# --- Stage 1: Build statically with MUSL
FROM rust:1.87 AS builder

# Install musl tools and add target
RUN apt-get update && \
    apt-get install -y musl-tools && \
    rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/bypass
COPY Cargo.toml ./
COPY src ./src

# Build for the MUSL target (static binary)
RUN cargo build --release --target x86_64-unknown-linux-musl

# --- Stage 2: Minimal runtime
FROM scratch AS prod

# Copy the static binary
COPY --from=builder /usr/src/bypass/target/x86_64-unknown-linux-musl/release/bypass /usr/local/bin/bypass

# Default mount point
ENTRYPOINT ["/usr/local/bin/bypass"]
CMD ["node_modules"]