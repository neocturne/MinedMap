FROM docker.io/library/rust:alpine AS BUILDER

WORKDIR /build
RUN apk update && apk add cmake build-base

COPY src /build/src
COPY crates /build/crates
COPY Cargo.toml Cargo.lock /build
RUN cargo build -r 

FROM scratch AS RUNNER

COPY --from=BUILDER /build/target/release/minedmap /minedmap
ENTRYPOINT [ "/minedmap" ]
