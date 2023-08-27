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
FROM debian:bullseye-slim

# install utils. libss1.1 is required for libssl.so.1.1
RUN apt-get update && apt-get install -y libssl1.1 curl procps 

# install google-chrome
RUN apt-get update && apt-get install -y \
  gnupg \
  chromium \
  fontconfig \
  fonts-noto \
  fonts-kacst \
  fonts-symbola \
  ttf-wqy-zenhei \
  fonts-thai-tlwg \
  ca-certificates \
  fonts-wqy-zenhei \
  fonts-freefont-ttf \
  apt-transport-https \
	fonts-ipafont-gothic \
	--no-install-recommends && \
  rm -rf /var/lib/apt/lists/*


# symlink google-chrome to chromium for arm64 compatibility
RUN ln -s /usr/bin/chromium /usr/bin/google-chrome-stable
RUN ln -s /usr/bin/chromium /usr/bin/google-chrome

COPY --from=build /send2kindle/target/release/send2kindle .

# set the startup command
ENV PORT 3310
EXPOSE 3310 

CMD ["./send2kindle"]
