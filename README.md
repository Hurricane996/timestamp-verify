# Timestamp Verifier
A web app that allows users to create a verifiable timestamp using HMAC codes. Built for the UNDERTALE speedrunning community to allow for verification of runs with multiple video segments.

# Building and running
## Docker
To run this with docker, first build the container with `docker build --tag "timestamp_verify" . --build-arg SECRET_KEY="<your secret key>"`, then run it with `docker run -d -p <port>:3000`. This will expose the web interface on whichever port 

## Local build
To build this locally, run `SECRET_KEY=<your secret key> cargo run`. This will expose the web interface at localhost:3000.

# Secret key
This app uses HMAC codes which rely on a "secret key". Anyone with knowledge of the secret key can trivially falsify timestamps. In order for the verification to be work the secret key needs to be known only by trusted individuals. As such, the secret key should be long and random, and knowledge of it should be kept to one or a few highly trusted community members.
