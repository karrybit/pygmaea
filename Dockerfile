FROM rust:stretch

LABEL maintainer "Takumi Karibe <takumi.k.5610@gmail.com>"

RUN apt-get -y update
RUN apt-get -y install \
    lcov ruby lldb
RUN apt-get clean

RUN gem install coveralls-lcov

RUN rustup component add rustfmt clippy

RUN rustup install nightly
RUN cargo +nightly install grcov
