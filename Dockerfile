FROM rust:slim AS builder

RUN apt-get update -y && \
  apt-get install -y python-pip make g++ python-setuptools libssl-dev pkg-config rsync && \
  pip install dockerize && \
  rustup target add x86_64-unknown-linux-gnu

WORKDIR /app
COPY . .

RUN cargo build --release --target x86_64-unknown-linux-gnu --features search/indexing-ja

RUN mv target/x86_64-unknown-linux-gnu/release/zola /usr/bin
RUN mkdir -p /workdir
WORKDIR /workdir
RUN dockerize -n  -o /workdir  /usr/bin/zola


FROM alpine
COPY --from=builder /workdir .
# CMD [ "/usr/bin/zola", "build" ]
# ENTRYPOINT [ "/usr/bin/zola" ]
# ENTRYPOINT [ "/bin/sh" ]
