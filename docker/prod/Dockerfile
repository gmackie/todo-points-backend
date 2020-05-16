# base image
FROM debian:buster-slim

# create app directory
RUN mkdir app
WORKDIR /app

# install libpq
RUN apt-get update; \
    apt-get install --no-install-recommends -y libpq-dev; \
    rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY ./todo-points-backend .
COPY ./diesel.toml .
COPY ./.env .

# expose port
EXPOSE 8000

# run the binary
ENTRYPOINT ["/app/todo-points-backend"]
