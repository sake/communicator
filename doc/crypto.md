# Design Concept

The communicator system consists of the following three types of participants.
- Client
- Keyserver
- Relay

Each of these entities has different properties with respect to the crypto system. 

The client can store small amounts of data, but has very limited input and output capabilities. 
Therefore it can not be used to manage whith whom he wants to exchange messages. 
He can only select a well known counterpart or a group of counterparts. 

The keyserver takes care on behalf of the client to define the counterparts of its clients.
These definitions are backed up by cryptographic keys enabling the clients to validate messages from their counterparts for themselves.
This gives the keyserver a great deal of power, so the keyserver should be operated by someone the participants trust.

The relay on the other side takes the role of message distributor.
It may only relay messages to the other adressed clients.
Thereby it may not replay old messages.
The worst that could happen is that the relay drops messages.

The crypto concept of the communicator is designed after the following rules/ requirements.

- There is no PKI to do key distribution
- Client and key-server can recognize each other during initialisation
- Clients can not recognize each other during initialization
- Messages exchanged between client and server are integrity protected
- Client groups are managed by the keyserver (add and remove clients)
- Groups of clients are closed, meaning unauthorized clients/attackers must not be able to send messages
- The payload of client to client messages can be encrypted, metadata is not so the relay-server can relay properly
- The relay server must be able to decide, based on the group, whether a message shall be relayed or not
- Old messages must not be accepted by clients
- Old messages must not be accepted by the relay server

Taking these rules into consideration, it is obvious that the server takes a special role in the system as it can manage groups. This means that the server can in general add itself to a group and become an evesdropper.
This makes it weaker than sophisticated secure group messaging protocols like Signal.
However there the client takes a different role as the user of the client can make desicions which is not the case here.
The effect is that the server should be operated by a party also in charge of the groups.


# Key Inventory

In the following the keys of the system and their purpose is explained.

The keyserver has the following keys:
- **Sk_Ks**, **Pk_Ks** Asymmetric keypair  

The client has the following keys:
- **Sk_Cn** asymmetric private key which is persisted on the client during registration
- **Pk_S** asymmetric public key of the server which is persisted on the client during registration

The server has the following keys:
- **Sk_S** asymetric private key which is persisted in the server
- **Pk_Cn** asymmetric public key of each client received during registration

Both parties (server and client) have the following shared secrets
- **Sk_ch** symmetric channel key established during client initialisation
- **Sk_gr** symmetric group key received during initialisation or rekeying and chosen by the server


# Registration

Clients are registered out of band. The registration is therefore not covered by this crypto concept.


# Client-Server Channel

During the initial handshake, a channel between the client and server is established.
The channel is mutually authenticated by the client and server with the asymmetric keys exchanged during the registration.
In case of the key-server, the client public key is not known beforehand as it serves only as a unique client address as shown later.
The authentication of the client is used in the server to associate the client with a set of groups.

The channel establishment is described by the following algorithm based on ECDH:
1. Each partner choses a fresh EC key pair named A (client) and B (server) in the following
2. The client sends message `M1=Pk_Cn` containing `Q_A`
3. The server calculates `(x_k,y)=d_B*Q_A`
4. The server sends messages `M2=Pk_S` containing `Q_B` and `M3=Sig(Sk_S, H(M1|M2))`
5. The client calculates `(x_k,y)=d_A*Q_B`
6. The client verifies the signature in M3 with Pk_S
7. The client sends message `M4=Sig(Sk_Cn, H(M1|M2|M3))`
8. In case the client sent message M4, the server verifies the signature in M4 with Pk_Cn
9. In case the verification is ok, both parties calculate `Sk_ch=H(x_k)` in order to obtain a different key length for the symmetric operations

After that, both parties know the secret key Sk_ch and know the peer with which they are talking with.
They both start with distinct counter values for sending and receiving set to 0.
These counters are incremented per message to make sure there are no replays.

The shared secret Sk_ch is used in this channel to secure the messages with an HMAC over the complete message.

The connection to the peer must be aborted in case an unexpected value is received.
When terminating a connection, all temporary secrets are forgotton.


# Group Communication

Messages exchanged between 
