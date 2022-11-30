from kafka import KafkaConsumer
from kafka import TopicPartition

def main():
    consumer = KafkaConsumer(bootstrap_servers='localhost:29092')
    consumer.assign([TopicPartition('key', 0)])
    for msg in consumer:
      print (msg)

if __name__ == "__main__":
    main()
