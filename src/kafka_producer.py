from kafka import KafkaProducer

def main():
    producer = KafkaProducer(bootstrap_servers='localhost:29092')
    for _ in range(100):
        producer.send('key', b'test_key')
    future = producer.send('table', b'table_name')
    result = future.get(timeout=60)

if __name__ == "__main__":
    main()
