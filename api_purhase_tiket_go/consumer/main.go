package main

import (
	"fmt"
	"log"

	"github.com/IBM/sarama"
)

func main() {
	consumer()
}

func consumer() {
	config := sarama.NewConfig()
	config.Consumer.Return.Errors = true

	consumer, err := sarama.NewConsumer([]string{"localhost:9092"}, config)
	if err != nil {
		log.Fatalf("Error creating consumer %v", err)
	}
	defer consumer.Close()

	partitionConsumer, err := consumer.ConsumePartition("test-topic", 0, sarama.OffsetNewest)
	if err != nil {
		log.Fatalf("Error consuming partition %v", err)
	}
	defer partitionConsumer.Close()

	for message := range partitionConsumer.Messages() {
		fmt.Printf("Message recebida: %s\n", string(message.Value))
	}

}
