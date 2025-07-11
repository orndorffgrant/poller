FROM denoland/deno:alpine AS deno-builder
COPY assets-src assets-src
WORKDIR /assets-src
RUN mkdir /assets && deno run --allow-net --allow-write vendor-assets.js

FROM node:24-alpine AS node-builder
COPY . .
WORKDIR /assets-src
RUN npm install
RUN mkdir /assets && npm run build-css

FROM rust:1-alpine AS rust-builder
RUN apk add --no-cache musl-dev sqlite-dev
COPY . .
COPY --from=deno-builder /assets /assets
COPY --from=node-builder /assets/styles.css /assets/styles.css
RUN cargo build --release

FROM alpine:3.22
RUN apk add --no-cache sqlite
RUN addgroup -g 1000 poller && \
    adduser -D -s /bin/sh -u 1000 -G poller poller
COPY --from=rust-builder /target/release/poller /usr/local/bin/poller
RUN mkdir -p /app/data && chown -R poller:poller /app
USER poller
EXPOSE 8000
CMD ["poller", "run", "/app/data/data.db"]
