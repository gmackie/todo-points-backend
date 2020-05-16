# build stage
FROM rust:slim as build

# install libpq and create new empty binary project
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*; \
    USER=root cargo new --bin app
WORKDIR /app

# copy manifests
COPY ./Cargo.toml ./Cargo.toml

# build this project to cache dependencies
RUN cargo build --release; \
    rm src/*.rs

# copy project source and necessary files
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .

# add .env and secret.key for Docker env
RUN touch .env; \
    mv src/secret.key.sample src/secret.key

# rebuild app with project source
RUN rm ./target/release/deps/todo_points_backend*; \
    cargo build --release

# deploy stage
FROM debian:buster-slim

# create app directory
RUN mkdir app
WORKDIR /app

# install libpq
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /app/target/release/todo-points-backend .
COPY --from=build /app/.env .
COPY --from=build /app/diesel.toml .

# expose port
EXPOSE 8000

# run the binary
ENTRYPOINT ["/app/todo-points-backend"]
