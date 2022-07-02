FROM rust:1.58.1-buster as builder

WORKDIR /app
COPY ./ ./

RUN cargo build --release

FROM ubuntu:latest
RUN apt update && apt install git -y
WORKDIR /opt/dagger
COPY --from=builder /app/target/release/dagger ./
COPY --chmod=+x ./run.sh /usr/bin/dagger
COPY ./tmpl ./tmpl

CMD ["./dagger"]

