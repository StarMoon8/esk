

NOT for production! 

Developing encryption-related applications in Rust is a feasible and attractive proposition due to Rust's strong emphasis on memory safety, concurrency, and security. The language's features make it a suitable choice for implementing cryptographic algorithms and building secure systems. There are numerous well-known encryption algorithms that could be implemented in Rust, ranging from traditional symmetric and asymmetric encryption algorithms to modern cryptographic primitives. Here, I will discuss several well-known encryption algorithms that could be used to create "decent encryption algo apps" in Rust, as well as the existing state of Rust libraries for cryptography.

Types of Encryption Algorithms and Apps That Could Be Made in Rust
There are multiple classes of encryption algorithms and applications that could be implemented in Rust. Let's categorize them:

1. Symmetric Encryption Algorithms
AES (Advanced Encryption Standard): AES is one of the most commonly used symmetric encryption standards worldwide. Libraries like aes or crypto2 provide an efficient implementation of AES.
ChaCha20: ChaCha20 is a stream cipher known for its performance, particularly in environments with limited hardware acceleration. ChaCha20 can be implemented using the chacha20 crate.
Blowfish and Twofish: These ciphers can also be implemented in Rust. There are implementations available, but they are not as popular as AES and ChaCha20.
Serpent: Another symmetric algorithm that is less widely used but known for being secure.
2. Asymmetric Encryption Algorithms
RSA (Rivest-Shamir-Adleman): RSA is one of the most well-known public-key encryption methods. Rust has support for RSA in libraries like rsa or ring.
ECC (Elliptic Curve Cryptography): Elliptic curves provide more efficient key sizes compared to RSA while maintaining the same level of security. Libraries like elliptic-curve and curve25519-dalek can be used to implement ECC in Rust.
3. Hybrid Encryption Systems
Hybrid systems use a combination of symmetric and asymmetric encryption. An app could use Rust to create a system where symmetric encryption (e.g., AES) is used for speed, and the symmetric key is then shared securely using RSA or ECC.

4. Hashing Algorithms and Applications
SHA-2 (Secure Hash Algorithm 2): Cryptographic hashing is a core component of secure systems, and the sha2 crate can be used to implement this.
SHA-3: Another popular hashing algorithm supported by the sha3 crate.
BLAKE2: Known for speed, BLAKE2 has implementations in Rust and can be used for hash-based message authentication code (HMAC).
Argon2: Password hashing algorithms like Argon2 can also be implemented with Rust (argon2 crate), useful for secure password storage.
5. Authenticated Encryption Algorithms
AES-GCM (Galois/Counter Mode): This mode provides both confidentiality and authentication, making it suitable for secure messaging applications. Rust libraries like aes-gcm can be used.
ChaCha20-Poly1305: This is a popular algorithm combination used in many modern encryption systems, such as TLS 1.3.
6. Digital Signature Algorithms
ECDSA (Elliptic Curve Digital Signature Algorithm): Suitable for signing messages and verifying integrity.
Ed25519: A highly efficient signature algorithm built on elliptic curves, popular for public-key infrastructure applications.
7. Public-Key Infrastructure (PKI)
Applications for creating, storing, and verifying digital certificates could also be made using Rust. Libraries like x509-parser can help with parsing certificates.
Well-Known Rust Cryptography Libraries
To build encryption-related apps in Rust, several cryptographic libraries are available:

RustCrypto Project: This is an umbrella organization for several important cryptographic libraries.

AES, ChaCha20, HMAC, PBKDF2, SHA2, etc. are all part of the RustCrypto collection.
RustCrypto libraries are modular, which means developers can use the specific algorithm they need without unnecessary dependencies.
Ring: The ring crate provides cryptographic primitives and is one of the most popular Rust cryptography libraries. It is suitable for building secure encryption apps that require cryptographic building blocks.

Sodiumoxide: Rust bindings for the libsodium C library, which implements cryptographic operations including symmetric encryption, public key encryption, and authenticated encryption. This library has a higher-level API, making it easier for newcomers to implement secure cryptographic systems.

Dalek Cryptography:

curve25519-dalek and ed25519-dalek are libraries that provide implementations for Curve25519 and Ed25519.
These are useful for building apps that involve secure key exchange or digital signing.
OpenSSL: The openssl crate allows Rust programs to interact with the well-known OpenSSL library, enabling encryption and secure communications.

Types of Encryption Applications That Could Be Built in Rust
Given the available encryption algorithms and libraries, here are some examples of decent encryption apps that can be built:

Secure File Storage Application: Use AES or ChaCha20 to encrypt files before saving them to disk. Rust's system-level control makes it ideal for building such an application with strong performance and security.

End-to-End Encrypted Messaging App: Combine AES or ChaCha20 with RSA or ECC for key exchange. Implement additional hashing mechanisms (e.g., HMAC-SHA256) for message authentication.

Password Manager: Store encrypted passwords using a combination of Argon2 for password hashing and AES for encryption.

VPN Software: Implement encryption for tunneling traffic using AES-GCM or ChaCha20-Poly1305, combined with a handshake mechanism that uses RSA or ECC.

Digital Signature Utility: Sign documents using ECDSA or Ed25519. This type of utility could be used for verifying software downloads or legal documents.

Encrypted Backup System: Encrypt backups using symmetric encryption before sending them to the cloud or other storage providers. Rust’s focus on performance could result in efficient implementations.

PKI Certificate Management Tool: Parse and verify X.509 certificates using the x509-parser crate.

How Many Encryption Apps Could Be Made?
To answer the question of "about how many well-known and decent encryption algo apps can be made in Rust?", consider the number of available algorithms and the possible combinations:

Symmetric Algorithms: At least 5-6 different apps could be built (AES, ChaCha20, Serpent, etc.).
Asymmetric Algorithms: A few more apps for public-key encryption using RSA or ECC.
Hybrid Systems: Many encryption apps can be designed with a hybrid of symmetric and asymmetric (AES+RSA, ChaCha20+ECC).
Secure Communications: Implementing TLS, VPN, or messaging solutions could easily yield 5-10 different applications.
Special Purpose: Password managers, digital signatures, file encryption, and PKI are different types of secure tools that could each be made into an application.
Overall, dozens of different well-known and secure encryption applications can be made in Rust by utilizing the existing cryptographic building blocks. Each of these apps can further be extended by choosing different combinations of encryption schemes and protocols, resulting in an even larger number of potential applications.

Rust's emphasis on performance and security makes it a perfect fit for building these types of applications, and its growing ecosystem of cryptographic libraries ensures that developers have the tools needed to succeed.



