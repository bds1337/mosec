# Компиляция для стабильного дебиана
FROM debian:bullseye
ENV DEBIAN_FRONTEND noninteractive
RUN apt update -y
RUN apt install -y \
    build-essential

COPY . .
# RUN cargo build --release
