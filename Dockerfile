FROM alpine:latest AS builder

WORKDIR /app

RUN apk update
RUN apk upgrade
RUN apk add cargo

COPY src ./src/
COPY Cargo.toml .

RUN ls -laR
RUN cargo build --release

FROM alpine:latest

WORKDIR /app

RUN apk update
RUN apk upgrade
RUN apk add ca-certificates libgcc

COPY --from=builder /app/target/release/spacestate_irc_bot .

CMD ["./spacestate_irc_bot"]

