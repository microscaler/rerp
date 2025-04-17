# Base runtime image for RERP microservices
# This image provides the minimal runtime environment for all RERP services
# Services use this as a base and copy their pre-built binaries

ARG TARGETPLATFORM=linux/amd64
ARG BUILDPLATFORM
FROM --platform=${TARGETPLATFORM} alpine:3.19

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    libgcc \
    tzdata

# Create app directory with proper permissions for live updates
WORKDIR /app

# Create standard directories
RUN mkdir -p /app/config /app/doc /app/static_site && \
    chmod -R 777 /app

# Set timezone to UTC
ENV TZ=UTC

# Default environment variables
ENV RUST_BACKTRACE=1
ENV RUST_LOG=info

# Labels for identification
LABEL org.opencontainers.image.title="RERP Base Runtime"
LABEL org.opencontainers.image.description="Base runtime image for RERP microservices"
LABEL org.opencontainers.image.vendor="Microscaler"
