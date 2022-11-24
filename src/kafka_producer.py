from kafka import KafkaProducer

def main():
    producer = KafkaProducer(bootstrap_servers='kafka:9092')
    for _ in range(100):
        producer.send('test', b'some_message_bytes')

if __name__ == "__main__":
    main()
