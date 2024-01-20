FROM debian:bookworm-slim

RUN mkdir -p /tempfiles

COPY ./tempfiles-rs /tempfiles/tempfiles-rs
COPY ./assets /tempfiles/assets
COPY ./templates /tempfiles/templates

WORKDIR /tempfiles

CMD [ "/tempfiles/tempfiles-rs" ]
