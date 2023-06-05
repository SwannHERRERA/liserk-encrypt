FROM rust:1.70 as builder

WORKDIR /usr/src/

COPY ./Cargo.toml ./Cargo.toml

COPY ./certificate_authority ./certificate_authority
COPY ./shared ./shared

RUN cargo build --release --workspace

# --------------

FROM debian:buster-slim

WORKDIR /usr/src/

COPY --from=builder /usr/src/certificate_authority/target/release/certificate_authority .

RUN mkdir /usr/src/certificates

EXPOSE 80

VOLUME [ "/usr/src/certificates" ]

CMD ["./certificate_authority"]
