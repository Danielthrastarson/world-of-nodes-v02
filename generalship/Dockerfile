FROM rust:1.98-bookworm AS build
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY generalship ./generalship
RUN cargo build --locked --release --manifest-path generalship/Cargo.toml --bin generalship-server
FROM debian:bookworm-slim
WORKDIR /app
COPY --from=build /app/generalship/target/release/generalship-server /app/server
ENV G0_BIND=0.0.0.0 PORT=8080 G0_DATA_DIR=/tmp/generalship-replays
USER 65534:65534
EXPOSE 8080
CMD ["/app/server"]
