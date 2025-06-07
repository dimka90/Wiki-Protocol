# Raw Ethereum Transaction Signer

A Node.js utility to manually sign raw Ethereum transactions using `ethereumjs-util`.

## Overview

This project demonstrates how to:

- Encode a transaction payload with RLP (Recursive Length Prefix)
- Hash the encoded payload using Keccak256
- Sign the hash with a private key using ECDSA (Elliptic Curve Digital Signature Algorithm)
- Generate a signed raw transaction ready for broadcast to the Ethereum network

## Features

- Accepts transaction data and private key as inputs
- Validates private key length
- Uses low-level Ethereum primitives to build and sign transactions
- Outputs the signed transaction as an RLP-encoded hex string

## Usage

1. Install dependencies:
   ```bash
   npm install ethereumjs-util
   ```
   
2. Run the script with transaction payload and private key as arguments:
```bash
node sign.js '[payload_as_JSON_array]' 'your_private_key_in_hex'
```

Example:
```bash
node sign.js '[ "0x", "0x77359400", "0x13880", "0x", "0x05", "0x6008600c60003960086000f36006600702600055" ]' 0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80
```
3. The output will include the signature parts (v, r, s) and the fully signed raw transaction hex.

- How it Works
- Parses the transaction payload from command line
- Converts the private key to a Buffer for signing
- Encodes the transaction with RLP
- Hashes the encoded transaction using Keccak256
- Signs the hash with the private key using ECDSA
- Appends the signature to the transaction payload
- Prints the fully signed transaction

## Prerequisites
- Node.js installed
- Basic understanding of Ethereum transactions and cryptography

