ARG BASE_CONTAINER=python:3.8
FROM --platform=linux/amd64 $BASE_CONTAINER

USER root
RUN apt update && \
    apt install --no-install-recommends -y build-essential gcc && \
    apt clean && rm -rf /var/lib/apt/lists/*

WORKDIR /consumer
COPY ./app ./app
WORKDIR ./app

RUN pip install --upgrade pip && pip install -r requirements.txt
CMD ["python", "kafka_producer.py"]
