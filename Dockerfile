FROM rust:1.64.0-alpine

WORKDIR /usr/src/tetris
COPY . .

RUN cargo install trunk
RUN cargo install --path .

EXPOSE 8000

CMD [ "trunk", "serve", "--release" ]
