package main

import (
	"fmt"

	"github.com/IBM/sarama"
)

func main() {
	producer()

}

func producer() {

	config := sarama.NewConfig()
	config.Producer.Return.Successes = true

	producer, err := sarama.NewSyncProducer([]string{"localhost:9092"}, config)
	if err != nil {
		panic(err)
	}
	defer producer.Close()

	var novaMenssagem string
	fmt.Scanln(&novaMenssagem)
	message := &sarama.ProducerMessage{
		Topic: "test-topic",
		Value: sarama.StringEncoder(novaMenssagem),
	}

	partition, offset, err := producer.SendMessage(message)
	if err != nil {
		panic(err)
	}

	fmt.Printf("Menssagem enviada com sucesso! a particao %d com offset %d\n", partition, offset)

}
