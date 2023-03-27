## Keras Training Notifier
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/Flippchen/Keras-Training-Notifier/pipeline.yaml?style=flat-square) ![Docker Pulls](https://img.shields.io/docker/pulls/philbooyy/keras_training_notifier?style=flat-square) ![Docker Image Size (latest by date)](https://img.shields.io/docker/image-size/philbooyy/keras_training_notifier?style=flat-square)

Connect your Keras training to Discord and get notified after every epoch.

## Installation
You can use docker-compose to run the notifier. You can also build the docker image yourself.
For the docker-compose, you need to edit the `variables.env` file the following content:

| variable        | content                                                                                                                 |
|-----------------|-------------------------------------------------------------------------------------------------------------------------|
| DISCORD_WEBHOOK | The webhook url of your discord channel                                                                                 |
| PORT            | The port the Bot will be listening. If you want to change this, you also need to change it in the `docker-compose.yaml` |

After that, you can run `docker-compose up -d`.

## Building from scratch
If you want to build the docker image yourself, you first need to build the project:
```bash
cargo build --release
```
Then you can build the docker image:
```bash
docker build . -t keras-training-notifier:latest
```
After that, you can run the docker image:
```bash
docker run -d -p PORT:PORT --env-file variables.env keras-training-notifier:latest
```
## Usage
To use the notifier, you need to add the following code to your Keras training:
```python
import tensorflow as tf
#...
server_callback = tf.keras.callbacks.RemoteMonitor(
    root="http://localhost:9000",
    path="",
    field="data",
    headers=None,
    send_as_json=True,
)
#...
model.fit(x, y, epochs=5, batch_size=32, validation_data=(x_test, y_test), callbacks=[server_callback])
```
To get a full sample have a look at the [keras.py](example_keras/keras.py) file.