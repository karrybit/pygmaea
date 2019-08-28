FROM rust:latest

LABEL maintainer "Takumi Karibe <takumi.k.5610@gmail.com>"

RUN apt-get -y update
RUN apt-get -y install \
    lcov ruby lldb
RUN apt-get clean

RUN gem install coveralls-lcov

RUN rustup update
RUN rustup self update
RUN rustup install stable
RUN rustup install nightly
RUN rustup component add rustfmt
RUN cargo install grcov

RUN rustup default stable
