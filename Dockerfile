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
	ca-certificates \
  apt-transport-https \
	--no-install-recommends && \
	curl -sSL https://dl.google.com/linux/linux_signing_key.pub | apt-key add - && \
  echo "deb http://dl.google.com/linux/chrome/deb/ stable main" > /etc/apt/sources.list.d/google.list && \
  apt-get update && \
  apt --fix-broken install && \
  apt-get install -y \
	google-chrome-stable \
	fontconfig \
  fonts-noto \
  fonts-kacst \
  fonts-symbola \
  fonts-thai-tlwg \
  fonts-wqy-zenhei \
  fonts-freefont-ttf \
	fonts-ipafont-gothic \
	--no-install-recommends && \
  rm -rf /var/lib/apt/lists/*

COPY --from=build /send2kindle/target/release/send2kindle .

# set the startup command
ENV PORT 3310
EXPOSE 3310 

CMD ["./send2kindle"]
