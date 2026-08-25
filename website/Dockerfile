FROM rust:1-alpine3.23 AS build

ARG pkg=website

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY . .

RUN set -eux; \
    cargo build --release; \
    objcopy --compress-debug-sections target/release/$pkg ./main

FROM alpine:3.23

WORKDIR /app

RUN chmod 777 /app

COPY --from=build /build/main ./main

COPY --from=build /build/static ./static/
COPY --from=build /build/Rocket.toml .
COPY --from=build /build/templates ./templates/

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8003

RUN ls -laR

EXPOSE 8003

CMD ["/app/main"]
