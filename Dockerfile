# Use a lightweight Alpine Linux base image
FROM alpine:latest

# Install dependencies for C, Python, and Rust
RUN apk add --no-cache \
    build-base \
    python3 \
    python3-dev \
    py3-pip \
    rust

# Set the working directory
WORKDIR /algorithmics

# Copy the current directory contents into the container
COPY . /algorithmics
