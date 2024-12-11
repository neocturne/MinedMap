FROM docker.io/library/rust:alpine AS BUILDER

WORKDIR /src
RUN apk update && apk add cmake build-base

COPY src /src/src
COPY crates /src/crates
COPY Cargo.toml Cargo.lock /src
RUN cargo build -r 

FROM scratch AS RUNNER

COPY --from=BUILDER /src/target/release/minedmap /minedmap
ENTRYPOINT [ "/minedmap" ]
