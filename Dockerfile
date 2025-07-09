# Stage 1: Build the frontend
FROM node:24-alpine AS frontend-builder

WORKDIR /app/web/netdrop

# Copy package files
COPY web/netdrop/package*.json ./

# Install dependencies
RUN npm ci

# Copy frontend source code
COPY web/netdrop/ ./

# Build the frontend
RUN npm run build

# Stage 2: Build the Rust program
FROM rust:1.88-alpine AS rust-builder

# Install build dependencies including diesel CLI
RUN apk add --no-cache musl-dev sqlite-dev sqlite-static
RUN cargo install diesel_cli --no-default-features --features sqlite

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./

# Copy source code
COPY src/ ./src/
COPY diesel.toml ./
COPY migrations/ ./migrations/

# Copy built frontend from previous stage
COPY --from=frontend-builder /app/web/netdrop/dist ./web/netdrop/dist

# Build the Rust application in release mode
RUN cargo build --release

# Create data directory and initialize database
RUN mkdir -p /app/data
ENV DATABASE_URL=/app/data/netdrop.db
RUN diesel migration run

# Stage 3: Final runtime image
FROM alpine:latest

# Install runtime dependencies
RUN apk add --no-cache sqlite

# Create app directory
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=rust-builder /app/target/release/netdrop /app/

# Copy the initialized database from the builder stage
COPY --from=rust-builder /app/data /app/data

# Set environment variables
ENV DATA_DIR=/app/data
ENV DATABASE_URL=/app/data/netdrop.db
ENV ROCKET_ADDRESS=0.0.0.0

# Expose the port
EXPOSE 8000

# Run the application
CMD ["./netdrop"]
