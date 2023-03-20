+++
title = "Encryption Algorithms"
date = 2023-01-28
weight = 20
template = "docs-page.html"
+++

## Relevant Requirements

**STCREAM-1056 / / R / F / DP**
Any data exchange shall be facilitated using strong and modern encryption mechanisms (e.g. AES, ECC or RSA)

**STCREAM-1714 / STCREAM-1056 / R / F / DP**
It shall be possible to upgrade the encryption mechanism in the future (e.g. when security breaches are discovered)

**STCREAM-1057 / / R / F / DP**
There shall be persistent proof of a data exchange (non-repudiation of exchanges), e.g. using digital signatures

**STCREAM-1665 / / R / F / MN**
If messages are exchanged as part of a conjunction, this shall take place in an encrypted manner

**STCREAM-1068 / STCREAM-1067 / R / F / UM**
User contact details (at least Email and Phone)

**STCREAM-1646 / - / R / F / SC**
The user master password shall not be stored on the application host, instead information (e.g. hashes) derived from it in a secure manner shall be used for server authentication

## Areas of Investigation

### The CREAM System

From the requirements we can deduce the need for the following cryptography in the CREAM system.

1. *STCREAM-1057* suggests that as data is uploaded the CREAM service verifies any signatures.
1. *STCREAM-1068 / STCREAM-1067* Suggests the use of symmetric encryption and decryption to protect PII.
1. *STCREAM-1665* Implies the CREAM system should have a way for users to discover the public asymmetric keys of other users. 
1. *STCREAM-1646* A password hashing function.
1. *STCREAM-1057* To sign messages in the browser.
1. Key wrapping. When we derive keys for users, we need to store them safely on the server.
1. PBKDF a function to derive keys from passwords.

### Available Algorithms

For the cryptography that we need we used the following source for recommendations.

- Libsodium https://github.com/jedisct1/libsodium A goto library for encryption algorithms. It is well tested and well supported.
- Fips recommendations https://en.wikipedia.org/wiki/Federal_Information_Processing_Standards
- https://developer.mozilla.org/en-US/docs/Web/API/Web_Crypto_API The algorithms available natively in browsers. 

Using those resources we can deduce the following algorithms.

1. AES-GCM for symmetric encryption and decryption.
1. ECDSA for signing. Elliptic curves were chosen due to the fact they have smaller key sizes and therefore require less storage space.
1. ECDH for public key encryption and key negotiation
1. PBKDF2 for key password based key derivation. Scrypt and Argon2id are not native to the browser, so PBKDF2 was chosen.
1. AES-KW for key wrapping. 
1. Argon2id (on the server) for hashing passwords before they are stored. *STCREAM-1646*

### Key agreement protocols

>  STCREAM-1665 / / R / F / MN If messages are exchanged as part of a conjunction, this shall take place in an encrypted manner

During a negotiation it is required that 2 parties can message each other without the CREAM system being able to decrypt the messages. This can be achieved using a key agreement protocol. Basically 2 public keys are used to derive a private key that both parties can then use to encrypt communications.

This can get more and more complex as more and more security is required. For example Signal the messenger uses something called Extended Triple Diffie-Hellman. https://signal.org/docs/specifications/x3dh/

It may be worth spending some time to look at different protocols. However for the purposes of a Proof of Concept a basic Diffie Hellman could be used. https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange

## What do we need to look at?

- A PoC for key exchange between two users of the CREAM system.
- Proof of concept for signing CCSDS data as it is uploaded to the CREAM system.

## Derived Technical Requirements

1. Messages between users in an exchange shall use a key agreement protocol. STCREAM-1665
1. It shall be possible for users to change their password which will re-wrap their private key.