FROM rust:1.76-bookworm as wasm
WORKDIR /src
RUN rustup target add wasm32-unknown-unknown
RUN cargo install wasm-bindgen-cli
COPY ./rust_core ./
RUN cargo build --release --target wasm32-unknown-unknown
RUN wasm-bindgen ./target/wasm32-unknown-unknown/release/rust_core.wasm --out-dir ./dist --target no-modules

FROM node:21.6-bookworm as build
WORKDIR /app
COPY ./package*.json ./
RUN npm install
COPY . .
COPY --from=wasm /src/dist ./static/wasm
RUN npm run build

FROM nginx:1.25.3

COPY ./nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=build /app/build /usr/share/nginx/html