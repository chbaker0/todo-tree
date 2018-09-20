FROM ubuntu:18.04
MAINTAINER Collin Baker <chbaker0@gmail.com>

EXPOSE 8080

ENV SOURCES=/sources

RUN apt-get -y update && apt-get -y install file gcc libssl-dev curl
RUN curl -sSf https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly --disable-sudo

RUN mkdir -p $SOURCES
ADD ./ $SOURCES

WORKDIR $SOURCES
RUN cargo build --release

CMD ROCKET_ENV=production ROCKET_PORT=8080 ./target/release/todo-tree
