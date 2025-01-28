# https://github.com/katopz/hello-rust-actix-cloudrun/blob/main/Dockerfile
# Start with a rust alpine image
FROM rust:1-alpine3.20 as builder
# if needed, add additional dependencies here
RUN apk add --no-cache musl-dev
# set the workdir and copy the source into it
WORKDIR /app

# Copy over the Cargo.toml files to the shell project
COPY Cargo.toml Cargo.lock ./

# Build and cache the dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo fetch
RUN cargo build --release
RUN rm src/main.rs

# Copy the actual code files and build the application
COPY src ./src/
COPY data ./data/
COPY templates ./templates/
# Update the file date
RUN touch src/main.rs
# do a release build
RUN cargo build --release

# use a plain alpine image, the alpine version needs to match the builder
FROM alpine:3.20 as runner
# if needed, install additional dependencies here
RUN apk add --no-cache libgcc
# copy the binary into the final image
COPY --from=0 /app/target/release/verron .

RUN mkdir ./data
RUN mkdir ./templates
COPY --from=0 /app/data/resume.json ./data/resume.json
COPY --from=0 /app/templates ./templates
# set the binary as entrypoint
ENTRYPOINT ["/verron"]