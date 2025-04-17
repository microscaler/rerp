# Multi-stage build for RERP website
# Stage 1: Build the SolidJS application
FROM --platform=linux/amd64 node:20-alpine AS builder

# Enable corepack for yarn support
# Use Yarn v1 to match the lockfile format
RUN corepack enable && corepack prepare yarn@1.22.22 --activate

WORKDIR /build

# Copy package files first for better layer caching
COPY ui/website/package.json ui/website/yarn.lock* ./

# Install dependencies using yarn
RUN yarn install --frozen-lockfile

# Copy shared components to parent directory (matches path resolution)
# This allows @shared alias to resolve correctly
COPY ui/shared/ ../shared/

# Copy source files
COPY ui/website/ ./

# Build the application
RUN yarn build

# Stage 2: Serve with nginx
FROM --platform=linux/amd64 nginx:alpine

# Copy built application from builder stage
COPY --from=builder /build/dist /usr/share/nginx/html

# Copy custom nginx configuration
COPY docker/website/nginx.conf /etc/nginx/conf.d/default.conf

# Expose port
EXPOSE 8080

CMD ["nginx", "-g", "daemon off;"]
