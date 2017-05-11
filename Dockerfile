FROM scratch

ENV ROCKET_ENV production

EXPOSE 8000

COPY ./target/x86_64-unknown-linux-musl/release/hello-rocket /hello-rocket
COPY ./Rocket.toml /Rocket.toml

COPY ./resources /resources
COPY ./templates /templates

CMD ["/hello-rocket"]