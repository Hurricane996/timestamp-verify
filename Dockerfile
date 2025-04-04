FROM rust:1.84

ARG SECRET_KEY
RUN test -n "$SECRET_KEY" || (echo "No secret key provided. make sure to run docker build with flag --build-arg SECRET_KEY=<secret>"; false) 

ENV SECRET_KEY=${SECRET_KEY}

WORKDIR /usr/src/timestamp-verify

COPY . .

RUN cargo install --path .

CMD ["server"]

EXPOSE 3000