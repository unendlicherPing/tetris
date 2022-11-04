FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown &&\
    cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/tetris
COPY . .

RUN trunk build --release


FROM nginx:1.23.2-alpine

COPY --from=build /usr/src/tetris/dist /usr/share/nginx/html

EXPOSE 80