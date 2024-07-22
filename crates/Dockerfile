FROM alpine

# avoid running rust compiler on a low ram ec2 instance by having it built in vc
COPY ./release-bin /usr/local/bin/api

ENV UPLOAD_DIR=/usr/src/app/uploads

EXPOSE 3000
ENTRYPOINT api
