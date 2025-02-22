FROM docker.io/library/alpine:latest AS builder

WORKDIR /build
RUN apk add --no-cache build-base cmake cargo

COPY . .
RUN cargo build -r 
RUN strip target/release/minedmap

FROM docker.io/library/alpine:latest

RUN addgroup -g 1000 -S minedmap \
    && adduser -S -D -H -u 1000 -h /output -s /sbin/nologin -G minedmap -g minedmap minedmap

RUN apk add --no-cache libgcc tini

COPY --from=builder /build/target/release/minedmap /bin/minedmap
ENTRYPOINT [ "/sbin/tini", "--", "/bin/minedmap" ]

USER minedmap:minedmap
