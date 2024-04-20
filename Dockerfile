ARG TARGETPLATFORM=${TARGETPLATFORM}
ARG BUILDPLATFORM=${BUILDPLATFORM}

FROM rust:1.77.2-bookworm AS build
ENV CARGO_HOME=/var/cache/cargo
WORKDIR /app/
WORKDIR /src/chorus/
COPY . ./
RUN \
  --mount=type=cache,id=cargo-cache-${TARGETPLATFORM},target=/var/cache/cargo,sharing=locked \
  --mount=type=cache,id=build-cache-${TARGETPLATFORM},target=/src/chorus/target,sharing=locked \
  cargo build --release \
  && find /src/chorus/target/release/ -type f -executable -not -path '*build*' -not -path '*deps*' -print0 | xargs -0 mv -t /app

FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /data/
WORKDIR /config/
WORKDIR /app/
COPY --from=build --chown=nonroot:nonroot /app ./
COPY ./sample/docker.config.toml /config/config.toml
VOLUME ["/config", "/data"]
EXPOSE 8080
ENTRYPOINT ["/app/chorus"]
CMD ["/config/config.toml"]