VERSION 0.6
FROM purtontech/rust-on-nails-devcontainer:1.1.8

# Exe names
ARG APP_NAME=trace
ARG APP_EXE_NAME=trace
ARG FEED_EXE_NAME=space-track-feed

# Folders
ARG AXUM_FOLDER=crates/axum-server
ARG DB_FOLDER=crates/db
ARG GRPC_API_FOLDER=crates/grpc-api
ARG PIPELINE_FOLDER=crates/asset-pipeline

# Base images
ARG DBMATE_VERSION=1.15.0
ARG ENVOY_PROXY=envoyproxy/envoy:v1.21.2

# To test an image
# If you want to test the pushed image -> docker pull purtontech/trace-server:latest
# docker run --rm -it --env APP_DATABASE_URL=$APP_DATABASE_URL purtontech/trace-server
# docker run -it --entrypoint /bin/bash purtontech/trace-server
ARG APP_IMAGE_NAME=purton-tech/skytrace:latest
ARG MIGRATIONS_IMAGE_NAME=purton-tech/skytrace-migrations:latest
ARG ENVOY_IMAGE_NAME=purton-tech/skytrace-envoy:latest
ARG FEED_IMAGE_NAME=purton-tech/skytrace-space-track-feed:latest

WORKDIR /build

USER vscode

all:
    BUILD +migration-container
    BUILD +envoy-container
    BUILD +app-container
    BUILD +feed-container

npm-deps:
    COPY $PIPELINE_FOLDER/package.json $PIPELINE_FOLDER/package.json
    COPY $PIPELINE_FOLDER/package-lock.json $PIPELINE_FOLDER/package-lock.json
    RUN cd $PIPELINE_FOLDER && npm install
    SAVE ARTIFACT $PIPELINE_FOLDER/node_modules

npm-build:
    FROM +npm-deps
    COPY $PIPELINE_FOLDER $PIPELINE_FOLDER
    COPY --if-exists $GRPC_API_FOLDER $GRPC_API_FOLDER
    COPY +npm-deps/node_modules $PIPELINE_FOLDER/node_modules
    RUN cd $PIPELINE_FOLDER && npm run release
    SAVE ARTIFACT $PIPELINE_FOLDER/dist

prepare-cache:
    # Copy in all our crates
    COPY --dir crates crates
    COPY Cargo.lock Cargo.toml .
    RUN cargo chef prepare --recipe-path recipe.json
    SAVE ARTIFACT recipe.json

build-cache:
    COPY +prepare-cache/recipe.json ./
    RUN cargo chef cook --release --target x86_64-unknown-linux-musl
    SAVE ARTIFACT target
    SAVE ARTIFACT $CARGO_HOME cargo_home
    SAVE IMAGE --cache-hint

build:
    # Copy in all our crates
    COPY --dir crates crates
    COPY --dir Cargo.lock Cargo.toml .
    COPY +build-cache/cargo_home $CARGO_HOME
    COPY +build-cache/target target
    COPY --dir +npm-build/dist $PIPELINE_FOLDER/
    # We need to run inside docker as we need postgres running for cornucopia
    ARG DATABASE_URL=postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable
    USER root
    WITH DOCKER \
        --pull postgres:alpine
        RUN docker run -d --rm --network=host -e POSTGRES_PASSWORD=testpassword postgres:alpine \
            && while ! pg_isready --host=localhost --port=5432 --username=postgres; do sleep 1; done ;\
                dbmate --migrations-dir $DB_FOLDER/migrations up \
            && cargo build --release --target x86_64-unknown-linux-musl --bin $APP_EXE_NAME \
            && cargo build --release --target x86_64-unknown-linux-musl --bin $FEED_EXE_NAME
    END
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$APP_EXE_NAME
    SAVE ARTIFACT target/x86_64-unknown-linux-musl/release/$FEED_EXE_NAME


envoy-container:
    FROM $ENVOY_PROXY
    COPY .devcontainer/envoy.yaml /etc/envoy/envoy.yaml
    # Point the www route to the cloudflare pages entry
    RUN sed -i '0,/www/{s/www/www-prod/}' /etc/envoy/envoy.yaml
    # The second development entry in our cluster list is the app
    # https://www.linuxquestions.org/questions/programming-9/replace-2nd-occurrence-of-a-string-in-a-file-sed-or-awk-800171/
    RUN sed -i '0,/development/! s/development/app/' /etc/envoy/envoy.yaml
    SAVE IMAGE --push $ENVOY_IMAGE_NAME

# This is our migrations sidecar
migration-container:
    FROM alpine
    RUN apk add --no-cache \
        curl \
        postgresql-client \
        tzdata
    RUN curl -OL https://github.com/amacneil/dbmate/releases/download/v$DBMATE_VERSION/dbmate-linux-amd64 \
        && mv ./dbmate-linux-amd64 /usr/bin/dbmate \
        && chmod +x /usr/bin/dbmate
    COPY --dir $DB_FOLDER .
    CMD dbmate up
    SAVE IMAGE --push $MIGRATIONS_IMAGE_NAME

# To test this locally run
# docker run -it --rm -e APP_DATABASE_URL=$APP_DATABASE_URL -p 7403:7403 purtontech/trace-server:latest
app-container:
    FROM scratch
    COPY +build/$APP_EXE_NAME axum-server
    # Place assets in a build folder as that's where statics is expecting them.
    COPY --dir +npm-build/dist /build/$PIPELINE_FOLDER/
    COPY --dir $PIPELINE_FOLDER/images /build/$PIPELINE_FOLDER/images
    ENTRYPOINT ["./axum-server"]
    SAVE IMAGE --push $APP_IMAGE_NAME

feed-container:
    FROM scratch
    COPY +build/$FEED_EXE_NAME space-track-feed
    ENTRYPOINT ["./space-track-feed"]
    SAVE IMAGE --push $FEED_IMAGE_NAME