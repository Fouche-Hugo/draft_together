FROM rust:1.82 as builder

WORKDIR /usr/src/draft_together
COPY ./back .

RUN cargo install --path server

FROM debian:bookworm-slim

COPY --from=builder /usr/local/cargo/bin/draft_together_server /usr/local/bin/draft_together_server
CMD ["draft_together_server"]