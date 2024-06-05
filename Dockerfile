FROM rustlang/rust:nightly
LABEL authors="defalt"
COPY . /app
WORKDIR /app
RUN #cargo install cargo-watch

ENTRYPOINT ["cargo","run"]