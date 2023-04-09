# Remote Server Monitoring Application
This repository contains the code for a remote server monitoring application that allows users to monitor and manage their remote servers from their mobile devices.

## Getting Started
To use this code, you will need to have the Rust programming language installed on your system. You will also need to have a Kafka broker set up to receive and send messages to the mobile application.

To run the code, simply clone the repository and run the following command in your terminal:
```cargo run```

This will start the consumer process that listens for messages from the Kafka broker and executes the commands received.

## Usage
The code provided in this repository listens for messages from a Kafka topic and executes the commands received. You will need to configure the Kafka broker to send messages to the specified topic in order for the application to function.

To customize the application for your needs, you can modify the code to perform different actions based on the commands received. You can also modify the Kafka configuration to connect to a different broker or use a different topic.

## Dependencies

This code uses the rdkafka library for interacting with the Kafka broker. You will need to have this library installed on your system in order to compile and run the code.

To install the rdkafka library, you can use the following command: ```cargo install rdkafka```
