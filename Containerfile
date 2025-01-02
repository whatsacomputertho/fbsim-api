# Build the API binary
FROM docker.io/rust:1-slim-bookworm AS build
ARG pkg=fbsim-api
WORKDIR /build
COPY . .
RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/$pkg ./main

# Build the API image
FROM docker.io/debian:bookworm-slim
WORKDIR /app
COPY --from=build /build/main ./
EXPOSE 8080
ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8080
CMD ./main