FROM docker.io/library/rust:1.91.0-alpine AS builder

WORKDIR /build
RUN apk add --no-cache build-base tini-static

ARG MINEDMAP_VERSION

COPY . .
RUN cargo build -r
RUN strip target/release/minedmap

FROM scratch

COPY --from=builder /sbin/tini-static /build/target/release/minedmap /bin/
ENTRYPOINT [ "/bin/tini-static", "--", "/bin/minedmap" ]

USER 1000:1000
