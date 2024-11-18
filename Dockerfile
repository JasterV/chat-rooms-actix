FROM rust:1.78.0 as build

WORKDIR /app
COPY . .
RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/actix-messaging /build-out/

# Ubuntu 24.10
FROM ubuntu:24.10 as production

RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=build /build-out/actix-messaging /usr/local/bin/actix-messaging

ENV PORT=8080

ENTRYPOINT [ "/usr/local/bin/actix-messaging" ]