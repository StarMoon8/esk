AES either was or will be cracked (in 1 minute)- that is for sure. To make aes post-quantum safe- i say why take chances? Do a round of AES with a somewhat proven aes tool like https://github.com/str4d/rage . Then use the otp (one time pad) method over the encrypted file. As such, your file should be quite quantum safe, and hella tough for quantum Ai to decrypt. (unless you are on windows or are connected to the internet-, in which case any key you use is known, any passwords you use are also known... and trying to encrypt anything is just silly.  lol)

The OTP method is simple and the best (uncrackable in theory) method known, but it is extremely tough to practice in real life. Any slip up like using the key twice diminishes the superior quality of the true otp method. 

Encryption itself is a simple file operation that 5th grade school children can understand. The tough part is encrypting things that can not be decrypted by anyone else. That is where the PhD level thinking is required. 

This repo is just screwing around with xor encryption and deteministic key making. A deterministic key that is encrypted using another deterministic key would take quantum ai robots about 5 more seconds to decrypt, but its not a bad idea. It is a mix of being user friendly, not having to store keys, and data integrity. 

All Encryption is rather crude. Future versions might make it impossible except for a single user to decrypt just by looking at it- but that is far away. For now, encryption is in the stone ages. 
