# Liserk Zero Knowledge Database

## Introduction
This project is a zero-knowledge database system designed for secure communication and data storage. It comprises various modules that work in unison to establish secure communication between a client and a server via TCP, with a primary focus on protecting the data being transmitted and stored.

## Modules

### Certificate Authority

The Certificate Authority module is responsible for handling the certificates used to ensure the integrity and authenticity of communications between the client and the server. This module generates, validates, and manages the certificates and keys used in secure exchanges.

### Server

The Server module is responsible for listening for incoming connections from clients. It handles client requests and responds accordingly. The server ensures that only authorized clients can establish a connection and exchange data, upholding security and data integrity.

### Client

The Client module is used to connect to the server. It is capable of sending requests to the server and receiving responses. The client employs encryption to ensure that the data transmitted is secure and cannot be read by unauthorized third parties.

### Order-Preserving Encryption (OPE)

This module is responsible for encrypting the data using Order-Preserving Encryption (OPE). OPE is a type of encryption that allows for the comparison of encrypted data without decrypting it. This module is vital for ensuring the confidentiality of the data while still allowing certain operations like comparison.

### Shared

The Shared module is used to share common data structures and utilities among different modules. This prevents code duplication and ensures consistency across modules.

### Tests

The Tests module contains integration tests that ensure the various components of the system work together correctly. These tests are vital for ensuring the system is reliable and secure.

## Communication Protocol

The communication protocol used in this project is simple yet effective, built on top of TCP. It doesn't use an additional layer as the data transiting are already encrypted, ensuring their security.

The protocol employs CBOR (Concise Binary Object Representation) for data serialization, which allows efficient encoding of data. The protocol utilizes an enum Message along with serde for encoding and understanding what is being transmitted.

The first byte contains the type of message, the following four bytes signify the size of the message, and then the actual message follows.

The system use tokio for handiling multiple connection at the same time

## Zero-Knowledge Database

One of the distinguishing features of this project is the implementation of a zero-knowledge database. This means that the server stores the data in such a way that it doesn't know the contents of the data it is storing. This is achieved through encryption and specific protocols that enable the client to interact with their data without exposing it to the server.

This approach is particularly useful for preserving user privacy and ensuring data security, especially in scenarios where the data is sensitive and should not be exposed to even the service provider.
