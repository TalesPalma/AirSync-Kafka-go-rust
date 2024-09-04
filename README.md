# AirSync-Kafka-go-rust

## Visão Geral do Projeto

A ideia do projeto é criar dois serviços que se comunicam usando o Kafka.

1. **Serviço de Compras de Passagem de Avião:** Este será uma API desenvolvida em Golang.
2. **Serviço de Reserva de Assentos:** Este serviço gerencia os assentos livres no avião após a compra ser confirmada. Será uma API desenvolvida em Rust.

## Compra de Passagem

- A API de compras em Go publica um evento no Kafka após a confirmação de uma compra.

## Reserva de Assentos

- O serviço em Rust consome esse evento, acessa o banco de dados (ou outro armazenamento) para verificar a disponibilidade e, se possível, reserva o assento, atualizando o estado no armazenamento.

## Armazenamento e Consulta

- As informações sobre aviões e assentos ficam armazenadas no banco de dados.
- O serviço em Rust pode expor uma API para consultas de disponibilidade ou cancelar reservas, se necessário.

## Alternativas ou Complementos

1. **API Gateway:** 
   - Se você precisar de um ponto centralizado para gerenciar as APIs e o roteamento de requisições, considere implementar um API Gateway. Isso pode ajudar a simplificar a exposição das APIs para clientes externos.

2. **Outbox Pattern:**
   - Se precisar garantir que as mensagens Kafka sejam publicadas apenas se as transações forem bem-sucedidas, o padrão de outbox pode ser útil. Nesse padrão, você primeiro escreve a mensagem em uma tabela de outbox antes de publicá-la.

3. **gRPC:**
   - Se precisar de comunicação mais direta entre serviços, pode considerar gRPC para chamadas síncronas entre Go e Rust, embora Kafka continue sendo excelente para comunicação assíncrona e processamento de eventos.

