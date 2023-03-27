FROM ubuntu:latest
LABEL Name=kers_notifier Version=0.0.1
COPY target/release/keras_training_notifier .
CMD [ "ls", "-la"]
RUN chmod +x keras_training_notifier
CMD [ "/keras_training_notifier" ]