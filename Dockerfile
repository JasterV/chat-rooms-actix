FROM rust:1.50.0 as build

WORKDIR /app
COPY . .
RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/actix-messaging /build-out/

# Ubuntu 18.04
FROM ubuntu:18.04 as production

RUN apt-get update && apt-get -y install ca-certificates libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=build /build-out/* /

CMD [ "./actix-messaging" ]