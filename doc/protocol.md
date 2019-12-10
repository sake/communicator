# Introduction

This document specifies the communicator protocol. It is a very lightweight protocol on top of a streaming protocol such as TCP or TLS. The communicator nodes do not connect to each other, but use a broker to relay the messages. Topics as in MQTT are used to request distribution to a well defined group of nodes. The protocol adds integrity protection which ensures end to end protection of the transmitted data frames.

## Scope

notes: 
* ed25519 takes about 16ms
* chachapoly has about 2.2mb/s esp32 and 1,1mb/s esp8266 at 160mhz
* key rotation bei gruppenevent/ nachrichtenanzahl/zeit
* operations definieren

The following requirements are addressed by the protocol specified below.

1. Message is integrity proteced
2. Message is replay protected
3. Streaming of messages to devices with very small buffers
4. Message sending restricted to authorized nodes
5. Removal of nodes
6. Adding of nodes


# Key Management

The following keys und key usages are intended to exist in the system.

## Controller Master Key (CMK)

## Device Master Key (DMK)

The device master key is a symmetric key which is used to perform the handshake with the controller.

## Device Derived Key (DDK)

## Channel Key (CHK)




# Messages

## Register

Registration request of a node with the server/ system.

## Setup

Message sent from the server to a node in order to setup the node.

## Reset Node

Message sent by a node to request resetting of replay counter in all connected nodes.

## Publish

Send a message from one node to the server and distribute the message from the server to all nodes registered with the topic.


# Message and Frame Format

A message is split up into packets, so that a node may send and/or receive an integrity protected stream of message data. This is similar to what the TLS record layer provides, however this mechanism is much simpler and uses only symmetric crypto, so that it is feasable to run it on embedded devices and relay it easily.

## Message Structure

A single message is transmitted as a sequence of packets. The first packet contains metadata of the message. The following packets transport the message payload followed by a termination packet.

## General Frame Layout

| Bytes           | Description     |
| --------------- | --------------- |
| 1               | Frame Type      |
| 2               | Frame Number    |
| 2               | Payload length  |
| sizeof(payload) | Payload         |
| sizeof(mac)     | HMAC over Frame |

## Message Header

The message header is the first message in a message stream. It therefore always has the frame number 0. Such a frame if accepted stops a currently received stream. An always increasing message number is included to prevent replay attacks of messages.

Frame Type is 0.

| Bytes         | Description    |
| ------------- | -------------- |
| 1             | Message Type   |
| 1-2           | HMAC Alg       |
| 4             | Message Number |
| 2             | Topic Size     |
| sizeof(topic) | Topic (string) |

## Message Payload

This message is a sequence of octets (the payload) and has no further structure.

Frame Type is 1.

## Message Termination

This message does not contain any data.

Frame Type is 2.


### HMAC Algorithms

Derived from PKCS11, the following HMAC algorithms are defined.

| Algorithm           | Identifier |
| ------------------- | ---------- |
| CKM_SHA_1_HMAC      | 0x0221     |
| CKM_SHA256_HMAC     | 0x0251	   |
| CKM_SHA224_HMAC     | 0x0256	   |
| CKM_SHA384_HMAC     | 0x0261	   |
| CKM_SHA512_HMAC     | 0x0271	   |
| CKM_SHA512_224_HMAC | 0x0049	   |
| CKM_SHA512_256_HMAC | 0x004D     |
