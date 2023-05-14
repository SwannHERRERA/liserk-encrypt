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

---

# Protocole de communication

Premier octet Type de message, 4 bits suivants, taille.

- Message d'initialisation :
- Version de la communication
- Version de l'athentification
- authetification
- ErrorResponse
- CommandComplete
- ReadyForQuery
- EmptyQueryResponse
- QueryResponse

J'ai des problèmatiques de securité, je serais pas capable de faire validé mon certificat par une authorité.
Je veux faire du post quantic direct dans le Protocol, et gerer moi même la couche de sécurité. 
Je me base quand même sur TLS 1.3 je prends les idées mais au lieu de faire du RSA j'utilise Kyber.
Pour mes certificats je vais faire un KMS dans un second temps, ce KMS utilisera ZKP pour valider les clefs.
Donc dans l'idée je vais generer des clef kyber assez rapidement.


## Process de sécurité reseau

- le navigateur du client envoie au serveur une demande de mise en place de connexion sécurisée par TLS.
- Le serveur envoie au client son certificat : celui-ci contient sa clé publique, ses informations (nom de la société, adresse postale du server, pays, e-mail de contact...) ainsi qu'une signature numérique.
- Le navigateur du client tente de vérifier la signature numérique grace a son cache.
- Si l'une d'entre elles fonctionne, le navigateur web en déduit le nom de l'autorité de certification qui a signé le certificat envoyé par le serveur. Il vérifie que celui-ci n'est pas expiré puis envoie une demande OCSP à cette autorité pour vérifier que le certificat du serveur n'a pas été révoqué.
- Si aucune d'entre elles ne fonctionne, le navigateur web tente de vérifier la signature numérique du certificat du serveur à l'aide de la clé publique contenue dans celui-ci.
- En cas de réussite, cela signifie que c'est frauduleux.
- En cas d'échec, le certificat est invalide, la connexion ne peut pas aboutir.
- Le client génère une clé de chiffrement symétrique, appelée clé de session, qu'il chiffre à l'aide de la clé publique contenue dans le certificat du serveur puis transmet cette clé de session au serveur.
- Le serveur déchiffre la clé de session envoyée par le client grâce à sa clé privée.
- Le client et le serveur commencent à s'échanger des données en chiffrant celles-ci avec la clé de session qu'ils ont en commun. On considère à partir de ce moment que la connexion TLS est alors établie entre le client et le serveur.
- Une fois la connexion terminée (déconnexion volontaire de l'utilisateur ou si durée d’inactivité trop élevée), le serveur révoque la clé de session.

Il faut que je vois pour la rotation des clefs.

Il faut que je fasse du WAL et de l'event sourcing,
Il faut que j'ai un streaming Replication protocol - pour les cas ou on veut faire un replay de ce qu'il s'est passé sur le reseau on veut pas que ça marche.
Pour tout ça j'utilise du CBor

---

# CI

## Fix a lot of errors, by adding Protoc (doc,clippy-stable,clippy-beta)

- name: Install Protoc
        uses: arduino/setup-protoc@v1
        with:
          version: '3.x'
          repo-token: ${{ secrets.GITHUB_TOKEN }}
