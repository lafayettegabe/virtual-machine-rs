FROM rust:latest AS build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:buster-slim

COPY --from=build /app/target/release/vm /usr/local/bin/

CMD ["vm"]