FROM rust:1.71.1 as build-env
WORKDIR /app
COPY . /app
RUN cargo build --release

FROM gcr.io/distroless/cc
WORKDIR /app
COPY --from=build-env /app/target/release/thunderfury ./
VOLUME ["/app/config", "/media/library"]
CMD ["./thunderfury", "server"]
