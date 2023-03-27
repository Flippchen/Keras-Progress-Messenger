FROM alpine:latest
LABEL Name=kers_notifier Version=0.0.1
COPY target/x86_64-unknown-linux-musl/release/keras_notifier .
WORKDIR /data
VOLUME [ "/data" ]
CMD [ "/keras_notifier" ]