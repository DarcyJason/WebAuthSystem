FROM m.daocloud.io/docker.io/library/rust:1.94.1

EXPOSE 7878

WORKDIR /backend

COPY backend/Cargo.toml backend/Cargo.lock ./
COPY backend/src ./src
COPY backend/migrations ./migrations
COPY backend/.sqlx ./.sqlx
COPY backend/.env .

RUN cargo build --release

CMD ["./target/release/backend"]
