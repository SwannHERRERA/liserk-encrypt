# Zero knowledge Database

## Client-Server Authentication:
1. The client and server establish a secure communication channel using a protocol like TLS.
2. The client authenticates itself to the server using an authentication mechanism such as client certificate.

## Key Generation and Exchange:
1. The client generates a symmetric encryption key (e.g., using AES-256) for encrypting and decrypting data.
2. The client encrypts the symmetric key with the server's public key using an asymmetric encryption algorithm **Kyber**.
3. The client securely sends the encrypted symmetric key to the server.
4. The server decrypts the symmetric key using its private key and stores it in a secure enclave.

## Data Encryption and Storage:
1. The client encrypts the data using the symmetric encryption key.
2. The client sends the encrypted data to the server.
3. The server stores the encrypted data in the database.

## Data Retrieval:
1. The client sends a query to the server, which may include encrypted search parameters.
2. The server processes the query and retrieves the encrypted data from the database.
3. The server sends the encrypted data to the client.
4. The client decrypts the data using the symmetric encryption key.

## Data Update:
1. The client encrypts the updated data using the symmetric encryption key.
2. The client sends the updated encrypted data and any necessary query parameters to the server.
3. The server updates the encrypted data in the database.

## Key Rotation:
1. Periodically, the client generates a new symmetric encryption key.
2. The client re-encrypts the data with the new key and sends the updated encrypted data to the server.
3. The client encrypts the new symmetric key with the server's public key and sends it to the server.
4. The server decrypts and stores the new symmetric key in the secure enclave.


