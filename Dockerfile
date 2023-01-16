# 1. Get latest slim rust image
FROM rust:1.63-slim-buster as build

# 2. Create new empty shell project
RUN USER=root cargo new --bin service_lowmaf_rust
WORKDIR /service_lowmaf_rust

# 3. Copy manifests over
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# 4. Cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# 5. Copy source files
COPY ./src ./src

# 6. Build for release
RUN rm ./target/release/deps/service_lowmaf_rust*
RUN cargo build --release

# 7. Final base
FROM rust:1.63-slim-buster

# 8. Copy build artifacts from build stage
COPY --from=build /service_lowmaf_rust/target/release/service_lowmaf_rust .

# 9. Set startup command to run binary
CMD ["./service_lowmaf_rust"]
