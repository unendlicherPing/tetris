FROM rust:latest as build

WORKDIR /usr/src/tetris
COPY . .

RUN rustup target add wasm32-unknown-unknown &&\
    cargo install trunk wasm-bindgen-cli &&\
    cd /usr/src/tetris/web &&\
    trunk build --release


FROM nginx:1.23.2-alpine

COPY --from=build /usr/src/tetris/web/dist /usr/share/nginx/html

EXPOSE 80