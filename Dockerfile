FROM rust:latest as build

RUN rustup target add wasm32-unknown-unknown &&\
    cargo install trunk wasm-bindgen-cli

WORKDIR /usr/src/tetris
COPY . .

CMD [ "trunk", "build", "--release" ]


FROM nginx:stable
COPY --from=build /usr/src/tetris/dist /usr/share/nginx/html
