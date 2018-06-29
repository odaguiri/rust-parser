# set image
FROM rust:1.23.0

# config container dir
ENV APP /usr/src/app
ENV USER $(whoami)
WORKDIR $APP

# enable container dir
ADD . $APP
