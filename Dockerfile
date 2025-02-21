FROM docker.io/library/alpine:latest AS builder

WORKDIR /build
RUN apk add --no-cache build-base cmake cargo

COPY . .
RUN cargo build -r 
RUN strip target/release/minedmap

FROM docker.io/library/alpine:latest

RUN apk add --no-cache libgcc tini

COPY --from=builder /build/target/release/minedmap /bin/minedmap
ENTRYPOINT [ "/sbin/tini", "--", "/bin/minedmap" ]
