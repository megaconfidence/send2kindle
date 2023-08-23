FROM rust:1.67 as build

# create a new empty shell project
RUN USER=root cargo new --bin send2kindle
WORKDIR /send2kindle

# copy over manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/send2kindle*
RUN cargo build --release

# final base
FROM yukinying/chrome-headless-browser-stable
# copy the build artifact from build stage
COPY --from=build /send2kindle/target/release/send2kindle .

# set the startup command
ENV PORT 80
EXPOSE 80 
CMD ["./send2kindle"]
