FROM rust:1.61.0-alpine

WORKDIR /usr/src/microserver
COPY . .

RUN cargo install --path .

CMD ["microserver"]