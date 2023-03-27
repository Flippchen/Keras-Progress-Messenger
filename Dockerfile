FROM ubuntu:latest
LABEL Name=keras_notifier Version=0.0.1
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*
COPY target/release/keras_training_notifier .
CMD [ "ls", "-la"]
RUN chmod +x keras_training_notifier
CMD [ "/keras_training_notifier" ]