# Stage 1: Build stage using rust:alpine
FROM rust:alpine AS build

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

# Set working directory
WORKDIR /build

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY migrations ./migrations
COPY .sqlx ./.sqlx

# Build the project with release optimizations
# Use musl target for static linking
RUN cargo build --release --bin uninews

# Stage 2: Runtime stage with minimal Alpine image
FROM alpine:latest

# Install runtime dependencies (minimal)
RUN apk add --no-cache \
    ca-certificates \
    tzdata \
    && addgroup -g 1000 uninews \
    && adduser -D -u 1000 -G uninews uninews

# Set working directory
WORKDIR /app

# Copy the binary from build stage
COPY --from=build /build/target/release/uninews /usr/local/bin/uninews

# Create data directory for SQLite database
RUN mkdir -p /app/data && chown -R uninews:uninews /app

# Switch to non-root user for security
USER uninews

# Set environment variables
ENV RUST_LOG=info
ENV DATABASE_URL=sqlite:///app/data/app.sqlite

# Expose volume for persistent data
VOLUME ["/app/data"]

# Run the binary in collect mode
CMD ["uninews", "collect"]
