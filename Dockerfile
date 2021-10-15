FROM ekidd/rust-musl-builder AS build
WORKDIR /usr/src/
USER root

# set rust build target
ENV RUST_TARGETS="x86_64-unknown-linux-musl"
RUN rustup target install x86_64-unknown-linux-musl

# create dummy project
RUN USER=root cargo new actix-app
WORKDIR /usr/src/actix-app

# moving deps info
COPY Cargo.lock Cargo.toml ./

# Caching deps
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN rm -rf target/x86_64-unknown-linux-musl/release/deps/actix_app*

# Replacing with actual src
RUN rm src/*.rs
COPY src ./src

# Only code changes should need to compile
RUN cargo build --target x86_64-unknown-linux-musl --release

# last build stage
FROM scratch
COPY --from=build /usr/src/actix-app/target/x86_64-unknown-linux-musl/release/actix-app .
USER 1000
CMD ["./actix-app"]