# Bitcoin Wallet (BIP-39 & BIP-32) in Rust

A lightweight, terminal-based deterministic Bitcoin wallet built from scratch in Rust. It implements core BIP-39 and BIP-32 cryptographic standards manually using basic math and low-level primitives.


## How to run

1. Make sure you have Rust and Cargo installed.
2. Clone the repository and navigate to the project directory:
```bash
   git clone https://github.com/massieuxalexandre/Bitcoin-wallet.git
   cd Bitcoin-wallet
   ```
3. Run the program :
```bash
   cargo run
   ```
4. Follow the interactive menu in the console:
   - 1: Generate a new random seed.
   - 2: Import an existing BIP-39 seed phrase.

5. Then follow the main menu:
    - 1: Show all representations of your seed
    - 2: Show master private key
    - 3: Show master public key
    - 4: Show master chain code
    - 5: Generate a child key at index N
    - 6: Generate a child key at index N at derivation level M
    - 7: Quit


## Technical report

### 1. Project structure
- main.rs: Handles the main CLI loop and menu options.
- seed.rs (BIP-39): Handles random entropy generation, checksum calculation, binary-to-word mapping using english.txt, and generates the 64-byte root seed.
- wallet.rs (BIP-32): Manages wallet initialization, master key extraction, and mathematical child key derivations.
- utils.rs: Helper functions for clean user input and menu display.

### 2. Cryptographic implementation 
To comply with academic requirements, high-level Bitcoin libraries were avoided. We rely solely on fundamental mathematical and cryptographic tools:
- rand & sha2: For entropy generation and checksums.
- hmac & sha512: Used for the core HMAC-SHA512 loops required by BIP-32.
- num-bigint: Used to handle large 256-bit arithmetic for private key additions.
- k256: Authorized exception used strictly to derive compressed public keys (33 bytes) from private keys on the secp256k1 curve.

### 3. Key derivation steps
- Master Keys: The 64-byte root seed is hashed via HMAC-SHA512 using the key "Bitcoin seed". The left 256 bits form the Master Private Key, and the right 256 bits form the Master Chain Code.
- Child Keys (generate_child_key): Concatenates the parent's public key with a 4-byte big-endian index N, runs HMAC-SHA512 keyed with the parent's chain code, extracts the new chain code from the right half, and computes the new private key via modular addition:
  Child Private Key = (Left 256-bit Integer + Parent Private Key) mod n
  (where n is the secp256k1 curve order).