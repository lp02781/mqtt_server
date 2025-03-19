FROM rustlang/rust:nightly

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN rustup default nightly  

CMD ["cargo", "run"]
