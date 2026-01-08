FROM rust:latest

WORKDIR /usr/src/budget-tracker
COPY ./.env ./.env
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
COPY ./build.rs ./build.rs
COPY ./.sqlx ./.sqlx
COPY ./migrations ./migrations

# ENV DATABASE_URL "postgres://postgres:password@db:5432/budget-tracker"
#
# RUN cargo install sqlx-cli
# RUN cargo sqlx prepare

RUN cargo install --path .

CMD ["budget-tracker"]
