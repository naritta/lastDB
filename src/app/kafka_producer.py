from kafka import KafkaProducer

def main():
    producer = KafkaProducer(bootstrap_servers='localhost:29092')
    future = producer.send('key', value=b'table_name', partition=0)
    # future = producer.send('key', value=b'0', partition=1)
    result = future.get(timeout=60)

if __name__ == "__main__":
    main()
