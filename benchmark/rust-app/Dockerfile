FROM rust:1.76.0-slim-bookworm as build
WORKDIR /app
COPY . .
RUN : \
  && apt-get update \
  && DEBIAN_FRONTEND=noninteractive \
  apt-get install -y -q --no-install-recommends \
    build-essential \
    cmake \
    pkg-config \
    libssl-dev
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=build /app/target/release/rust-app /
CMD ["/rust-app"]
