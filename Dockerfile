FROM alpine:latest
LABEL Name=kers_notifier Version=0.0.1
COPY target/release/keras_training_notifier .
RUN chmod +x keras_training_notifier
CMD [ "/keras_training_notifier" ]