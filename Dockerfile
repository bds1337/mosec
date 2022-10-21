# Компиляция для стабильного дебиана
#FROM debian:bullseye
FROM rust:1-bullseye as builder
SHELL ["/bin/bash", "-c"]
# ENV DEBIAN_FRONTEND noninteractive
#RUN apt update -y
#RUN apt install -y \
#    build-essential \
#    curl

#RUN curl https://sh.rustup.rs -sSf | bash -s -- -y

COPY . .
# RUN cargo build --release
# RUN cargo build --release

FROM scratch
RUN "ls -la ."
#COPY --from=builder ./target/release/mosec /home/bds/mosec
COPY --from=builder README.md /home/bds
