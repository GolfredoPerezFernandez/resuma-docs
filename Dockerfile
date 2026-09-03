# Resuma docs site — build from this repo root: docker build -t resuma-docs .

FROM node:22-bookworm AS client
WORKDIR /app
COPY package.json package-lock.json tsconfig.json ./
COPY client ./client
RUN npm ci 2>/dev/null || npm install
RUN npm run build:client

FROM rust:1-bookworm AS builder
WORKDIR /app

COPY Cargo.toml rust-toolchain.toml ./
COPY src ./src
COPY --from=client /app/static/client ./static/client

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home resuma

WORKDIR /app
COPY --from=builder /app/target/release/website /app/website
COPY --from=builder /app/src/pages /app/pages
COPY public ./public
RUN chown -R resuma:resuma /app

USER resuma

ENV HOST=0.0.0.0
ENV PORT=3000
ENV RESUMA_PAGES_ROOT=/app/pages
ENV CARGO_MANIFEST_DIR=/app
ENV RESUMA_ENV=production
ENV RESUMA_TRUST_PROXY=1

EXPOSE 3000
CMD ["/app/website"]
